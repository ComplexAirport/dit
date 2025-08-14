/// Accepts two parameters: \
/// The first one is the model name to implement the methods for. \
/// The second one should be an enum with `SerializationError(String)`
/// and `DeserializationError(String)` properties. Example:
/// `impl_read_write_model!(Tree, TreeErr)`
///
/// Implements two important methods for api_models:
/// 1. `write_to(&self, path: impl AsRef<Path>)` - serializes and writes the model to the given path
/// 2. `read_from(path: impl AsRef<Path>)` - reads and deserializes the model and returns it
#[macro_export]
macro_rules! impl_read_write_model {
    ($model:ident, $err_enum:ident) => {
        impl $model {
            pub fn write_to(&self, path: &Path) -> DitResult<()> {
                let serialized = serde_json::to_string(self)
                    .map_err(|_| $err_enum::SerializationError(self.hash.clone()))?;

                write_to_file(path, serialized)
            }

            pub fn read_from(path: &Path) -> DitResult<$model> {
                let serialized = read_to_string(&path)?;

                let model: $model = serde_json::from_str(&serialized)
                    .map_err(|_| $err_enum::DeserializationError(path_to_string(path)))?;

                Ok(model)
            }
        }
    }
}
