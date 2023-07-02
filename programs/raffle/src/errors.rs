use anchor_lang::prelude::*;

#[error_code]
pub enum RaffleError {
    #[msg("Amount should be bigger than 0")]
    InvalidAmount,
    #[msg("Invalid NFT")]
    InvalidNft,
    #[msg("Insufficient NFT")]
    InsufficientNft,
    #[msg("Raffle isn't finished")]
    NotFinishRaffle,
    #[msg("Over max count")]
    OverMaxCount,
    #[msg("Raffle had finished or not exist")]
    OutOfRaffle,
    #[msg("Alreay set winner")]
    AlreadySetWinner,
    #[msg("Error in set winner")]
    SetWinnerError,
    #[msg("Raffle already started")]
    StartedRaffle,
    #[msg("Too many ticket")]
    TooManyTicket,
    #[msg("Not winner")]
    NotWinner,
    #[msg("Error in claim prize")]
    ClaimPrizeError,
}