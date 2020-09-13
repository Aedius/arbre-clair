#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::path::Path;
use serde_json::json;
use tiny_http::{Method, Request, Response, Server, StatusCode};

mod craft;

use crate::craft::recipe::{handle_recipe, handle_craft};

const FOLDER_PREFIX: &str = "static";

lazy_static! {
    pub static ref URL_CRAFT_RE: Regex = Regex::new(r"/api/recipe-list/([^/]+)$").unwrap();
    pub static ref URL_CRAFT_RECIPE_RE: Regex = Regex::new(r"/api/recipe/detail/([^/]+)/([0-9]+)$").unwrap();
}

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
                ResourceKind::Api(path) => {
                    respond_api(path, request)
                }
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
        return ResourceKind::Static(format!("{}/pages/index.html", prefix));
    }

    ResourceKind::Static(format!("{}{}", prefix, path))
}

fn respond_api(path: String, request: Request) {
    if URL_CRAFT_RE.is_match(path.as_str()) {
        let caps = URL_CRAFT_RE.captures(path.as_str()).unwrap();
        let as_text = caps.get(1).map_or("", |m| m.as_str());


        let recipes = handle_craft(as_text);

        return match recipes {
            None => {
                return match request.respond(Response::new_empty(StatusCode(404))) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e)
                    }
                };
            }
            Some(recipes) => {
                let response = tiny_http::Response::from_string(format!("{}", json!(recipes)));
                match request.respond(response) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
        };
    }

    if URL_CRAFT_RECIPE_RE.is_match(path.as_str()) {
        let caps = URL_CRAFT_RECIPE_RE.captures(path.as_str()).unwrap();

        let as_text = caps.get(1).map_or("", |m| m.as_str());
        let as_int = caps.get(2).map_or(1, |m| m.as_str().parse::<i32>().unwrap());

        let recipe = handle_recipe(as_text, as_int);

        return match recipe {
            None => {
                return match request.respond(Response::new_empty(StatusCode(404))) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e)
                    }
                };
            }
            Some(recipe) => {
                println!("{:?}", recipe);

                let response = tiny_http::Response::from_string(format!("{}", json!(recipe)));
                match request.respond(response) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
        };

    }

    return match request.respond(Response::new_empty(StatusCode(404))) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e)
        }
    };
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
