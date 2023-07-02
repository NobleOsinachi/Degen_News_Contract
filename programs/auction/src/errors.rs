use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionError {
    #[msg("Bid price must be bigger than min price")]
    InvalidPrice,
     #[msg("Invalid Nft")]
    InvalidNft,
    #[msg("Auction had finished or not exist")]
    OutOfAuction,
    #[msg("Auction isn't finished")]
    NotFinishAuction,
    #[msg("Over max count")]
    OverMaxCount,
    #[msg("Error in crate bid")]
    CreateBidError,
    #[msg("Error in update bid")]
    UpdateBidError,
    #[msg("Error in cancel bid")]
    CancelBidError,
    #[msg("Error in claim bid")]
    ClaimBidError,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Already claimed prize")]
    AlreadyClaimedPrize,
    #[msg("Not winner")]
    NotWinner,
    #[msg("Error in claim prize")]
    ClaimPrizeError,
    #[msg("Error in set winner")]
    SetWinnerError,
    #[msg("Auction already started")]
    StartedAuction
    #[msg("Insufficient NFT")]
    InsufficientNft
}