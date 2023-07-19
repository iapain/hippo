use bpaf::*;
use hippo::cli::{Cli, Repository};
use serde_yaml;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = long("file")
        .short('f')
        .help("YAML manifest file")
        .argument::<PathBuf>("FILE");

    let opts = construct!(Cli { file }).to_options().run();
    let fd = std::fs::read_to_string(opts.file)?;
    let parsed: Repository = serde_yaml::from_str::<Repository>(&fd)?;
    println!("{:?}", parsed);
    Ok(())
}
