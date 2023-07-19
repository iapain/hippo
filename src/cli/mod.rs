use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Cli {
    /// Bitbucket Repositorysitory env
    pub file: PathBuf,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RepositoryVariable {
    key: String,
    value: String,
    secure: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryMetadata {
    name: String,
    description: Option<String>,
    home_page: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RepositoryDeployment {
    name: String,
    envs: Vec<RepositoryVariable>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RepositoryPipeline {
    enable: bool,
}


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct RepositorySpec {
    organization: String,
    slug: String,
    private: bool,
    main_branch: Option<String>,
    deployments: Option<Vec<RepositoryDeployment>>,
    pipeline: Option<RepositoryPipeline>,
    envs: Option<Vec<RepositoryVariable>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    api_version: String,
    source: String,
    metadata: RepositoryMetadata,
    spec: RepositorySpec,
}

#[cfg(test)]
mod tests {
    use super::PathBuf;
    use bpaf::*;

    #[test]
    fn test_cli_help() {
        let p = short('f').long("file").argument::<PathBuf>("FILE");
        let opts = construct!(p).to_options();

        let res = opts
            .run_inner(Args::from(&["-h"]))
            .unwrap_err()
            .unwrap_stdout();

        let expected_help = "Usage: -f FILE

Available options:
    -f, --file <FILE>
    -h, --help         Prints help information
";
        assert_eq!(res, expected_help);
    }
}
