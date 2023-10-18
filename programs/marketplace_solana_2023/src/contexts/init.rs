use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{errors::MarketplaceError, state::Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Marketplace::LEN,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = rewards,
    )]
    rewards: Account<'info, Mint>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, bumps: &BTreeMap<String, u8>, name: String, fee: u16) -> Result<()> {
        require!(
            name.len() > 3 && name.len() < 33,
            MarketplaceError::InvalidName
        );
        self.marketplace.admin = self.admin.key();
        self.marketplace.fee = fee;
        self.marketplace.name = name;
        self.marketplace.bump = *bumps
            .get("marketplace")
            .ok_or(MarketplaceError::BumpError)?;
        self.marketplace.treasury_bump =
            *bumps.get("treasury").ok_or(MarketplaceError::BumpError)?;
        Ok(())
    }
}
