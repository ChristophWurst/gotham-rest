use std::panic::RefUnwindSafe;

use failure::Fail;
use futures::future::err;
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use gotham::state::State;
use hyper::StatusCode;
use serde::de::DeserializeOwned;

use error::ResourceError;

pub trait ResourceId: DeserializeOwned + RefUnwindSafe + Send + Sized + Sync {}

impl ResourceId for i32 {}
impl ResourceId for i64 {}
impl ResourceId for String {}

pub trait Resource {
    type Id: ResourceId;

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
