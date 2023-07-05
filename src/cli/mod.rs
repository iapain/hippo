use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Cli {
    /// Bitbucket repository env
    pub file: PathBuf,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RepoVariable {
    key: String,
    value: String,
    secure: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Repo {
    source: String,
    slug: String,
    variables: Vec<RepoVariable>,
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
