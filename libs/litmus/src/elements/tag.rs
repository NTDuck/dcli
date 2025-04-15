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

impl From<Tag> for Tags {
    fn from(tag: Tag) -> Self {
        let mut tags = HashSet::new();
        tags.insert(tag);

        Self(tags)
    }
}

impl FromIterator<Tag> for Tags {
    fn from_iter<TagsIter: IntoIterator<Item = Tag>>(tags: TagsIter) -> Self {
        let tags = tags
            .into_iter()
            .collect();

        Self(tags)
    }
}

pub type Tag = MaybeOwnedStr;
