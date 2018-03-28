#[macro_use]
extern crate failure;
extern crate futures;
extern crate gotham;
extern crate hyper;

use failure::Fail;
use futures::future::err;
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use gotham::state::State;
use hyper::StatusCode;

#[derive(Debug, Fail)]
enum ResourceError {
    #[fail(display = "method not allowed")] MethodNotAllowed,
}

pub trait Resource {
    type Id: Sized;

    fn index(state: State) -> Box<HandlerFuture> {
        let error = ResourceError::MethodNotAllowed;
        Box::new(err((
            state,
            HandlerError::with_status(
                error.compat().into_handler_error(),
                StatusCode::MethodNotAllowed,
            ),
        )))
    }

    fn get(state: State) -> Box<HandlerFuture> {
        let error = ResourceError::MethodNotAllowed;
        Box::new(err((
            state,
            HandlerError::with_status(
                error.compat().into_handler_error(),
                StatusCode::MethodNotAllowed,
            ),
        )))
    }

    fn update(state: State) -> Box<HandlerFuture> {
        let error = ResourceError::MethodNotAllowed;
        Box::new(err((
            state,
            HandlerError::with_status(
                error.compat().into_handler_error(),
                StatusCode::MethodNotAllowed,
            ),
        )))
    }

    fn delete(state: State) -> Box<HandlerFuture> {
        let error = ResourceError::MethodNotAllowed;
        Box::new(err((
            state,
            HandlerError::with_status(
                error.compat().into_handler_error(),
                StatusCode::MethodNotAllowed,
            ),
        )))
    }
}
