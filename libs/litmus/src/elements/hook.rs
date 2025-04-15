use crate::elements::Tag;
use crate::utils::aliases::{Arc, HashMap};

use super::World;

#[derive(Default, Clone)]
pub(super) struct Hooks<WorldImpl> {
    tagged: HashMap<Tag, Arc<dyn HookFn<WorldImpl>>>,
    untagged: Vec<Arc<dyn HookFn<WorldImpl>>>,
}

impl<WorldImpl> Hooks<WorldImpl> 
where 
    WorldImpl: World,
{
    pub(super) fn add(self, tag: Tag, hook: impl HookFn<WorldImpl>) -> Self {
        let mut tagged = self.tagged;
        let hook = Arc::new(hook);
        tagged.insert(tag, hook);

        Self {
            tagged,
            ..self
        }
    }

    pub(super) fn add_untagged(self, hook: impl HookFn<WorldImpl>) -> Self {
        let mut untagged = self.untagged;
        let hook = Arc::new(hook);
        untagged.push(hook);

        Self {
            untagged,
            ..self
        }
    }
}

impl<T, U, HookFnImpl, WorldImpl> From<(T, HookFnImpl)> for Hooks<WorldImpl>
where 
    T: IntoIterator<Item = U>,
    U: Into<Tag>,
    HookFnImpl: HookFn<WorldImpl>,
    WorldImpl: World,
{
    fn from((iter, hook): (T, HookFnImpl)) -> Self {
        let hook = Arc::new(hook);
        let mut tagged: HashMap<Tag, Arc<dyn HookFn<WorldImpl>>> = HashMap::new();

        let _ = iter
            .into_iter()
            .map(Into::into)
            .map(|tag| tagged.insert(tag, hook.clone()));

        Self {
            tagged,
            untagged: Vec::default(),
        }
    }
}

impl<HookFnImpl, WorldImpl> From<HookFnImpl> for Hooks<WorldImpl>
where 
    HookFnImpl: HookFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(hook: HookFnImpl) -> Self {
        let hook = Arc::new(hook);
        let untagged: Vec<Arc<dyn HookFn<WorldImpl>>> = vec![hook];

        Self {
            tagged: HashMap::default(),
            untagged,
        }
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
