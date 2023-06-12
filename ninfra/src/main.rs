use warp::Filter;
use nix::eval::{NixFile, NixFileOptions};
use nix::store::NixStore;
use shiplift::Docker;
use std::convert::Infallible
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
  let docker_client = Docker::new();

  let nix_route = warp::path!("provision") 
    .and(warp::get())
    .and(warp::query::<>())
}

