use std::{fmt, sync::Arc};

use clap::{Command, Parser, Subcommand, ValueEnum};

mod formulation_provider;
mod providers;
mod registry;
use crate::registry::Registry;
use std::process::Command as Cmd;
struct FormulationCommands {
    commands: Arc<[Cmd]>,
    formulation: Formulation,
}

fn echo_cmd() -> Cmd {
    let mut echo_cmd = Cmd::new("bash");
    echo_cmd.arg("-c").arg("echo Hello from Bash!");
    echo_cmd
}

fn o2_rust_app_cmd() -> Cmd {
    let mut echo_cmd = Cmd::new("bash");
    echo_cmd.arg("-c").arg("echo Hello from Bash!");
    echo_cmd
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new project
    #[command(arg_required_else_help = true, alias = "create")]
    New {
        /// The Forumulation name
        #[arg(value_enum)]
        formulation: Formulation,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum Formulation {
    O2AppRust,
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::New { formulation } => write!(f, "Generating {:#?}", formulation),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, name = "o2", about = "A fictional versioning CLI", long_about = None)]
struct O2 {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    // use super::echo_cmd; // Not needed, echo_cmd is in scope
    let args = O2::parse();
    let registry = Registry::new();
    if let Commands::New { formulation } = args.command {
        registry.process(formulation);
    }

    // let mut echo_cmd = Cmd::new("bash").arg("-c").arg("echo Hello from Bash!");

    // let cmds = FormulationCommands {
    //     formulation: Formulation::O2AppRust,
    //     commands: Arc::new([echo_cmd(), echo_cmd()]),
    // };

    // for mut cmd in cmds.commands {
    //     let status = cmd
    //         .spawn()
    //         .expect("Failed to spawn command")
    //         .wait()
    //         .expect("Failed to wait on child");
    //     println!("Command exited with status: {}", status);
    // }

    // println!("Hello {} !", args.command)
}
