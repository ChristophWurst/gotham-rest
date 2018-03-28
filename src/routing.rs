use std::panic::RefUnwindSafe;

use gotham::pipeline::chain::PipelineHandleChain;
use gotham::router::builder::{DefineSingleRoute, DrawRoutes, RouterBuilder};

use resource::Resource;

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
