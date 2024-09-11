use {crate::state::*, anchor_lang::prelude::*, anchor_spl::token_interface};

pub fn handler_bond(ctx: Context<BondLamportsContext>, lamports_to_bond: u64) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct BondLamportsContext<'info> {
    // treasury authority account
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds = [TREASURY_AUTH_SEED.as_bytes()],
        bump
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    // token mint account (only two)
    #[account(
        mint::authority = treasury_authority,
        mint::decimals = TREASURY_MINT_DECIMALS,
        mint::token_program = token_program,
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,

    #[account(
        seeds = [token_mint.key().as_ref(), TREASURY_STATE_SEED.as_bytes()],
        bump,
    )]
    pub treasury_state: Account<'info, TreasuryState>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
    // pub rent: Sysvar<'info, Rent>,
}
