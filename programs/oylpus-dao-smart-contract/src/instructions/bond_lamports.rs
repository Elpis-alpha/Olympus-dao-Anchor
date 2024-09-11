use {crate::state::*, anchor_lang::prelude::*, anchor_spl::token_interface};

pub fn handler_bond(ctx: Context<BondLamportsContext>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct BondLamportsContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
    // pub rent: Sysvar<'info, Rent>,
}
