#[macro_use]
extern crate failure;
extern crate futures;
extern crate gotham;
extern crate hyper;

mod error;
mod resource;
mod routing;

pub use resource::Resource;
pub use routing::ResourceRouterBuilder;
