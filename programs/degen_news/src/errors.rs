use anchor_lang::prelude::*;

#[error_code]
pub enum DegenNewsError {
    #[msg("This news didn't approved")]
    NotApprovedDegenNews
}