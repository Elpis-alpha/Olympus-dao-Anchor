use anchor_lang::prelude::*;

declare_id!("B8i8uoj4JGaJqiYaaEHQFdrHUVFEpgvtkFpAvbZxLbML");

#[program]
pub mod oylpus_dao_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
