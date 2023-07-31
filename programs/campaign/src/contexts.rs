use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::account::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(campaign_id: u64)]
pub struct CreateCampaignContext<'info> {
  #[account(mut)]
  pub advertiser: Signer<'info>,
  #[account(init, seeds = [
    POOL_SEED.as_bytes(), 
    &campaign_id.to_le_bytes(), 
    advertiser.key().as_ref()], 
    bump, 
    payer = advertiser, 
    space = size_of::<Pool>() + 8
  )]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EditCampaignContext<'info> {
  #[account(mut)]
  pub advertiser: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteCampaignContext<'info> {
  #[account(mut)]
  pub advertiser: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ApproveCampaignContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>
}

#[derive(Accounts)]
pub struct DenyCampaignContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  pub admin: Signer<'info>,
  /// CHECK: it's not dangerous
  #[account(mut)]
  pub advertiser AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>,
}