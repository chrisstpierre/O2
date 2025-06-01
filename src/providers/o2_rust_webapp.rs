use crate::formulation_provider::{CommandProvider, ExecMode, MyCommand};
use clap::Parser;
use std::process::Command;

/// Define the context using `clap::Parser` derive.
/// This automatically handles CLI arguments parsing and help message generation.
#[derive(Parser, Debug)]
#[command(name = "o2-rust-webapp")]
pub struct O2RustWebAppContext {
    /// The name of the project
    #[arg(short, long)]
    pub project_name: String,

    /// Whether to include a frontend
    #[arg(long, default_value_t = false)]
    pub wants_frontend: bool,

    /// Which cloud provider to deploy to
    #[arg(short, long, default_value = "aws")]
    pub cloud_provider: String,
}

pub struct O2RustWebAppProvider;

impl CommandProvider for O2RustWebAppProvider {
    type Context = O2RustWebAppContext;

    /// Parse from CLI arguments instead of interactive prompts
    fn init_context() -> std::io::Result<Self::Context> {
        // Parse command-line args using clap
        let ctx = O2RustWebAppContext::parse();
        Ok(ctx)
    }

    fn get_commands(ctx: &Self::Context) -> Vec<MyCommand> {
        let mut cmds = Vec::new();

        let mut build_cmd = Command::new("echo");
        build_cmd.arg(format!("Building project: {}", ctx.project_name));
        cmds.push(MyCommand::new(build_cmd, ExecMode::Sequential));

        if ctx.wants_frontend {
            let mut frontend_cmd = Command::new("echo");
            frontend_cmd.arg("Setting up frontend for o2-rust-webapp");
            cmds.push(MyCommand::new(frontend_cmd, ExecMode::Sequential));
        }

        let mut cloud_cmd = Command::new("echo");
        cloud_cmd.arg(format!(
            "Deploying to cloud provider: {}",
            ctx.cloud_provider
        ));
        cmds.push(MyCommand::new(cloud_cmd, ExecMode::Parallel));

        cmds
    }
}
