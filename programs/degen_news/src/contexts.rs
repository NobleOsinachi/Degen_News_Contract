use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Mint, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use std::mem::size_of;

use crate::account::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(news_id: u64)]
pub struct CreateNewsContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(init, seeds = [
    POOL_SEED.as_bytes(), 
    &news_id.to_le_bytes(), 
    reporter.key().as_ref()], 
    bump, 
    payer = reporer, 
    space = size_of::<Pool>() + 8
  )]
  pub pool: AccountLoader<'info, Pool>
}

#[derive(Accounts)]
pub struct EditAuctionContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>
}

#[derive(Accounts)]
pub struct DeleteNewsContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
}

#[derive(Accounts)]
pub struct AppproveNewsContext<'info> {
  #[account(mut, constraint = senior.key() == SENIOR_KEY)]
  pub senior: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>
}

#[derive(Accounts)]
pub struct PublishNewsContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub reporter: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>
}