use crate::{
    errors::MarketplaceError, state::Listing, state::Marketplace, state::Whitelist, validate_nft,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount},
    token::{transfer as spl_transfer, Mint, Token, TokenAccount, Transfer as SplTransfer},
};
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct List<'info> {
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
        init_if_needed,
        payer = maker,
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
        init,
        payer = maker,
        space = Listing::LEN,
        seeds = [whitelist.key().as_ref(), maker_mint.key().as_ref()],
        bump
    )]
    listing: Account<'info, Listing>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    metadata: Account<'info, MetadataAccount>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> List<'info> {
    pub fn create_listing(&mut self, bumps: &BTreeMap<String, u8>, price: u64) -> Result<()> {
        validate_nft!(self.metadata.collection, self.collection_mint);
        self.listing.maker = self.maker.key();
        self.listing.mint = self.maker_mint.key();
        self.listing.price = price;
        self.listing.bump = *bumps.get("listing").ok_or(MarketplaceError::BumpError)?;
        self.listing.auth_bump = *bumps.get("vault").ok_or(MarketplaceError::BumpError)?;
        Ok(())
    }

    pub fn deposit_nft(&self) -> Result<()> {
        let accounts = SplTransfer {
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        spl_transfer(ctx, 1)
    }
}
