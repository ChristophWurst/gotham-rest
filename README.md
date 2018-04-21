# gotham-rest
RESTful extensions for the Gotham web framework

[![Build Status](https://travis-ci.org/ChristophWurst/gotham-rest.svg?branch=master)](https://travis-ci.org/ChristophWurst/gotham-rest)

This library tries to eliminate some boilerplate code required to write a Gotham REST service.

```rust
use gotham_rest::{Resource, ResourceIdPathExtractor, ResourceRouterBuilder};

struct BookResource;

impl Resource for BookResource {
    type Id = i32;

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
        route.resource::<BookResource>("/books");
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
```
