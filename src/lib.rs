#[macro_use]
extern crate failure;
extern crate futures;
extern crate gotham;
extern crate hyper;

use std::panic::RefUnwindSafe;

use failure::Fail;
use futures::future::err;
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use gotham::pipeline::chain::PipelineHandleChain;
use gotham::router::builder::{DefineSingleRoute, DrawRoutes, RouterBuilder};
use gotham::state::State;
use hyper::StatusCode;

#[derive(Debug, Fail)]
enum ResourceError {
    #[fail(display = "method not allowed")] MethodNotAllowed,
}

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

pub trait ResourceRouterBuilder {
    fn resource<R: Resource + 'static>(&mut self, path: &str);
}

impl<'a, C, P> ResourceRouterBuilder for RouterBuilder<'a, C, P>
where
    C: PipelineHandleChain<P> + Copy + Send + Sync + 'static,
    P: RefUnwindSafe + Send + Sync + 'static,
{
    fn resource<R: Resource + 'static>(&mut self, path: &str) {
        RouterBuilder::get(self, path).to(R::index);
    }
}
