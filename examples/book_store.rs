extern crate futures;
extern crate gotham;
extern crate gotham_rest;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use futures::future::ok;
use gotham::handler::HandlerFuture;
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham_rest::{Resource, ResourceId, ResourceIdPathExtractor, ResourceRouterBuilder};
use hyper::StatusCode;

pub fn say_hello(state: State) -> Box<HandlerFuture> {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((format!("Hello, Bookstore!").into_bytes(), mime::TEXT_PLAIN)),
    );

    Box::new(ok((state, res)))
}

#[derive(Serialize)]
struct Book {
    id: i32,
    name: String,
}

struct BookResource;

fn get_books() -> Vec<Book> {
    vec![
        Book {
            id: 1,
            name: "Programming Rust".to_string(),
        },
    ]
}

fn get_book(id: ResourceId) -> Option<Book> {
    get_books()
        .into_iter()
        .filter(|ref b| b.id == id)
        .take(1)
        .next()
}

impl Resource for BookResource {
    fn index(state: State) -> Box<HandlerFuture> {
        let books = get_books();

        let json = serde_json::to_string(&books).unwrap();

        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        );

        Box::new(ok((state, res)))
    }

    fn get(state: State) -> Box<HandlerFuture> {
        let id = ResourceIdPathExtractor::borrow_from(&state).id();
        let book = get_book(id);

        let json = serde_json::to_string(&book).unwrap();

        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        );

        Box::new(ok((state, res)))
    }
}

fn router() -> Router {
    let (chain, pipelines) = single_pipeline(new_pipeline().build());

    build_router(chain, pipelines, |route| {
        route.get("/").to(say_hello);
        route.resource::<BookResource>("/books");
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
