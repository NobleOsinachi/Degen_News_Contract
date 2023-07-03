use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;

#[account(zero_copy)]
// #[repr(packed)]
pub struct Pool {
  pub raffle_id: u64, 
  pub mint: Pubkey,
  pub start_time: u32,
  pub end_time: u32,
  pub ticket_price: u64,
  pub buyers: [Buyer; MAX_BUYER_COUNT],
  pub count: u32,
  pub total_ticket: u32,
  pub purchased_ticket: u32,
  pub state: u32,
  pub min_nft_count: u32,
}

impl Default for Pool {
  #[inline]
  fn default() -> Pool {
      Pool {
        raffle_id: 0,
        mint: anchor_lang::solana_program::pubkey!("3ttYrBAp5D2sTG2gaBjg8EtrZecqBQSBuFRhsqHWPYxX"),
        start_time: 0,
        end_time: 0,
        ticket_price: 0,

        buyers: [
              Buyer {
                  ..Default::default()
              }; MAX_BUYER_COUNT
        ],
        count: 0,
        total_ticket: MAX_TOTAL_TICKET,
        purchased_ticket: 0,
        state: 0,
        min_nft_count: 0,
      }
  }
}

impl Pool {

  fn find_buyer(&self, buyer: Pubkey) -> usize {
    let mut index = MAX_BUYER_COUNT;
    for i in 0..self.count as usize{
      if self.buyers[i].buyer == buyer {
        index = i;
        break;
      }
    }

    index
  }

  pub fn buy_ticket(&mut self, buyer: Pubkey, amount: u32) -> Result<()> {
    let index = self.find_buyer(buyer);
    msg!("index {}", index);
    if index == MAX_BUYER_COUNT {
      self.buyers[self.count as usize] = Buyer {
        buyer,
        purchased_ticket: amount,
        is_winner: 0
      };
      self.count += 1;
      require!((self.count as usize) < MAX_BUYER_COUNT, RaffleError::OverMaxCount);
    }
    else {
      self.buyers[index].purchased_ticket += amount;
    }


    self.purchased_ticket += amount;
    Ok(())
  }

  pub fn set_winner(&mut self, random: u64) -> Result<()> {
    let rand = random.checked_rem(self.purchased_ticket as u64).unwrap() as u32;
    let mut start: u32 = 0;
    let mut winner: usize = 0;
    
    for i in 0..self.count as usize {
      start += self.buyers[i].purchased_ticket;
      if start >= 0 && start >= rand {
        winner = i;
        break;
      }
    }

    self.buyers[winner].is_winner = 1;

    Ok(())
  }

  pub fn claim_prize(&mut self, buyer: Pubkey) -> Result<bool> {
    let index = self.find_buyer(buyer);
    Ok(index < MAX_BUYER_COUNT && self.buyers[index].is_winner > 0)
  }
}
#[zero_copy]
#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct Buyer {
  pub buyer: Pubkey,
  pub purchased_ticket: u32,
  pub is_winner: u32
}