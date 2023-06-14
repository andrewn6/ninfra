use warp::Filter;
use tokio::runtime::Runtime;
use std::process::Command;
use sqlite::{Connection, Result};

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

async fn provision_handler(service: String) -> Result<impl warp::Reply, warp::Rejection> { 
  tokio::task::spawn_blocking(move || {
    provision_service(&service);
  })
  .await
  .map_err(|_| warp::reject::reject()?);

  Ok(warp::reply::json(&format!("Provision service: {}", service)))
}

fn main() -> Result<()> {
  let conn = Connection::open("../../db/ninfra.db")?;

  conn.execute(
    "CREATE TABLE IF NOT EXISTS provisions (
      id INTEGER PRIMARY KEY AUTOINCREMENT
      service TEXT NOT NULL,
      timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"
  )?;

  let db_conn = warp::any().map(move || conn.clone());

  let provision = warp::path!("provision" / String)
    .and(db_conn.clone())
    .and_then(provision_handler);

  warp::serve(provision).run(([127, 0, 0 ,1], 3030));

  Ok(())
}
