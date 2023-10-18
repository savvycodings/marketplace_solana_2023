use anchor_lang::prelude::*;

declare_id!("4pnzxTjn7VxS2fBj3H3ESzxrRhrAdzu79TwQ73LnxgW6");

#[program]
pub mod marketplace_solana_2023 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _name: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
}

#[account]
pub struct Marketplace {
    admin: Pubkey,
    fee: u16,
}

#[account]
pub struct Collection {
    name: String,
    fee: u16,
}

#[account]
pub struct Listing {
    owner: Pubkey,
    mint: Pubkey,
    price: u64,
    exp: u64,
}
