use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock};

pub fn get_current_time() -> Result<u32> {
  let clock = clock::Clock::get().unwrap();
  Ok(clock.unix_timestamp as u32)
}
