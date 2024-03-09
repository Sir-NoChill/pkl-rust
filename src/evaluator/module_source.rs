use std::{env::current_dir, path::PathBuf};

use url::Url;


/// Represents a source for Pkl evaluation
pub struct ModuleSource {
    /// The URI of the resource.
    pub uri: Url,

    /// The text contents of the resource, if any exists.
    ///
    /// If Contents is empty, it gets resolved by Pkl during evaluation time.
    /// If the scheme of the Uri matches a ModuleReader, it will be used to resolve the module.
    pub contents: Option<String>,
}

/// Builds a ModuleSource from a file path
///
/// # Example
///
/// ```
/// use pkl_rust::evaluator::module_source::file_source;
/// let source = file_source("test/file.pkl".into());
///
/// assert_eq!(source.uri.scheme(), "file");
/// // assert_eq!(source.uri.path(), "/home/$USER/$DIR/test/file.pkl");
/// ```
pub fn file_source(path: PathBuf) -> ModuleSource {
    let result: PathBuf;
    let url_entry = Url::parse_with_params("file:/", &[("scheme", "file")]).expect("Failed to convert path to uri");

    if !path.is_absolute() {
        let pwd: PathBuf = current_dir().expect("Failed to resolve current working dir");
        result = pwd.join(&path);
        println!("Absolute path: {:?}", result);
    } else {
        result = path;
    }

    let url_string: &str = result.to_str().expect("Filed to convert path to string");
    let res = url_entry.join(url_string).expect("Failed to join to the rest of the path");

    return ModuleSource{uri: res, contents: None};
}
