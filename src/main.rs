use failure::Error;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Server};
use log::{error, info};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

mod repository;
use repository::GitRepository;
mod controller;
mod router;

fn main() -> Result<(), Error> {
    pretty_env_logger::init_custom_env("DEBUG");

    // read env vars
    let url = env!("GIT_SERVICE_REPO_URL", "$GIT_SERVICE_REPO_URL not found");
    let clone_path = env!(
        "GIT_SERVICE_CLONE_PATH",
        "$GIT_SERVICE_CLONE_PATH not found"
    );
    let port = env!("GIT_SERVICE_PORT", "$GIT_SERVICE_PORT not found");

    info!("Starting repository sync service");
    info!("  url = {}", url);
    info!("  clone_path = {}", clone_path);
    info!("  port = {}", port);

    // open or clone repo
    let mut repo = GitRepository::open(&clone_path);
    if repo.is_ok() {
        info!("Opened repository");
    } else {
        repo = GitRepository::clone(&url, &clone_path);
        if repo.is_ok() {
            info!("Cloned repository");
        }
    }
    let repo = Arc::new(Mutex::new(repo?));

    // create hyper service
    let service = move || {
        let repo = repo.clone();
        service_fn_ok(move |req: Request<Body>| router::handle_request(req, &repo))
    };

    // start server
    let addr: SocketAddr = format!("127.0.0.1:{}", port)
        .parse()
        .expect("Could not parse port, invalid input");
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| error!("Server error: {}", e));
    hyper::rt::run(server);
    Ok(())
}
