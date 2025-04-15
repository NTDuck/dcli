use crate::elements::Tag;
use crate::utils::aliases::{Arc, HashMap};

use super::World;

pub(super) struct Hooks<WorldImpl>(HashMap<Tag, Arc<dyn HookFn<WorldImpl>>>);

impl<WorldImpl> Hooks<WorldImpl> 
where 
    WorldImpl: World,
{
    pub(super) fn add(self, tag: Tag, hook: impl HookFn<WorldImpl>) -> Self {
        let mut hooks_by_tags = self.0;
        let hook = Arc::new(hook);
        hooks_by_tags.insert(tag, hook);

        Self(hooks_by_tags)
    }
}

impl<HookFnImpl, WorldImpl> From<(Tag, HookFnImpl)> for Hooks<WorldImpl>
where 
    HookFnImpl: HookFn<WorldImpl>,
    WorldImpl: World,
{
    fn from((tag, hook): (Tag, HookFnImpl)) -> Self {
        let mut hooks_by_tags: HashMap<Tag, Arc<dyn HookFn<WorldImpl>>> = HashMap::new();
        let hook = Arc::new(hook);
        hooks_by_tags.insert(tag, hook);

        Self(hooks_by_tags)
    }
}

pub trait HookFn<WorldImpl>: Fn(&mut WorldImpl) -> () + Send + Sync + 'static {}

impl<T, WorldImpl> HookFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> () + Send + Sync + 'static,
{
}

pub trait GlobalHookFn: FnOnce() -> () + Send + Sync + 'static {}

impl<T> GlobalHookFn for T
where 
    T: FnOnce() -> () + Send + Sync + 'static,
{
}
