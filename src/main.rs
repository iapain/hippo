use bpaf::Bpaf;
use std::path::PathBuf;
use serde::{Deserialize};
use serde_yaml;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
struct Cli {
    /// Bitbucket repository env
    file: PathBuf
}

#[derive(Debug, Deserialize)]
struct RepoVariable {
    key: String,
    value: String,
    secure: Option<bool>
}

#[derive(Debug, Deserialize)]
struct Repo { 
    source: String,
    slug: String,
    variables: Vec<RepoVariable>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = cli().run();
    let fd = std::fs::read_to_string(opts.file)?;
    let parsed: Repo = serde_yaml::from_str::<Repo>(&fd)?;
    println!("{:?}", parsed);
    Ok(())
}
