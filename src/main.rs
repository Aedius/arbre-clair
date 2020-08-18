use std::fs::File;
use std::path::Path;

use tiny_http::{Method, Request, Response, Server, StatusCode};

const FOLDER_PREFIX : &str = "static";

fn main() {
    let server = Server::http("0.0.0.0:8081").unwrap();

    println!("listening on 8081");

    for request in server.incoming_requests() {
        println!("received request!\n, method: {:?}\n, url: {:?}\n, headers: {:?}\n",
                 request.method(),
                 request.url(),
                 request.headers(),
        );
        if request.method() == &Method::Get {
            let path = get_path(FOLDER_PREFIX, request.url().to_string());

            match path {
                ResourceKind::Static(path) => {
                    respond_file(path, request)
                }
                ResourceKind::Api(_) => {}
            };
        } else {
            match request.respond(Response::new_empty(StatusCode(405))) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
    }
}


enum ResourceKind {
    Static(String),
    Api(String),
}


fn get_path(prefix: &str, path: String) -> ResourceKind {
    if path.len() > 5 {
        if &path[0..5] == "/api/" {
            return ResourceKind::Api(path);
        }
    }

    if path == "/".to_string() {
        return ResourceKind::Static(format!("{}/index.html", prefix));
    }

    ResourceKind::Static(format!("{}{}", prefix, path))
}

fn respond_file(path: String, request: Request) {
    let file = match File::open(&Path::new(path.as_str())) {
        Ok(file) => {
            file
        }
        Err(_) => {
            return match request.respond(Response::new_empty(StatusCode(404))) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            };
        }
    };

    let response = tiny_http::Response::from_file(file);
    match request.respond(response) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e)
        }
    }
}
