pub mod errors;
pub mod instructions;
pub mod utils;
pub mod state;
use {anchor_lang::prelude::*, instructions::*};


declare_id!("B8i8uoj4JGaJqiYaaEHQFdrHUVFEpgvtkFpAvbZxLbML");

#[program]
pub mod oylpus_dao_smart_contract {
    use super::*;

    pub fn initialize_dapp(ctx: Context<InitializeDapp>) -> Result<()> {
        msg!("Initializing...");

        initialize_dapp::handler(ctx)
    }
}
