use std::{env::current_dir, path::PathBuf};

use url::Url;


/// Represents a source for Pkl evaluation
pub struct ModuleSource {
    /// The URI of the resource.
    uri: Url,

    /// The text contents of the resource, if any exists.
    ///
    /// If Contents is empty, it gets resolved by Pkl during evaluation time.
    /// If the scheme of the Uri matches a ModuleReader, it will be used to resolve the module.
    contents: Option<String>,
}

impl ModuleSource {
    /// Immutable access for the uri of a ModuleSource
    pub fn uri(&self) -> &Url {return &self.uri;}

    /// Immutable access for the contents of a ModuleSource
    pub fn contents(&self) -> &Option<String> {return &self.contents}
}

/// Builds a ModuleSource from a file path
///
/// # Example
///
/// ```
/// use pkl_rust::evaluator::module_source::file_source;
/// let source = file_source("test/file.pkl".into());
///
/// assert_eq!(source.uri().scheme(), "file");
/// // assert_eq!(source.uri.path(), "/home/$USER/$DIR/test/file.pkl");
/// ```
pub fn file_source(path: PathBuf) -> ModuleSource {
    let result: PathBuf;
    //TODO fix, this is sloppy
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

/// Builds a ModuleSource from a string input
///
/// # Example
///
/// ```
/// use pkl_rust::evaluator::module_source::text_source;
///
/// let pkl = text_source("Attribute = 1".into());
/// assert_eq!(pkl.uri().scheme(), "repl");
/// ```
pub fn text_source(text: String) -> ModuleSource {
    let uri_entry = Url::parse("repl:/").expect("Failed to parse uri entry");
    //TODO this is also sloppy
    return ModuleSource{ uri: uri_entry, contents: Some(text), };
}

/// Builds a ModuleSource using the input uri
/// # Example
///
/// ```
/// use pkl_rust::evaluator::module_source::uri_source;
/// use url::Url;
///
/// let pkl = uri_source(Url::parse("file:/test/pkl.pkl").unwrap());
/// assert_eq!(pkl.uri().scheme(), "file");
/// ```
pub fn uri_source(uri: Url) -> ModuleSource {
    return ModuleSource{ uri, contents: None };
}
