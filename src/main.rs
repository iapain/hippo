use bpaf::*;
use hippo::cli::{Cli, Repository, Opt, Command};
use serde_yaml;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = positional::<Command>("Command").help("context command: create | delete");
    let context = construct!(Opt::Context {
        command
    })
    .to_options()
    .descr("Add or removes a context");
    let context_cmd = context.command("context");

    let file = long("file")
        .short('f')
        .help("YAML manifest file")
        .argument::<PathBuf>("FILE");

    let apply = construct!(Opt::Apply {
        file,
    })
    .to_options()
    .descr("Apply a yaml file");
    let apply_cmd = apply.command("apply");

    let opts = construct!([context_cmd, apply_cmd])
        .to_options()
        .descr("The stupid content tracker")
        .run();
   
    match opts {
        Opt::Apply{ file: file_path} => {
            let fd = std::fs::read_to_string(file_path)?;
            let parsed: Repository = serde_yaml::from_str::<Repository>(&fd)?;
            println!("{:?}", parsed);
        },
        _ => {
            println!("{:?}", opts);
        }
    }
    
    Ok(())
}
