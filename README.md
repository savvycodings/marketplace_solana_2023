# Solana Marketplace Program

The provided code represents a robust Solana program that powers a decentralized NFT marketplace. Leveraging the capabilities of Solana's blockchain infrastructure, this program facilitates secure and transparent NFT trading within the Solana network. Below, you'll find an overview of the program's core instructions.

## Table of Contents

- [Introduction]
- [Instructions]
  - [Initialize]
  - [WhitelistCollection]
  - [Purchase]
  - [List]
  - [Delist]

## Introduction

This Solana program serves as the backbone of a decentralized marketplace, enabling the seamless exchange of NFTs and SOL tokens within the Solana ecosystem. It delivers a secure and efficient environment for users to interact with NFTs while taking full advantage of Solana's high-performance blockchain technology.

### Initialize

The `Initialize` instruction is used to set up the initial parameters of the Solana Marketplace.

- `admin`: The signer responsible for initializing the marketplace.
- `marketplace`: Account for managing the marketplace state. It is initialized with a unique seed based on the provided `name`.
- `rewards`: Account representing the rewards associated with the marketplace.
- `treasury`: System account responsible for managing funds.
- `token_program`: Program for token management.
- `system_program`: Program for system-level operations.

---

### WhitelistCollection

The `WhitelistCollection` instruction is used to whitelist a collection of NFTs within the marketplace. It sets the bump value for the whitelist account.

- `admin`: The signer initiating the whitelisting.
- `marketplace`: Account representing the marketplace.
- `mint`: Account representing the NFT mint.
- `whitelist`: Whitelist account for storing whitelist information.
- `system_program`: Program for system-level operations.

---

### Purchase

The `Purchase` instruction is used to purchase an NFT from the marketplace. It handles the transfer of SOL and NFTs, as well as closing associated token accounts.

- `taker`: The signer initiating the purchase.
- `maker`: The unchecked account representing the maker.
- `marketplace`: Account representing the marketplace.
- `taker_ata`: Associated token account of the taker.
- `vault`: Vault account for the maker.
- `treasury`: System account for managing funds.
- `maker_mint`: Account representing the NFT mint.
- `collection_mint`: Account representing the collection's mint.
- `whitelist`: Whitelist account for checking permissions.
- `listing`: Mutable account for the listing.
- `associated_token_program`: Program for managing associated tokens.
- `token_program`: Program for token operations.
- `system_program`: Program for system-level operations.

---

### List

The `List` instruction is used to list an NFT in the marketplace. It handles the creation of listings, deposits, and transfers of NFTs and SOL tokens.

- `maker`: The signer initiating the listing.
- `marketplace`: Account representing the marketplace.
- `maker_ata`: Associated token account of the maker.
- `vault`: Vault account for the maker.
- `maker_mint`: Account representing the NFT mint.
- `collection_mint`: Account representing the collection's mint.
- `whitelist`: Whitelist account for checking permissions.
- `listing`: Mutable account for the listing.
- `metadata`: Metadata account for the NFT.
- `metadata_program`: Program for managing metadata.
- `associated_token_program`: Program for managing associated tokens.
- `token_program`: Program for token operations.
- `system_program`: Program for system-level operations.

---

### Delist

The `Delist` instruction allows users to delist an NFT from the marketplace. It includes actions like withdrawing NFTs, transferring funds, and closing accounts.

- `maker`: The signer initiating the delisting.
- `marketplace`: Account representing the marketplace.
- `maker_ata`: Associated token account of the maker.
- `vault`: Vault account for the maker.
- `maker_mint`: Account representing the NFT mint.
- `collection_mint`: Account representing the collection's mint.
- `whitelist`: Whitelist account for checking permissions.
- `listing`: Mutable account for the listing.
- `associated_token_program`: Program for managing associated tokens.
- `token_program`: Program for token operations.
- `system_program`: Program for system-level operations.

---
