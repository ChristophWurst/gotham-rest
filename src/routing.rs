use std::panic::RefUnwindSafe;

use gotham::pipeline::chain::PipelineHandleChain;
use gotham::router::builder::{DefineSingleRoute, DrawRoutes, RouterBuilder};

use resource::{Resource, ResourceId};

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct ResourceIdPathExtractor {
    id: ResourceId,
}

impl ResourceIdPathExtractor {
    pub fn id(&self) -> ResourceId {
        self.id
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
        let id_path = format!("{}/:id", path);
        RouterBuilder::get(self, path).to(R::index);
        RouterBuilder::get(self, &id_path)
            .with_path_extractor::<ResourceIdPathExtractor>()
            .to(R::get);
        RouterBuilder::put(self, path).to(R::create);
        RouterBuilder::post(self, &id_path)
            .with_path_extractor::<ResourceIdPathExtractor>()
            .to(R::update);
        RouterBuilder::delete(self, &id_path)
            .with_path_extractor::<ResourceIdPathExtractor>()
            .to(R::delete);
    }
}
