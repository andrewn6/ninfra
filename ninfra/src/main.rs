use warp::Filter;
use std::process::Command;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::result::Result as StdResult;

type Cache = Arc<Mutex<HashMap<String, String>>>;


fn provision_service(service: &str) {
  let output = Command::new("nix")
    .arg("build")
    .arg("-f")
    .arg("../../../infra.nix")
    .arg(service)
    .output()
    .expect("Failed to provision service");
  
  if output.status.success() {
    println!("Successfully provisioned service: {}", service);
  } else {
    println!("Failed to provision service: {}", service);
  }
}

async fn provision_handler(service: String, _conn: SqlitePool, cache: Cache) -> StdResult<impl warp::Reply, warp::Rejection> {
  {
      let cache_guard = cache.lock().unwrap();
      if let Some(result) = cache_guard.get(&service) {
          return Ok(warp::reply::json(&format!("Service already provisioned: {}", service)));
      }
  } // This scope ensures that the lock is dropped before we move on

  let service_clone = service.clone();
  let cache_clone = Arc::clone(&cache);

  tokio::task::spawn_blocking(move || {
      let mut cache_guard = cache_clone.lock().unwrap();
      provision_service(&service_clone);
      cache_guard.insert(service_clone.clone(), "Provisioned".to_string());
  })
  .await
  .map_err(|_| warp::reject::reject())?;

  Ok(warp::reply::json(&format!("Provision service: {}", service)))
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
  let pool = SqlitePool::connect_with(
    SqliteConnectOptions::new()
      .filename("../../db/ninfra.db")
      .create_if_missing(true)
  ).await?;

  sqlx::query(
    "CREATE TABLE IF NOT EXISTS provisions (
      id INTEGER PRIMARY KEY,
      service TEXT NOT NULL,
      provisioned_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )"
  )
  .execute(&pool)
  .await?;

  let db_conn = warp::any().map(move || pool.clone());

  let cache: Cache = Arc::new(HashMap::new().into());

  let cache_filter = warp::any().map(move || cache.clone());

  let provision = warp::path!("provision" / String)
    .and(db_conn.clone())
    .and(cache_filter.clone())
    .and_then(provision_handler);

  warp::serve(provision).run(([127, 0, 0 ,1], 3030));

  Ok(())
}
