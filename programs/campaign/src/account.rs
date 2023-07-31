use anchor_lang::prelude::*;

#[account(zero_copy)]
// #[repr(packed)]
pub struct Pool {
  pub campaign_id: u64, 
  pub advertiser: Pubkey,
  pub created_at: u32,
  pub updated_at: u32,
  pub state: u32,
}

impl Default for Pool {
  #[inline]
  fn default() -> Pool {
      Pool {
          campaign_id: 0,
          advertiser: anchor_lang::solana_program::pubkey!("3ttYrBAp5D2sTG2gaBjg8EtrZecqBQSBuFRhsqHWPYxX"),
          created_at: 0,
          updated_at: 0,
          state: 0
      }
  }
}