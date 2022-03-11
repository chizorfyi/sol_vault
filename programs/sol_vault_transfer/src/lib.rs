use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, SetAuthority, Token, Transfer};
use spl_token::instruction::AuthorityType;
use zo::{self, program::ZoAbi as Zo, State, Margin, Control, Cache, cpi::accounts::{CreateMargin, Deposit, Withdraw as ZoWithdraw} };
// use std::collections::HashSet;

declare_id!("B9nAoiZPKrFy1sycYNHi4vu9acr5gztt68cMbUfV6ZWS");

#[program]
pub mod sol_vault_transfer {

    use super::*;
  
    pub fn create_user_bank (ctx: Context<CreateUserBank>) -> Result<()> {
        let user_bank = &mut ctx.accounts.user_bank;
        user_bank.depositor = *ctx.accounts.depositor.key;
        user_bank.vault_count = 0;
        user_bank.user_vaults = Vec::<Pubkey>::with_capacity(20);
        Ok(())
    }

    pub fn deposit_to_vault (ctx: Context<DepositToVault>, transfer_amount: u64) -> Result<()> {
        
        let (pda_account, _) = Pubkey::find_program_address(&[ctx.accounts.depositor.key.as_ref()], ctx.program_id);
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
        
        

        let (_, bump) = Pubkey::find_program_address(&[ctx.accounts.depositor.key.as_ref()], ctx.program_id);
        let seed_signature = &[&ctx.accounts.depositor.key.as_ref()[..], &[bump]];

        token::transfer(ctx.accounts.into_withdraw_from_vault_context().with_signer(&[&seed_signature[..]]), ctx.accounts.vault.vault_amount as u64 )?;
        // token::transfer(ctx.accounts.into_withdraw_from_vault_context().with_signer(&[&seed_signature[..]]), 1 as u64 )?;
        
        let user_bank = &mut ctx.accounts.user_bank;
        user_bank.remove_from_bank(ctx.accounts.vault.to_account_info().key);
        
        Ok(())

        
    }

    pub fn create_zo_margin (ctx: Context<CreateZoMargin>, zo_margin_nonce: u8) -> Result<()> {
        
        zo::cpi::create_margin(ctx.accounts.into_create_zo_margin_context(), zo_margin_nonce)?;
        
        Ok(())

    }
    
    pub fn zo_deposit (ctx: Context<ZoDeposit>, amount: u64) -> Result<()> {
        zo::cpi::deposit(ctx.accounts.into_zo_deposit_context(), false , amount)?;
        Ok(())
    }
    
    pub fn zo_withdrawal (ctx: Context<ZoWithdrawal>, amount: u64) -> Result<()> {
        zo::cpi::withdraw(ctx.accounts.into_zo_withdrawal_context(), false ,amount)?;
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
    
    #[account(mut)]
    pub depositor_token_acct: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub vault_token_acct: Account<'info, TokenAccount>,
    
    #[account(init, payer=depositor, space= 8 + Vault::LEN)]
    pub vault: Account<'info, Vault>,
    
    // pub pda_account: AccountInfo<'info>,
    
    #[account(mut)]
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
    
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    #[account(mut)]
    pub depositor_token_acct: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub vault_token_acct: Account<'info, TokenAccount>,
    
    /// CHECK: PDA Account
    #[account(mut)]
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

#[derive(Accounts)]
#[instruction(zo_margin_nonce: u8)]
pub struct CreateZoMargin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub zo_program_state: AccountLoader<'info, State>,
    
    #[account(mut)]
    pub zo_margin: AccountLoader<'info, Margin>,
    
    pub zo_program: Program<'info, Zo>,
    
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    
    pub rent: Sysvar<'info, Rent>,
    
    pub system_program: Program<'info, System>,
}

impl <'info> CreateZoMargin <'info> {
    fn into_create_zo_margin_context(&self) -> CpiContext<'_, '_, '_, 'info, CreateMargin<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts = zo::cpi::accounts::CreateMargin {
            state: self.zo_program_state.to_account_info(),
            payer: self.authority.to_account_info(),
            authority: self.authority.to_account_info(),
            margin: self.zo_margin.to_account_info(),
            control: self.control.to_account_info(),
            rent: self.rent.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ZoDeposit<'info> {
    #[account(mut)]
    
    pub authority: Signer<'info>,
    
    pub zo_program_state: AccountLoader<'info, State>,
    
    #[account(mut)]
    pub zo_program_margin: AccountLoader<'info, Margin>,
    
    pub zo_program: Program<'info, Zo>,
    
    ///CHECK: State Signer
    pub state_signer: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub zo_program_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

impl <'info> ZoDeposit <'info> {
    fn into_zo_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, Deposit<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts = Deposit {
             state: self.zo_program_state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            cache: self.cache.to_account_info(),
            authority: self.authority.to_account_info(),
            margin: self.zo_program_margin.to_account_info(),
            token_account: self.token_account.to_account_info(),
            vault: self.zo_program_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ZoWithdrawal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub zo_program_state: AccountLoader<'info, State>,
    
    #[account(mut)]
    pub zo_program_margin: AccountLoader<'info, Margin>,
    
    pub zo_program: Program<'info, Zo>,
    
    ///CHECK: State Signer
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub zo_program_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

impl <'info> ZoWithdrawal <'info> {
    fn into_zo_withdrawal_context(&self) -> CpiContext<'_, '_, '_, 'info, ZoWithdraw<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts =  ZoWithdraw {
            state: self.zo_program_state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            cache: self.cache.to_account_info(),
            control: self.control.to_account_info(),
            authority: self.authority.to_account_info(),
            margin: self.zo_program_margin.to_account_info(),
            token_account: self.token_account.to_account_info(),
            vault: self.zo_program_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        
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

    pub vault_amount: u64,
}

impl Vault {
    pub const LEN: usize = 64 + 32 + 32 + 32 + 32;
}

#[account]
// #[derive(Default)]
pub struct UserBank {
    pub depositor: Pubkey,
    pub vault_count: u8,
    // pub user_vaults: [VaultDetails; 20],
    // pub user_vaults: HashSet<VaultDetails>,
    pub user_vaults: Vec<Pubkey>,
}

impl UserBank {

      pub const LEN: usize = (32 * 20) //user_vaults 
      + 32 // depositor_pubkey 
      + 8; // vault_count

    fn add_to_bank (&mut self, vault_details: Pubkey) {
       
        if self.vault_count >= 20 {
            return;
        }

        self.user_vaults.push(vault_details) ;
        self.vault_count = self.vault_count.checked_add(1).unwrap();
    }

    fn remove_from_bank (&mut self, key: &Pubkey)  {

        let index = self.user_vaults.iter().position(|x| x == key).unwrap();
        self.user_vaults.remove(index);
        self.vault_count = self.vault_count.checked_sub(1).unwrap();
        

    }
    
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]  
// // #[zero_copy]
// pub struct VaultDetails {
//     pub depositor: Pubkey,
//     pub vault_pubkey: Pubkey,
// }

