use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, SetAuthority, Token};
use spl_token::instruction::AuthorityType;
use std::result;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_vault_transfer {



    use super::*;
  

    pub fn create_user_bank (ctx: Context<CreateUserBank>) -> Result<()> {
        let user_bank = &mut ctx.accounts.user_bank;
        user_bank.depositor = *ctx.accounts.depositor.key;
        user_bank.vault_count = 0;
        Ok(())
    }

    pub fn deposit_to_vault (ctx: Context<DepositToVault>, transfer_amount: u32) -> Result<()> {
        
        let (pda_account, _bump_seed) = Pubkey::find_program_address(&[ctx.accounts.depositor.key.as_ref()], ctx.program_id);
        token::set_authority(ctx.accounts.into_set_authority_context(), AuthorityType::AccountOwner, Some(pda_account))?;
        
        ctx.accounts.vault.depositor = *ctx.accounts.depositor.key;
        ctx.accounts.vault.depositor_token_account = *ctx.accounts.depositor_token_acct.to_account_info().key;
        ctx.accounts.vault.vault_token_account = *ctx.accounts.vault_token_acct.to_account_info().key;
        ctx.accounts.vault.vault_amount = transfer_amount;
        ctx.accounts.vault.pda_account = pda_account;

        ctx.accounts.user_bank.add_to_bank(VaultDetails { 
            depositor: *ctx.accounts.depositor.key, 
            vault_pubkey: *ctx.accounts.vault.to_account_info().key, 
        });

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
    
    // pub vault_authority: Signer<'info>,
    
    #[account(init, payer=depositor, space= 8 + Vault::LEN)]
    pub vault: Account<'info, Vault>,
    
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

//Withdraew from vault
// #[derive(Accounts)]
// pub struct WithdrawFromVault<'info> {
    
//     #[account(mut)]
//     pub depositor: Signer<'info>,
    
//     pub depositor_token_acct: Account<'info, TokenAccount>,
    
//     pub vault_token_acct: Account<'info, TokenAccount>,
    
//     // pub vault_authority: Signer<'info>,
    
//     #[account(init, payer=depositor, space= 8 + Vault::LEN)]
//     pub vault: Account<'info, Vault>,
    
//     #[account(mut)]
//     pub user_bank: Account<'info, UserBank>,
    
//     pub token_program: Program<'info, Token>,
    
//     pub system_program: Program<'info, System>,
// }

// impl<'info> WithdrawFromVault<'info> {
//     fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
//         let cpi_accounts = SetAuthority {
//             account_or_mint: self.vault_token_acct.to_account_info().clone(),
//             current_authority: self.depositor.to_account_info().clone(),
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
    
    pub pda_account: Pubkey,

    pub vault_amount: u32,
}

impl Vault {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 32;
}

#[account]
// #[derive(Default)]
pub struct UserBank {
    pub depositor: Pubkey,
    pub vault_count: u8,
    pub user_vaults: [VaultDetails; 20],
}

impl UserBank {

      pub const LEN: usize = (32 * 2 * 20) //user_vaults 
      + 32 // depositor_pubkey 
      + 8; // vault_count

    fn add_to_bank (&mut self, vault_details: VaultDetails) {
       
        if self.vault_count >= 20 {
            return;
        }

        self.user_vaults[(self.vault_count as usize)] = vault_details;
        self.vault_count.checked_add(1).unwrap();
    }  
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]  
// #[zero_copy]
pub struct VaultDetails {
    pub depositor: Pubkey,
    pub vault_pubkey: Pubkey,
}