use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_vault_transfer {


    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {

}


#[derive(Accounts)]
pub struct DepositToMerstabVault<'info> {
    pub user: Signer<'info>,
    pub user_token_acct: Account<'info, TokenAccount>,
    pub vault_token_acct: Account<'info, TokenAccount>,
    pub vault_authority: Signer<'info>,
    pub vault: Account<'info, Vault>,
}

#[account]
pub struct Vault {
    pub depositor: Pubkey,
    pub depositor_token_account: Pubkey,
    pub vault_token_account: Pubkey,
    pub vault_amount: u64
}