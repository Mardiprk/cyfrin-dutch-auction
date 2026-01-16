use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, CloseAccount, Mint, Token, TokenAccount, Transfer
};

declare_id!("HYFNrjfr3Re8gtUHbL7xhcMrPf9chgWXGigGDaD1rEeM");

#[program]
pub mod dutch {
    use super::*;

    pub fn init(ctx: Context<Init>,
        start_price: u64,
        end_price: u64,
        start_time: i64,
        end_time: i64,
        sell_amount: u64
    ) -> Result<()> {
        // -- ChECKS --
        require!(ctx.accounts.sell_mint.key() != ctx.accounts.buy_mint.key(), AuctionError::SameToken);
        require!(start_price >= end_price, AuctionError::InvalidPrice);

        let clock = Clock::get()?.unix_timestamp;

        require!(clock <= start_time, AuctionError::InvalidTime);
        require!(start_time < end_time, AuctionError::InvalidTime);
        require!(sell_amount > 0, AuctionError::InvalidAmount);
        
        let cpi_accounts = Transfer{
            from: ctx.accounts.seller_sell_ata.to_account_info(),
            to: ctx.accounts.auction_sell_ata.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts
            ),
            sell_amount
        )?;

        // -- STORE STATE -- 
        let auction = &mut ctx.accounts.auction;

        auction.seller = ctx.accounts.seller.key();
        auction.sell_mint = ctx.accounts.sell_mint.key();
        auction.buy_mint = ctx.accounts.buy_mint.key();
        auction.start_price = start_price;
        auction.end_price = end_price;
        auction.start_time = start_time;
        auction.end_time = end_time;
        auction.sell_amount = sell_amount;
        auction.bump = ctx.bumps.auction;

        Ok(())

    }

    pub fn buy(
        ctx: Context<Buy>,
        max_price: u64,
    ) -> Result<()>{
        let auction = &ctx.accounts.auction;
        let now = Clock::get()?.unix_timestamp;

        require!(now >= auction.start_time, AuctionError::NotStarted);
        require!(now <= auction.end_time, AuctionError::Ended);

        // ---- Linear price calculation ----
        let elapsed = now - auction.start_time;
        let duration = auction.end_time - auction.start_time;

        let price_diff = auction.start_price - auction.end_price;
        let decay = (price_diff as i128 * elapsed as i128) / duration as i128;

        let current_price = auction.start_price - decay as u64;
        
        require!(current_price >= auction.end_price, AuctionError::InvalidPrice);
        require!(current_price <= max_price, AuctionError::PriceTooHigh);

        let total_cost = current_price.checked_mul(auction.sell_amount).ok_or(AuctionError::Overflow)?;
        
        //transfer buy token to seller
        let cpi_accounts = Transfer{
            from: ctx.accounts.buyer_buy_ata.to_account_info(),
            to: ctx.accounts.seller_buy_ata.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts
            ), 
            total_cost
        )?;

        // transfer sell token to buyer (PDA Signer)
        let seeds = &[b"auction", auction.seller.as_ref(),&[auction.bump]];

        let cpi_accounts= Transfer{
            from: ctx.accounts.auction_sell_ata.to_account_info(),
            to: ctx.accounts.buyer_sell_ata.to_account_info(),
            authority: ctx.accounts.auction.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                cpi_accounts,
                &[seeds]
            ),
            auction.sell_amount
        )?;

        // -- CLOSE PDA ACcount
        token::close_account(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount{
                    account: ctx.accounts.auction_sell_ata.to_account_info(),
                    destination: ctx.accounts.seller.to_account_info(),
                    authority: ctx.accounts.auction.to_account_info(),
                },
                &[seeds]
            )
        )?;


        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()>{

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    pub sell_mint: Account<'info, Mint>,
    pub buy_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = seller,
        space = 8 + Auction::LEN,
        seeds = [b"auction", seller.key().as_ref()],
        bump
    )]
    pub auction: Account<'info, Auction>,

    #[account(mut)]
    pub seller_sell_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        token::mint = sell_mint,
        token::authority = auction
    )]
    pub auction_sell_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"auction", seller.key().as_ref()],
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,

    #[account(mut)]
    pub seller_sell_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub seller_buy_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_buy_ata: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub buyer_sell_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub auction_sell_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [b"auction", seller.key().as_ref()],
        bump = auction.bump,
        has_one = seller
    )]
    pub auction: Account<'info, Auction>,

    #[account(mut)]
    pub seller_sell_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub auction_sell_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Auction{
    pub seller: Pubkey,
    pub sell_mint: Pubkey,
    pub buy_mint: Pubkey,
    pub start_price: u64,
    pub end_price: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub sell_amount: u64,
    pub bump: u8
}

impl Auction{
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1;
}

#[error_code]
pub enum AuctionError {
    #[msg("Sell token and buy token must be different")]
    SameToken,
    #[msg("Invalid price configuration")]
    InvalidPrice,
    #[msg("Invalid time configuration")]
    InvalidTime,
    #[msg("Auction has not started")]
    NotStarted,
    #[msg("Auction has ended")]
    Ended,
    #[msg("Sell amount must be greater than zero")]
    InvalidAmount,
    #[msg("Price exceeds max acceptable price")]
    PriceTooHigh,
    #[msg("Math overflow")]
    Overflow,
}
