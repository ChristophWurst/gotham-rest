use std::panic::RefUnwindSafe;

use gotham::pipeline::chain::PipelineHandleChain;
use gotham::router::builder::{DefineSingleRoute, DrawRoutes, RouterBuilder};

use resource::{Resource, ResourceId};

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct ResourceIdPathExtractor<ID: RefUnwindSafe + 'static> {
    id: ID,
}

impl<ID> ResourceIdPathExtractor<ID>
where
    ID: ResourceId,
{
    pub fn id_ref(&self) -> &ID {
        &self.id
    }
}

impl<ID> ResourceIdPathExtractor<ID>
where
    ID: ResourceId + Copy,
{
    pub fn id(&self) -> ID {
        self.id
    }
}

pub trait ResourceRouterBuilder {
    fn resource<R>(&mut self, path: &str)
    where
        R: Resource + 'static;
}

impl<'a, C, P> ResourceRouterBuilder for RouterBuilder<'a, C, P>
where
    C: PipelineHandleChain<P> + Copy + Send + Sync + 'static,
    P: RefUnwindSafe + Send + Sync + 'static,
{
    fn resource<R: Resource + 'static>(&mut self, path: &str) {
        let id_path = format!("{}/:id", path);
        RouterBuilder::get(self, path).to(R::index);
        RouterBuilder::get(self, &id_path)
            .with_path_extractor::<ResourceIdPathExtractor<R::Id>>()
            .to(R::get);
        RouterBuilder::put(self, path).to(R::create);
        RouterBuilder::post(self, &id_path)
            .with_path_extractor::<ResourceIdPathExtractor<R::Id>>()
            .to(R::update);
        RouterBuilder::delete(self, &id_path)
            .with_path_extractor::<ResourceIdPathExtractor<R::Id>>()
            .to(R::delete);
    }
}
