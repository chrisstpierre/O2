use super::*;
use std::str::FromStr;
// mod registry;
pub struct Registry;
use crate::formulation_provider::CommandProvider;
use crate::providers::o2_rust_webapp::O2RustWebAppProvider;
impl Registry {
    pub fn new() -> Self {
        Registry {}
    }
    // fn get_provider_from_enum<P, C>(formulation: Formulation) -> P
    // where
    //     P: CommandProvider<Context = C>,
    // {
    //     match formulation {
    //         Formulation::O2AppRust => O2RustWebAppProvider,
    //     }
    // }

    pub fn process(&self, formulation: Formulation) {
        // let provider = Self::get_provider_from_enum(formulation);
        // let provider = O2RustWebAppProvider;
        if let Ok(provider_with_context) = O2RustWebAppProvider::init_context() {
            let mut cmds = O2RustWebAppProvider::get_commands(&provider_with_context);
            for cmd in cmds.iter_mut() {
                let status = cmd
                    .command
                    .spawn()
                    .expect("Failed to spawn command")
                    .wait()
                    .expect("Failed to wait on child");
                println!("Command exited with status: {}", status);
            }
        }
        // provider.init_context();
    }
}
