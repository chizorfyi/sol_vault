use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Transfer};
use zo::{self, ZO_DEX_PID, program::ZoAbi as Zo, State, Margin, Control, Cache, cpi::accounts::{CreateMargin, Deposit, Withdraw as ZoWithdraw, CreatePerpOpenOrders, PlacePerpOrder, CancelPerpOrder, CancelAllPerpOrders} };

declare_id!("B9nAoiZPKrFy1sycYNHi4vu9acr5gztt68cMbUfV6ZWS");
#[program]
pub mod sol_vault_transfer {    
    
    use super::*;
    
    pub fn create_zo_margin (ctx: Context<CreateZoMargin>, zo_margin_nonce: u8) -> Result<()> {     
        let (_, bump) = Pubkey::find_program_address(&[b"msvault".as_ref()], ctx.program_id);        
        let seed_signature = &[&b"msvault".as_ref()[..], &[bump]];        
        zo::cpi::create_margin(ctx.accounts.into_create_zo_margin_context().with_signer(&[&seed_signature[..]]), zo_margin_nonce)?;           
        // zo::cpi::create_margin(ctx.accounts.into_create_zo_margin_context(), zo_margin_nonce)?;  
        Ok(())       
    }
    
    pub fn zo_deposit (ctx: Context<ZoDeposit>, repay_only:bool, amount: u64) -> Result<()> {    
        let (_, bump) = Pubkey::find_program_address(&[b"msvault".as_ref()], ctx.program_id);        
        let seed_signature = &[&b"msvault".as_ref()[..], &[bump]];        
        zo::cpi::deposit(ctx.accounts.into_zo_deposit_context().with_signer(&[&seed_signature[..]]), repay_only , amount)?;
        // zo::cpi::deposit(ctx.accounts.into_zo_deposit_context(), repay_only , amount)?;
        Ok(())
    }
    
    pub fn zo_withdrawal (ctx: Context<ZoWithdrawal>, allow_borrow: bool, amount: u64) -> Result<()> {
        let (_, bump) = Pubkey::find_program_address(&[b"msvault".as_ref()], ctx.program_id);        
        let seed_signature = &[&b"msvault".as_ref()[..], &[bump]];  
        zo::cpi::withdraw(ctx.accounts.into_zo_withdrawal_context().with_signer(&[&seed_signature[..]]), allow_borrow ,amount)?;
        // zo::cpi::withdraw(ctx.accounts.into_zo_withdrawal_context(), allow_borrow ,amount)?;
        Ok(())
    }
    
    pub fn create_zo_perp_order (ctx: Context<CreateZoPerpOpenOrders>) -> Result<()> {
        zo::cpi::create_perp_open_orders(ctx.accounts.into_create_zo_perp_order_context())?;
        Ok(())
    }
    
    
    pub fn place_zo_perp_order (ctx: Context<PlaceZoPerpOrder>, is_long: bool, limit_price: u64, max_base_quantity:u64, max_quote_quantity:u64, order_type: ZoOrderType, limit: u16, client_id: u64) -> Result<()> {
        zo::cpi::place_perp_order(ctx.accounts.into_place_zo_perp_order_context(), is_long, limit_price, max_base_quantity, max_quote_quantity, order_type.into(), limit, client_id)?;
        Ok(())
    }
    
    pub fn cancel_zo_perp_order (ctx: Context<CancelZoPerpOrder>, order_id: Option<u128>, is_long: Option<bool>, client_id: Option<u64>) -> Result<()> {
        zo::cpi::cancel_perp_order(ctx.accounts.into_cancel_zo_perp_order_context(), order_id, is_long, client_id)?;
        Ok(())
    }
    
    pub fn cancel_all_zo_perp_order (ctx: Context<CancelAllZoPerpOrders>, limit: u16) -> Result<()> {
        zo::cpi::cancel_all_perp_orders(ctx.accounts.into_cancel_all_zo_perp_orders_context() , limit)?;
        Ok(())
    }

