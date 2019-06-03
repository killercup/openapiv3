use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T: Clone> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String
    },
    Item(Arc<T>),
}

impl<T: Clone> ReferenceOr<T> {
    pub fn ref_(r: &str) -> Self {
        ReferenceOr::Reference { reference: r.to_owned() }
    }

    pub fn boxed_item(item: T) -> ReferenceOr<T> {
        ReferenceOr::Item(Arc::new(item))
    }
}
