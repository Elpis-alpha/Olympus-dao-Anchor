use anchor_lang::prelude::*;
use crate::state::TREASURY_AUTH_SEED;

pub fn handler(ctx: Context<InitializeDapp>) -> Result<()> {
    let treasury_authority = &mut ctx.accounts.treasury_authority;
    msg!("Pool state initialized");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeDapp<'info> {
    // treasury authority account
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds = [TREASURY_AUTH_SEED.as_bytes()],
        bump
    )]
    pub treasury_authority: UncheckedAccount<'info>,
}