    pub fn create_vault (ctx: Context<CreateVault>) -> Result<()> {
        let (pda_account, _) = Pubkey::find_program_address(&[b"msvault".as_ref()], ctx.program_id);
        let vault = &mut ctx.accounts.vault;
        
        vault.depositor = *ctx.accounts.depositor.key;
        vault.depositor_token_account = *ctx.accounts.depositor_token_acct.to_account_info().key;
        vault.vault_token_account = *ctx.accounts.vault_token_acct.to_account_info().key;
        vault.vault_amount = 0;
        vault.pda_account = pda_account;
        Ok(())        
    }

    pub fn deposit_to_vault (ctx: Context<DepositToVault>, transfer_amount: u64) -> Result<()> {                
        token::transfer(ctx.accounts.into_deposit_to_vault_context(), transfer_amount)?;        
        ctx.accounts.vault.vault_amount = ctx.accounts.vault.add_to_vault(transfer_amount);
        Ok(())
    }
    
    pub fn withdraw_from_vault (ctx: Context<WithdrawFromVault>, transfer_amount: u64) -> Result<()> {        
        let (_, bump) = Pubkey::find_program_address(&[b"msvault".as_ref()], ctx.program_id);        
        let seed_signature = &[&b"msvault".as_ref()[..], &[bump]];        

        token::transfer(ctx.accounts.into_withdraw_from_vault_context().with_signer(&[&seed_signature[..]]), transfer_amount)?;                
        ctx.accounts.vault.vault_amount = ctx.accounts.vault.sub_from_vault(transfer_amount);
        Ok(())
    }



}


#[derive(Accounts)]
#[instruction(zo_margin_nonce: u8)]
pub struct CreateZoMargin<'info> {
    
    ///CHECK: pda signer
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    ///CHECK: uninitialized
    #[account(mut)]
    pub zo_program_state: AccountInfo<'info>,
    // pub zo_program_state: AccountLoader<'info, State>,
    
    ///CHECK: uninitialized
    #[account(mut)]
    pub zo_margin: UncheckedAccount<'info>,
    
    pub zo_program: Program<'info, Zo>,
    
    ///CHECK: uninitialized
    #[account(zero)]
    pub control: UncheckedAccount<'info>,
    
    ///CHECK: uninitialized
    pub rent: AccountInfo<'info>,
    // pub rent: Sysvar<'info, Rent>,
    
    ///CHECK: uninitialized
    pub system_program: AccountInfo<'info>,
    // pub system_program: Program<'info, System>,
}

impl <'info> CreateZoMargin <'info> {
    fn into_create_zo_margin_context(&self) -> CpiContext<'_, '_, '_, 'info, CreateMargin<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts = zo::cpi::accounts::CreateMargin {
            state: self.zo_program_state.to_account_info(),
            payer: self.payer.to_account_info(),
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
    
    ///CHECK: pda signer
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,
    
    pub zo_program_state: AccountLoader<'info, State>,
    
    #[account(mut)]
    pub zo_program_margin: AccountLoader<'info, Margin>,
    
    pub zo_program: Program<'info, Zo>,
    
    ///CHECK: State Signer
    pub state_signer: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    
    // #[account(mut)]
    #[account(mut, constraint = token_account.owner == *authority.to_account_info().key)]
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
    ///CHECK: pda signer
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,
    
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
    
    #[account(mut, constraint = token_account.owner == *authority.to_account_info().key)]
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

#[derive(Accounts)]
pub struct CreateZoPerpOpenOrders<'info> {
    
    pub state: AccountLoader<'info, State>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
        
    ///CHECK: unchecked
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    pub dex_program: UncheckedAccount<'info>,
    
    pub rent: Sysvar<'info, Rent>,

    pub zo_program: Program<'info, Zo>,
    
    pub system_program: Program<'info, System>,
}

impl <'info> CreateZoPerpOpenOrders <'info> {
    fn into_create_zo_perp_order_context(&self) -> CpiContext<'_, '_, '_, 'info, CreatePerpOpenOrders<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts =  CreatePerpOpenOrders {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            control: self.control.to_account_info(),
            authority: self.authority.to_account_info(),
            payer: self.payer.to_account_info(),
            margin: self.margin.to_account_info(),
            rent: self.rent.to_account_info(),
            open_orders: self.open_orders.to_account_info(),
            system_program: self.system_program.to_account_info(),
            dex_market: self.dex_market.to_account_info(),
            dex_program: self.dex_program.to_account_info(),
        };
        
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct PlaceZoPerpOrder<'info> {
    
