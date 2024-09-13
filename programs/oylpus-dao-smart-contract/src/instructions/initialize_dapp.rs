use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token_interface,
    std::mem::size_of,
};

pub fn handler_lamp(ctx: Context<InitializeLamp>) -> Result<()> {
    let lamports_vault = &mut ctx.accounts.treasury_vault_lamports;
    lamports_vault.balance = 0;
    lamports_vault.bump = ctx.bumps.treasury_vault_lamports;

    let initial_amount: u64 = 100;

    msg!("Treasury vault lamports balance initialized");

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key(),
        &ctx.accounts.treasury_vault_lamports.key(),
        initial_amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.treasury_vault_lamports.to_account_info(),
        ],
    )?;

    let vault = &mut ctx.accounts.treasury_vault_lamports;
    // MathematicalOverflowError
    vault.balance = vault
        .balance
        .checked_add(initial_amount)
        .ok_or(ErrorCode::MathematicalOverflowError)?;

    Ok(())
}

pub fn handler_tres(ctx: Context<InitializeDapp>) -> Result<()> {
    let treasury_state = &mut ctx.accounts.treasury_state;

    treasury_state.token_mint = ctx.accounts.token_mint.key();
    treasury_state.bump = ctx.bumps.treasury_state;

    treasury_state.treasury_authority = ctx.accounts.treasury_authority.key();
    treasury_state.treasury_authority_bump = ctx.bumps.treasury_authority;

    treasury_state.treasury_vault = ctx.accounts.treasury_vault.key();
    treasury_state.treasury_vault_bump = ctx.bumps.treasury_vault;
    treasury_state.treasury_vault_balance = 0;

    treasury_state.treasury_vault_lamports = ctx.accounts.treasury_vault_lamports.key();
    treasury_state.treasury_vault_lamports_bump = ctx.bumps.treasury_vault_lamports;
    msg!("Treasury state initialized");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLamp<'info> {
    // treasury authority account
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds = [TREASURY_AUTH_SEED.as_bytes()],
        bump
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    // treasury vaults for lamports
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<TreasuryVaultLamports>(),
        seeds = [treasury_authority.key().as_ref(), TREASURY_LAMPORTS_SEED.as_bytes()],
        bump,
    )]
    // pub treasury_vault_lamports: AccountInfo<'info>,
    pub treasury_vault_lamports: Account<'info, TreasuryVaultLamports>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
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

    // token mint account (only two)
    #[account(
        // init,
        // seeds = [TREASURY_MINT_SEED.as_bytes()],
        // bump,
        // payer = payer,
        mint::authority = treasury_authority,
        mint::decimals = TREASURY_MINT_DECIMALS,
        mint::token_program = token_program,
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,

    // treasury state account
    #[account(
        init,
        seeds = [token_mint.key().as_ref(), treasury_authority.key().as_ref(), TREASURY_STATE_SEED.as_bytes()],
        bump,
        payer = payer,
        space = 8 + size_of::<TreasuryState>(),
    )]
    pub treasury_state: Account<'info, TreasuryState>,

    // treasury vault
    #[account(
        init,
        payer = payer,
        seeds = [treasury_authority.key().as_ref(), TREASURY_SEED.as_bytes()],
        bump,
        token::mint = token_mint,
        token::authority = treasury_authority,
        token::token_program = token_program,
    )]
    pub treasury_vault: InterfaceAccount<'info, token_interface::TokenAccount>,

    // treasury vaults for lamports
    #[account(
        seeds = [treasury_authority.key().as_ref(), TREASURY_LAMPORTS_SEED.as_bytes()],
        bump,
    )]
    pub treasury_vault_lamports: Account<'info, TreasuryVaultLamports>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
