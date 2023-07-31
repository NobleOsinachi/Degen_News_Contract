use anchor_lang::prelude::*;

#[error_code]
pub enum NewsError {
    #[msg("This news didn't approved")]
    NotApprovedNews
}