use anchor_lang::prelude::*;
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
    payer = reporter, 
    space = size_of::<Pool>() + 8
  )]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EditNewsContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteNewsContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ApproveNewsContext<'info> {
  #[account(mut, constraint = senior.key() == SENIOR_KEY)]
  pub senior: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PublishNewsContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  pub admin: Signer<'info>,
    /// CHECK: it's not dangerous
  #[account(mut)]
  pub reporter: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>,
}