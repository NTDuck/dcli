#[cfg(feature = "client")]
use serde::Serialize;

#[cfg(feature = "client")]
pub fn append_query(uri: &str, model: &impl Serialize) -> String {
    let query = serde_qs::to_string(model).unwrap();
    return format!("{}?{}", uri, query);
}
