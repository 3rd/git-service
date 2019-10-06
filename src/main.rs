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

    let remote = env!("REMOTE", "$REMOTE not found");
    let branch = env!("BRANCH", "$BRANCH not found");
    let clone_path = env!("CLONE_PATH", "$CLONE_PATH not found");
    let port = env!("PORT", "$PORT not found");

    info!("Starting repository sync service");
    info!("  remote = {}", remote);
    info!("  branch = {}", branch);
    info!("  clone_path = {}", clone_path);
    info!("  port = {}", port);

    let mut repo = GitRepository::open(&clone_path);
    if repo.is_ok() {
        info!("Opened repository");
    } else {
        repo = GitRepository::clone(&remote, &clone_path);
        if repo.is_ok() {
            info!("Cloned repository");
        }
    }
    let repo = Arc::new(Mutex::new(repo?));

    let service = move || {
        let repo = repo.clone();
        service_fn_ok(move |req: Request<Body>| router::handle_request(req, &repo))
    };
    let addr: SocketAddr = format!("127.0.0.1:{}", port)
        .parse()
        .expect("Could not parse port, invalid input");
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| error!("Server error: {}", e));
    hyper::rt::run(server);
    Ok(())
}
