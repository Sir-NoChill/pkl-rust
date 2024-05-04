use std::{collections::HashMap, path::Path, borrow::Cow};
use dirs::home_dir;
use std::env;

use super::{msg_api::outgoing::{ResourceReader, ModuleReader}, logger::Logger};

//TODO documentation
pub struct EvaluatorOptions<'a> {
    properties: HashMap<String, String>,

    env: HashMap<String, String>,

    module_paths: Vec<String>,

    logger: Logger,

    output_format: String,

    allowed_modules: Vec<String>,

    allowed_resources: Vec<String>,

    resource_readers: Vec<ResourceReader>,

    module_readers: Vec<ModuleReader>,

    cache_dir: Cow<'a, Path>,

    root_dir: String, //TODO this should also be a path

    project_dir: String, //TODO this should be a path

    declared_project_dependency: ProjectDependencies
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

impl<'a> Default for EvaluatorOptions<'a> {
    fn default() -> Self {
        let allowed_resources: Vec<String> = vec_of_strings!["http:", "https:", "file:",
                                                     "env:", "prop:", "modulepath:",
                                                     "package:", "projectpackage:"];
        let allowed_modules: Vec<String> = vec_of_strings!["pkl:", "repl:", "file:", "http:",
                                                   "https:", "modulepath:", "package:",
                                                   "projectpackage:"];
        let mut dirname = home_dir().expect("No home directory found.");
        dirname.push("/.pkl/cache");

        let mut os_env: HashMap<String, String> = Default::default();

        for (key, value) in env::vars_os() {
            os_env.insert(key.to_str().expect("Failed to obtain key").into(),
                            value.to_str().expect("Failed to obtain value for key").into());
        }

        Self {
            properties: Default::default(),
            env: os_env,
            module_paths: Default::default(),
            logger: Default::default(),
            output_format: Default::default(),
            allowed_modules,
            allowed_resources,
            resource_readers: Default::default(),
            module_readers: Default::default(),
            cache_dir: Cow::from(dirname),
            root_dir: Default::default(),
            project_dir: Default::default(),
            declared_project_dependency: Default::default(),
        }
    }
}

#[derive(Default)]
struct ProjectRemoteDependency {
    package_uri: String, // TODO this should be a path
    checksums: String, //TODO should this be unified with the msg_api::Checksums type?
}

#[derive(Default)]
struct ProjectLocalDependency {
    package_uri: String,
    project_file_uri: String,
    dependencies: ProjectDependencies
}

#[derive(Default)]
struct ProjectDependencies {
    local_dependencies: HashMap<String, ProjectLocalDependency>,
    remote_dependencies: HashMap<String, ProjectRemoteDependency>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_test() {
        let defaults: EvaluatorOptions = Default::default();

        defaults.logger.trace("hello, ".into(), "world".into());
    }
}
