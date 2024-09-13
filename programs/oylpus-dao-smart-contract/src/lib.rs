pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
use {anchor_lang::prelude::*, instructions::*};

declare_id!("B8i8uoj4JGaJqiYaaEHQFdrHUVFEpgvtkFpAvbZxLbML");

#[program]
pub mod oylpus_dao_smart_contract {
    use super::*;

    pub fn initialize_lamp(ctx: Context<InitializeLamp>) -> Result<()> {
        msg!("Initializing...");

        initialize_dapp::handler_lamp(ctx)
    }

    pub fn initialize_dapp(ctx: Context<InitializeDapp>) -> Result<()> {
        msg!("Initializing...");

        initialize_dapp::handler_tres(ctx)
    }

    pub fn bond_lamports(ctx: Context<BondLamportsContext>, lamports_to_bond: u64) -> Result<()> {
        msg!("Bonding {} lamports ...", lamports_to_bond);

        bond_lamports::handler_bond(ctx, lamports_to_bond)
    }
}