    ///CHECK: unchecked
    pub state: AccountInfo<'info>,
    #[account(mut)]
    
    ///CHECK: unchecked
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    
    ///CHECK: unchecked
    pub cache: AccountInfo<'info>,
    
    pub authority: Signer<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub control: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    
    ///CHECK: unchecked
    #[account(address = ZO_DEX_PID)]
    pub dex_program: AccountInfo<'info>,

    pub zo_program: Program<'info, Zo>,
    
    pub rent: Sysvar<'info, Rent>,
}

impl <'info> PlaceZoPerpOrder <'info> {
    fn into_place_zo_perp_order_context(&self) -> CpiContext<'_, '_, '_, 'info, PlacePerpOrder<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts =  PlacePerpOrder {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            control: self.control.to_account_info(),
            authority: self.authority.to_account_info(),
            margin: self.margin.to_account_info(),
            rent: self.rent.to_account_info(),
            open_orders: self.open_orders.to_account_info(),
            dex_market: self.dex_market.to_account_info(),
            dex_program: self.dex_program.to_account_info(),
            cache: self.cache.to_account_info(),
            req_q: self.req_q.to_account_info(),
            event_q: self.event_q.to_account_info(),
            market_bids: self.market_bids.to_account_info(),
            market_asks: self.market_asks.to_account_info(),
        };
        
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


#[derive(Accounts)]
pub struct CancelZoPerpOrder<'info> {
    
    pub state: AccountLoader<'info, State>,
    
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub event_q: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    pub dex_program: UncheckedAccount<'info>,
    
    pub zo_program: Program<'info, Zo>,
}

impl <'info> CancelZoPerpOrder <'info> {
    fn into_cancel_zo_perp_order_context(&self) -> CpiContext<'_, '_, '_, 'info, CancelPerpOrder<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts =  CancelPerpOrder {
            state: self.state.to_account_info(),
            control: self.control.to_account_info(),
            authority: self.authority.to_account_info(),
            margin: self.margin.to_account_info(),
            open_orders: self.open_orders.to_account_info(),
            dex_market: self.dex_market.to_account_info(),
            dex_program: self.dex_program.to_account_info(),
            cache: self.cache.to_account_info(),
            event_q: self.event_q.to_account_info(),
            market_bids: self.market_bids.to_account_info(),
            market_asks: self.market_asks.to_account_info(),
        };
        
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


#[derive(Accounts)]
pub struct CancelAllZoPerpOrders<'info> {
    
    pub authority: Signer<'info>,
    
    pub state: AccountLoader<'info, State>,
    
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub req_q: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub event_q: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    
    ///CHECK: unchecked
    pub dex_program: UncheckedAccount<'info>,
    
    pub zo_program: Program<'info, Zo>,
}

impl <'info> CancelAllZoPerpOrders <'info> {
    fn into_cancel_all_zo_perp_orders_context(&self) -> CpiContext<'_, '_, '_, 'info, CancelAllPerpOrders<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts =  CancelAllPerpOrders {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            control: self.control.to_account_info(),
            authority: self.authority.to_account_info(),
            margin: self.margin.to_account_info(),
            open_orders: self.open_orders.to_account_info(),
            dex_market: self.dex_market.to_account_info(),
            dex_program: self.dex_program.to_account_info(),
            cache: self.cache.to_account_info(),
            req_q: self.req_q.to_account_info(),
            event_q: self.event_q.to_account_info(),
            market_bids: self.market_bids.to_account_info(),
            market_asks: self.market_asks.to_account_info(),
        };
        
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct DepositToVault<'info> {
    
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    #[account(mut)]
    pub depositor_token_acct: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub vault_token_acct: Account<'info, TokenAccount>,
    
