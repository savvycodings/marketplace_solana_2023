use crate::{state::Listing, state::Marketplace, state::Whitelist};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer as spl_transfer, Mint, Token, TokenAccount, Transfer as SplTransfer},
};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint
    )]
    maker_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"auth", maker_mint.key().as_ref()],
        bump,
        token::authority = vault,
        token::mint = maker_mint
    )]
    vault: Account<'info, TokenAccount>,
    maker_mint: Account<'info, Mint>,
    collection_mint: Account<'info, Mint>,
    #[account(
        seeds = [marketplace.key().as_ref(), collection_mint.key().as_ref()],
        bump = whitelist.bump
    )]
    whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        close = maker,
        seeds = [whitelist.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump
    )]
    listing: Account<'info, Listing>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&self) -> Result<()> {
        let accounts = SplTransfer {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let seeds = [
            b"auth",
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.auth_bump],
        ];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        spl_transfer(ctx, 1)
    }
}
