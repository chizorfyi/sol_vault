use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, SetAuthority};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_vault_transfer {


    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_user_bank (ctx: Context<CreateUserBank>, depositor: Pubkey ) -> Result<()> {
        let mut user_bank = ctx.accounts.user_bank.load_init()?;
        user_bank.depositor = depositor;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {

}

#[derive(Accounts)]
pub struct CreateUserBank <'info> {
    
    #[account(zero)]
    pub user_bank: AccountLoader<'info, UserBank>,
    // pub depositor: Signer<'info>,
}


#[derive(Accounts)]
pub struct DepositToMerstabVault<'info> {
    pub user: Signer<'info>,
    pub user_token_acct: Account<'info, TokenAccount>,
    pub vault_token_acct: Account<'info, TokenAccount>,
    pub vault_authority: Signer<'info>,
    pub vault: Account<'info, Vault>,
    pub user_bank: AccountLoader<'info, UserBank>,
}

// impl<'info> DepositToMerstabVault<'info> {
//     fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
//         let cpi_accounts = SetAuthority {
//             account_or_mint: self.pda_deposit_token_account.to_account_info().clone(),
//             current_authority: self.pda_account.clone(),
//         };
//         let cpi_program = self.token_program.to_account_info();
//         CpiContext::new(cpi_program, cpi_accounts)
//     }
// }

#[account]
pub struct Vault {
    
    //the depositor public key
    pub depositor: Pubkey, 
    
    // depositor token account
    pub depositor_token_account: Pubkey, 
    
    // depositor token account to transfer authority to pda
    pub vault_token_account: Pubkey, 
    
    pub vault_amount: u64,
    
    pub pda_account: Pubkey
}

#[account(zero_copy)]
// #[derive(Default)]
pub struct UserBank {
    pub depositor: Pubkey,
    pub vault_count: u8,
    pub user_vaults: [VaultDetails; 100],
}

// impl UserBank {
//     fn append (&mut self) {}  
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]  //would be used if UserBank account would not hit stack limit
#[zero_copy]
pub struct VaultDetails {
    pub depositor: Pubkey,
    pub vault_pubkey: Pubkey,
}