    #[account(
        mut, 
        constraint = vault.depositor_token_account == *depositor_token_acct.to_account_info().key, 
        constraint = vault.vault_token_account == *vault_token_acct.to_account_info().key 
    )]
    pub vault: Account<'info, Vault>,
    
    pub token_program: Program<'info, Token>,
}

impl<'info> DepositToVault<'info> {
    fn into_deposit_to_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.depositor_token_acct.to_account_info().clone(),
            to: self.vault_token_acct.to_account_info().clone(),
            authority: self.depositor.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
        

    }
}

//Withdraw from vault
#[derive(Accounts)]
#[instruction(transfer_amount: u64)]
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
    
    #[account(
        mut, 
        constraint = vault.vault_amount >= transfer_amount, 
        constraint = vault.depositor_token_account == *depositor_token_acct.to_account_info().key, 
        constraint = vault.vault_token_account == *vault_token_acct.to_account_info().key 
    )]
    pub vault: Account<'info, Vault>,
    
    pub token_program: Program<'info, Token>,
    
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

// #[instruction(bump:u8)]
#[derive(Accounts)]
pub struct CreateVault <'info> {
    
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    #[account(init, payer=depositor, seeds=[depositor.key.as_ref(), b"vault".as_ref()], bump, space= 8 + Vault::LEN)]
    pub vault: Account<'info, Vault>,
    
    
    #[account(constraint = vault_token_acct.mint == Vault::zo_devnet_usdc_mint())]
    pub vault_token_acct: Account<'info, TokenAccount>,
    
    #[account(constraint = depositor_token_acct.mint == Vault::zo_devnet_usdc_mint())]
    pub depositor_token_acct: Account<'info, TokenAccount>,
    
    // pub mint: Account<'info, Mint>,
        
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Vault {
    
    //the depositor public key
    pub depositor: Pubkey, 
    
    // depositor token account
    pub depositor_token_account: Pubkey, 
    
    pub vault_token_account: Pubkey, 
    
    pub pda_account: Pubkey,

    pub vault_amount: u64,
}

impl Vault {
    pub const LEN: usize = 64 + 32 + 32 + 32 + 32;

    pub fn add_to_vault (&mut self, amount: u64) -> u64 {
        self.vault_amount.checked_add(amount).unwrap()
    }
    
    pub fn sub_from_vault (&mut self, amount: u64) -> u64 {
        self.vault_amount.checked_sub(amount).unwrap()
    }

    pub fn zo_devnet_usdc_mint () -> Pubkey {
        Pubkey::from_str("7UT1javY6X1M9R2UrPGrwcZ78SX3huaXyETff5hm5YdX").unwrap()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub enum ZoOrderType {
    Limit,
    ImmediateOrCancel,
    PostOnly,
    ReduceOnlyIoc,
    ReduceOnlyLimit,
    FillOrKill,
}

impl From<ZoOrderType> for zo::OrderType {
    fn from(x: ZoOrderType) -> Self {
        match x {
            ZoOrderType::Limit => Self::Limit,
            ZoOrderType::ImmediateOrCancel => Self::ImmediateOrCancel,
            ZoOrderType::PostOnly => Self::PostOnly,
            ZoOrderType::ReduceOnlyIoc => Self::ReduceOnlyIoc,
            ZoOrderType::ReduceOnlyLimit => Self::ReduceOnlyLimit,
            ZoOrderType::FillOrKill => Self::FillOrKill,
            
        }
    }
}
