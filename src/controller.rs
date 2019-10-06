use hyper::{Body, Response, StatusCode};
use log::debug;
use serde::Serialize;
use serde_json::json;

use super::repository::GitRepository;

fn send_empty() -> Response<Body> {
    Response::new(Body::empty())
}

fn send_error(error: String) -> Response<Body> {
    let mut response = Response::new(Body::from(json!({ "error": error }).to_string()));
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    response
}

fn send_json_response<T: ?Sized>(value: &T) -> Response<Body>
where
    T: Serialize,
{
    match serde_json::to_string(&value) {
        Ok(serialized) => Response::new(Body::from(serialized)),
        Err(e) => send_error(format!("{}", e)),
    }
}

pub fn index() -> Response<Body> {
    debug!("index");
    Response::new(Body::from("alive"))
}

pub fn not_found() -> Response<Body> {
    debug!("not_found");
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::NOT_FOUND;
    response
}

pub fn remotes(repo: &GitRepository) -> Response<Body> {
    debug!("remotes");
    match repo.get_remotes() {
        Ok(branches) => send_json_response(&branches),
        Err(e) => send_error(format!("{}", e)),
    }
}

pub fn local_branches(repo: &GitRepository) -> Response<Body> {
    debug!("local_branches");
    match repo.get_local_branches() {
        Ok(branches) => send_json_response(&branches),
        Err(e) => send_error(format!("{}", e)),
    }
}

pub fn remote_branches(repo: &GitRepository) -> Response<Body> {
    debug!("remote_branches");
    match repo.get_remote_branches() {
        Ok(branches) => send_json_response(&branches),
        Err(e) => send_error(format!("{}", e)),
    }
}

pub fn last_commit(repo: &GitRepository, branch: &str) -> Response<Body> {
    debug!("last_commit");
    match repo.get_last_commit(&branch) {
        Ok(commit) => send_json_response(&commit),
        Err(e) => send_error(format!("{}", e)),
    }
}

pub fn checkout(repo: &GitRepository, branch: &str) -> Response<Body> {
    debug!("checkout {}", branch);
    match repo.checkout(&branch) {
        Ok(()) => send_empty(),
        Err(e) => send_error(format!("{}", e)),
    }
}

pub fn fetch(repo: &GitRepository, branch: &str) -> Response<Body> {
    debug!("fetch {}", branch);
    match repo.fetch("origin", &branch) {
        Ok(()) => send_empty(),
        Err(e) => send_error(format!("{}", e)),
    }
}

pub fn pull(repo: &GitRepository, branch: &str) -> Response<Body> {
    debug!("pull {}", branch);
    match repo.pull("origin", &branch) {
        Ok(()) => send_empty(),
        Err(e) => send_error(format!("{}", e)),
    }
}
