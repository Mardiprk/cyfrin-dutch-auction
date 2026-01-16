use anchor_lang::prelude::*;

declare_id!("HYFNrjfr3Re8gtUHbL7xhcMrPf9chgWXGigGDaD1rEeM");

#[program]
pub mod dutch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {}

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