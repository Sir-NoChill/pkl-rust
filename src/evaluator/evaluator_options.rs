use std::{collections::HashMap, path::PathBuf};
use dirs::home_dir;
use std::env;

use super::{msg_api::outgoing::{ResourceReader, ModuleReader}, logger::Logger};

//TODO documentation
//TODO this should be taken from a pkl file in the future
pub struct EvaluatorOptions {
    pub properties: HashMap<String, String>,
    pub env: HashMap<String, String>,
    pub module_paths: Vec<String>,
    pub logger: Logger,
    pub output_format: String,
    pub allowed_modules: Vec<String>,
    pub allowed_resources: Vec<String>,
    pub resource_readers: Vec<ResourceReader>,
    pub module_readers: Vec<ModuleReader>,
    pub cache_dir: PathBuf,
    pub root_dir: String, //TODO this should also be a path
    pub project_dir: String, //TODO this should be a path
    pub declared_project_dependency: ProjectDependencies
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

impl Default for EvaluatorOptions {
    fn default() -> Self {
        let allowed_resources: Vec<String> = vec_of_strings!["http:", "https:", "file:",
                                                     "env:", "prop:", "modulepath:",
                                                     "package:", "projectpackage:"];
        let allowed_modules: Vec<String> = vec_of_strings!["pkl:", "repl:", "file:", "http:",
                                                   "https:", "modulepath:", "package:",
                                                   "projectpackage:"];
        let mut dirname = home_dir().expect("No home directory found!");
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
            cache_dir: dirname,
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
