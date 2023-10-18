use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid name")]
    InvalidName,
    #[msg("Invalid bump")]
    BumpError,
    #[msg("Collection not set")]
    CollectionNotSet,
    #[msg("Invalid collection")]
    InvalidCollection,
}
