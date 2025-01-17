use anyhow::Result;
/// extract str to type<T>
pub fn extract<T: serde::de::DeserializeOwned>(data: &str) -> Result<T> {
    Ok(serde_json::from_str::<T>(data)?)
}
