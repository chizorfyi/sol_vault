use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use zo::{self, ZO_DEX_PID, program::ZoAbi as Zo, State, Margin, Control, Cache, cpi::accounts::{CreateMargin, Deposit, Withdraw as ZoWithdraw, CreatePerpOpenOrders, PlacePerpOrder, CancelPerpOrder, CancelAllPerpOrders} };

declare_id!("B9nAoiZPKrFy1sycYNHi4vu9acr5gztt68cMbUfV6ZWS");

#[program]
pub mod sol_vault_transfer {

    use super::*;

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
    
    pub fn create_zo_perp_order (ctx: Context<CreateZoPerpOpenOrders>) -> Result<()> {
        zo::cpi::create_perp_open_orders(ctx.accounts.into_create_zo_perp_order_context())?;
        Ok(())
    }
    
    
    pub fn place_zo_perp_order (ctx: Context<PlaceZoPerpOrder>, is_long: bool, limit_price: u64, max_base_quantity:u64, max_quote_quantity:u64, order_type: zo::OrderType, limit: u16, client_id: u64) -> Result<()> {
        zo::cpi::place_perp_order(ctx.accounts.into_place_zo_perp_order_context(), is_long, limit_price, max_base_quantity, max_quote_quantity, order_type, limit, client_id)?;
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


}


#[derive(Accounts)]
#[instruction(zo_margin_nonce: u8)]
pub struct CreateZoMargin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub zo_program_state: AccountLoader<'info, State>,
    
    ///CHECK: uninitialized
    #[account(mut)]
    pub zo_margin: UncheckedAccount<'info>,
    
    pub zo_program: Program<'info, Zo>,
    
    ///CHECK: uninitialized
    #[account(mut)]
    pub control: UncheckedAccount<'info>,
    
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

#[derive(Accounts)]
pub struct CreateZoPerpOpenOrders<'info> {
    
    pub state: AccountLoader<'info, State>,
    
    ///CHECK: unchecked
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    
    #[account(mut)]
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
    pub dex_program: UncheckedAccount<'info>,
    
    pub rent: Sysvar<'info, Rent>,
    
    pub system_program: Program<'info, System>,
}

impl <'info> CreateZoPerpOpenOrders <'info> {
    fn into_create_zo_perp_order_context(&self) -> CpiContext<'_, '_, '_, 'info, CreatePerpOpenOrders<'info>> {
        let cpi_program = self.dex_program.to_account_info();
        let cpi_accounts =  CreatePerpOpenOrders {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            control: self.control.to_account_info(),
            authority: self.authority.to_account_info(),
            payer: self.authority.to_account_info(),
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
    
    pub rent: Sysvar<'info, Rent>,
}

impl <'info> PlaceZoPerpOrder <'info> {
    fn into_place_zo_perp_order_context(&self) -> CpiContext<'_, '_, '_, 'info, PlacePerpOrder<'info>> {
        let cpi_program = self.dex_program.to_account_info();
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
}

impl <'info> CancelZoPerpOrder <'info> {
    fn into_cancel_zo_perp_order_context(&self) -> CpiContext<'_, '_, '_, 'info, CancelPerpOrder<'info>> {
        let cpi_program = self.dex_program.to_account_info();
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
}

impl <'info> CancelAllZoPerpOrders <'info> {
    fn into_cancel_all_zo_perp_orders_context(&self) -> CpiContext<'_, '_, '_, 'info, CancelAllPerpOrders<'info>> {
        let cpi_program = self.dex_program.to_account_info();
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
