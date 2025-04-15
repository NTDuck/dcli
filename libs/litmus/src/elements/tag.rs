use crate::utils::aliases::{HashSet, MaybeOwnedStr};

#[derive(Clone)]
pub(super) struct Tags(HashSet<Tag>);

impl Tags {
    pub(super) fn chain(self, tag: impl Into<Tag>) -> Self {
        let mut tags = self.0;
        let tag = tag.into();
        tags.insert(tag);

        Self(tags)
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self(HashSet::new())
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

pub type Tag = MaybeOwnedStr;
