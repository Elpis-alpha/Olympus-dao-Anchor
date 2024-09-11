use {anchor_lang::prelude::*, solana_program::pubkey::Pubkey};

pub const TREASURY_SEED: &str = "treasury_seed";
pub const TREASURY_LAMPORTS_SEED: &str = "treasury_lamports_seed";
pub const TREASURY_AUTH_SEED: &str = "treasury_authority_seed";
pub const TREASURY_MINT_SEED: &str = "treasury_mint_seed";
pub const TREASURY_STATE_SEED: &str = "treasury_state_seed";
pub const TREASURY_MINT_DECIMALS: u8 = 6;

#[account]
pub struct TreasuryState {
    pub bump: u8,
    pub amount: u64,
    pub token_mint: Pubkey,
    // pub staking_token_mint: Pubkey,
    // pub staking_token_mint_bump: u8,
    pub treasury_bump: u8,
    pub treasury_authority: Pubkey,
    pub treasury_authority_bump: u8,
    pub treasury_vault: Pubkey,
    pub treasury_vault_lamports: Pubkey,
}

#[account]
pub struct BondedUserState {
    pub bump: u8,
    pub user: Pubkey,
    pub user_token_account: Pubkey,
    pub treasury_state: Pubkey,
    pub bonded_lamports: u64,
    pub bond_date: i64,
}
