#[macro_use]
extern crate failure;
extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod error;
mod resource;
mod routing;

pub use resource::{Resource, ResourceId};
pub use routing::{ResourceIdPathExtractor, ResourceRouterBuilder};
