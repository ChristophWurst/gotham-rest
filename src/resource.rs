use failure::Fail;
use futures::future::err;
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use gotham::state::State;
use hyper::StatusCode;

use error::ResourceError;

pub type ResourceId = i32;

pub trait Resource {
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

    fn create(state: State) -> Box<HandlerFuture> {
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
