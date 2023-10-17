use anchor_lang::prelude::*;

declare_id!("4pnzxTjn7VxS2fBj3H3ESzxrRhrAdzu79TwQ73LnxgW6");

#[program]
pub mod marketplace_solana_2023 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
