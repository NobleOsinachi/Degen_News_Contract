use anchor_lang::prelude::*;

use crate::constants::*;
#[account(zero_copy)]
// #[repr(packed)]
pub struct Pool {
  pub news_id: u64, 
  pub reporter: Pubkey,
  pub created_at: u32,
  pub updated_at: u32,
  pub state: u32,
}

impl Default for Pool {
  #[inline]
  fn default() -> Pool {
      Pool {
          news_id: 0,
          created_at: 0,
          updated_at: 0,
          reporter: anchor_lang::solana_program::pubkey!("3ttYrBAp5D2sTG2gaBjg8EtrZecqBQSBuFRhsqHWPYxX"),
          state: 0
      }
  }
}