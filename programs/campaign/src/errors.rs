use anchor_lang::prelude::*;

#[error_code]
pub enum CampaignError {
    #[msg("This news didn't approved")]
    NotApprovedCampaign
}