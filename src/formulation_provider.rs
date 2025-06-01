use std::process::Command;

pub enum ExecMode {
    Sequential,
    Parallel,
}

pub struct MyCommand {
    pub command: Command,
    pub exec_mode: ExecMode,
}

impl MyCommand {
    pub fn new(command: Command, exec_mode: ExecMode) -> Self {
        Self { command, exec_mode }
    }
}

pub trait CommandProvider {
    type Context;

    fn init_context() -> std::io::Result<Self::Context>;

    fn get_commands(ctx: &Self::Context) -> Vec<MyCommand>;

    fn extend_commands(ctx: &Self::Context, existing: &mut Vec<MyCommand>) {
        let new_cmds = Self::get_commands(ctx);
        existing.extend(new_cmds);
    }
}
