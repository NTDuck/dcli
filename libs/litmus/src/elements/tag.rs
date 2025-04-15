use crate::utils::aliases::{HashSet, MaybeOwnedStr};

pub(super) struct Tags(HashSet<Tag>);

impl Tags {
    pub(super) fn chain(self, tag: impl Into<Tag>) -> Self {
        let mut tags = self.0;
        let tag = tag.into();
        tags.insert(tag);

        Self(tags)
    }
}

impl From<Tag> for Tags {
    fn from(tag: Tag) -> Self {
        let mut tags = HashSet::new();
        tags.insert(tag);

        Self(tags)
    }
}

pub type Tag = MaybeOwnedStr;
