use std::ops::Deref;

use crate::utils::aliases::{HashSet, MaybeOwnedStr};

#[derive(Default, Clone)]
pub(super) struct Tags(HashSet<Tag>);

impl Tags {
    pub(super) fn chain(self, tag: impl Into<Tag>) -> Self {
        let mut tags = self.0;
        let tag = tag.into();
        tags.insert(tag);

        Self(tags)
    }
}

impl<U> FromIterator<U> for Tags
where 
    U: Into<Tag>,
{
    fn from_iter<Iter: IntoIterator<Item = U>>(iter: Iter) -> Self {
        let tags = iter
            .into_iter()
            .map(Into::into)
            .collect();

        Self(tags)
    }
}

impl Deref for Tags {
    type Target = HashSet<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Tag = MaybeOwnedStr;
