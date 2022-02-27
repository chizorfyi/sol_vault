use std::collections::HashSet;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, SetAuthority, Token, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_vault_transfer {

    use super::*;
  
    pub fn create_user_bank (ctx: Context<CreateUserBank>) -> Result<()> {
        let user_bank = &mut ctx.accounts.user_bank;
        user_bank.depositor = *ctx.accounts.depositor.key;
        user_bank.vault_count = 0;
        user_bank.user_vaults = HashSet::with_capacity(20);
        Ok(())
    }

    pub fn deposit_to_vault (ctx: Context<DepositToVault>, transfer_amount: u32) -> Result<()> {
        
        let (pda_account, bump) = Pubkey::find_program_address(&[ctx.accounts.depositor.key.as_ref()], ctx.program_id);
        token::set_authority(ctx.accounts.into_set_authority_context(), AuthorityType::AccountOwner, Some(pda_account))?;
        
        ctx.accounts.vault.depositor = *ctx.accounts.depositor.key;
        ctx.accounts.vault.depositor_token_account = *ctx.accounts.depositor_token_acct.to_account_info().key;
        ctx.accounts.vault.vault_token_account = *ctx.accounts.vault_token_acct.to_account_info().key;
        ctx.accounts.vault.vault_amount = transfer_amount;
        ctx.accounts.vault.pda_account = pda_account;

        // ctx.accounts.user_bank.add_to_bank(VaultDetails { 
        //     depositor: *ctx.accounts.depositor.key, 
        //     vault_pubkey: *ctx.accounts.vault.to_account_info().key, 
        // });
        ctx.accounts.user_bank.add_to_bank(*ctx.accounts.vault.to_account_info().key);

        Ok(())
    }

    pub fn withdraw_from_vault (ctx: Context<WithdrawFromVault> ) -> Result<()> {
        
        

        let (pda_account, bump) = Pubkey::find_program_address(&[ctx.accounts.depositor.key.as_ref()], ctx.program_id);
        let seed_signature = &[&ctx.accounts.depositor.key.as_ref()[..], &[bump]];

        token::transfer(ctx.accounts.into_withdraw_from_vault_context().with_signer(&[&seed_signature[..]]), ctx.accounts.vault.vault_amount as u64 )?;
        
        let user_bank = &mut ctx.accounts.user_bank;
        user_bank.remove_from_bank(ctx.accounts.vault.to_account_info().key);
        
        Ok(())

        
    }

}

#[derive(Accounts)]
pub struct CreateUserBank <'info> {
    
    #[account(init, payer=depositor, space= 8 + UserBank::LEN)]
    pub user_bank: Account<'info, UserBank>,
    
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct DepositToVault<'info> {
    
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    pub depositor_token_acct: Account<'info, TokenAccount>,
    
    pub vault_token_acct: Account<'info, TokenAccount>,
    
    #[account(init, payer=depositor, space= 8 + Vault::LEN)]
    pub vault: Account<'info, Vault>,
    
    // pub pda_account: AccountInfo<'info>,
    
    pub user_bank: Account<'info, UserBank>,
    
    pub token_program: Program<'info, Token>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> DepositToVault<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_token_acct.to_account_info().clone(),
            current_authority: self.depositor.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

//Withdraw from vault
#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    
    // #[account(mut)]
    pub depositor: AccountInfo<'info>,
    
    #[account(mut)]
    pub depositor_token_acct: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub vault_token_acct: Account<'info, TokenAccount>,
    
    pub pda_account: AccountInfo<'info>,
    
    #[account(mut, close = depositor)]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub user_bank: Account<'info, UserBank>,
    
    pub token_program: Program<'info, Token>,
    
    // pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFromVault<'info> {
    fn into_withdraw_from_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_token_acct.to_account_info().clone(),
            to: self.depositor_token_acct.to_account_info().clone(),
            authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


#[account]
pub struct Vault {
    
    //the depositor public key
    pub depositor: Pubkey, 
    
    // depositor token account
    pub depositor_token_account: Pubkey, 
    
    // depositor token account to transfer authority to pda
    pub vault_token_account: Pubkey, 
    
    pub pda_account: Pubkey,

    pub vault_amount: u32,
}

impl Vault {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 32;
}

#[account]
// #[derive(Default, Debug)]
pub struct UserBank {
    pub depositor: Pubkey,
    pub vault_count: u8,
    // pub user_vaults: [VaultDetails; 20],
    // pub user_vaults: HashSet<VaultDetails>,
    pub user_vaults: HashSet<Pubkey>,
}

impl UserBank {

      pub const LEN: usize = (32 * 20) //user_vaults 
      + 32 // depositor_pubkey 
      + 8; // vault_count

    fn add_to_bank (&mut self, vault_details: Pubkey) {
       
        if self.vault_count >= 20 {
            return;
        }

        self.user_vaults.insert(vault_details);
        self.vault_count.checked_add(1).unwrap();
    }

    fn remove_from_bank (&mut self, key: &Pubkey) -> bool {
        self.user_vaults.remove(key)
    }
    
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]  
// // #[zero_copy]
// pub struct VaultDetails {
//     pub depositor: Pubkey,
//     pub vault_pubkey: Pubkey,
// }