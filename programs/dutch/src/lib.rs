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
        let clock = Clock::get()?;
        let rent = Rent::get()?;

        Ok(())

    }

    pub fn buy(
        ctx: Context<Buy>,
        max_price: u64,
    ) -> Result<()>{

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
