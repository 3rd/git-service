use git2::{Repository, ResetType};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server, StatusCode};
use log::{debug, error, info};
use std::env;
use std::net::SocketAddr;

fn open_repo(path: &str) -> Result<Repository, git2::Error> {
    Repository::open(&path)
}

fn git_clone(remote: &str, path: &str) -> Result<Repository, git2::Error> {
    Repository::clone(&remote, path)
}

fn git_checkout(repo: &Repository, branch: &str) -> Result<(), git2::Error> {
    let obj = repo.revparse_single(&format!("refs/heads/{}", branch))?;
    repo.checkout_tree(&obj, None)
}

fn git_pull(repo: &Repository, branch: &str) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[&branch], None, None)?;
    let obj_id = repo.refname_to_id(&format!("refs/remotes/origin/{}", branch))?;
    let obj = repo.find_object(obj_id, None)?;
    repo.reset(&obj, ResetType::Hard, None)
}

fn update(remote: &str, branch: &str, clone_path: &str) {
    let mut repo = open_repo(&clone_path);

    // open or clone repo
    if repo.is_err() {
        repo = git_clone(&remote, &clone_path);
        if repo.is_ok() {
            debug!("Cloned repo at {}", clone_path);
        } else {
            error!(
                "Could neither open or clone the repo: {}",
                repo.err().unwrap()
            );
            return;
        }
    } else {
        debug!("Opened repo from {}", clone_path);
    }
    let repo = repo.unwrap();

    // pull
    let pull_result = git_pull(&repo, &branch);
    match pull_result {
        Err(e) => error!("Failed to pull: {}", e),
        _ => (),
    };
    debug!("Pulled changes from remote");

    // checkout
    // TODO: check if already on the target branch and skip
    match git_checkout(&repo, &branch) {
        Ok(()) => debug!("Checked out to branch: {}", &branch),
        Err(e) => error!("Failed to checkout: {}", e),
    };
}

fn main() {
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

    update(&remote, &branch, &clone_path);

    let service = move || {
        service_fn_ok(move |req: Request<Body>| {
            if req.uri().path() == "/update" {
                update(&remote, &branch, &clone_path);
                return Response::new(Body::from("success"));
            }
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        })
    };

    let addr: SocketAddr = format!("127.0.0.1:{}", port)
        .parse()
        .expect("Could not parse port, invalid input");
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| error!("Server error: {}", e));
    hyper::rt::run(server);
}
