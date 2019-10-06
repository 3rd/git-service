use hyper::{Body, Method, Request, Response};
use regex::Regex;
use std::sync::Arc;
use std::sync::Mutex;

use super::controller;
use super::repository::GitRepository;

pub fn handle_request(req: Request<Body>, repo: &Arc<Mutex<GitRepository>>) -> Response<Body> {
    // /branch/:name/:operation
    let re = Regex::new(r"/branch/([^\\/]*)/([^\\/]*)").unwrap();
    if let Some(m) = re.captures(req.uri().path()) {
        let branch = m.get(1).unwrap().as_str();
        if !branch.is_empty() {
            let operation = m.get(2).unwrap().as_str();
            match operation {
                "checkout" => return controller::checkout(&repo.lock().unwrap(), &branch),
                "fetch" => return controller::fetch(&repo.lock().unwrap(), &branch),
                "pull" => return controller::pull(&repo.lock().unwrap(), &branch),
                _ => (),
            }
        }
    }

    // /branch/:name - last commit on branch
    let re = Regex::new(r"/branch/([^\\/]*)").unwrap();
    if let Some(m) = re.captures(req.uri().path()) {
        let branch = m.get(1).unwrap().as_str();
        return controller::last_commit(&repo.lock().unwrap(), &branch);
    }

    // static
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => controller::index(),
        (&Method::GET, "/remotes") => controller::remotes(&repo.lock().unwrap()),
        (&Method::GET, "/branches/local") => controller::local_branches(&repo.lock().unwrap()),
        (&Method::GET, "/branches/remote") => controller::remote_branches(&repo.lock().unwrap()),
        _ => controller::not_found(),
    }
}
