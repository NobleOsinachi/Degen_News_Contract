use anchor_lang::prelude::*;
use anchor_spl::token::{self};

pub mod contexts;
pub mod utils;
pub mod constants;
pub mod account;
pub mod errors;

use contexts::*;
use utils::*;
use errors::*;
use constants::*;

declare_id!("2pKctoN7q3VoqWq78BKLGdwmGfzE2UHrPphqTm4D7pjH");

#[program]
pub mod auction {
    use super::*;

    use anchor_lang::AccountsClose;

    pub fn create_auction(
        ctx: Context<CreateAuctionContext>, 
        auction_id: u64, 
        start_time: u32,
        end_time: u32, 
        // min_nft_count: u32,
        min_price: u64
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_init()?;

        require!(
            start_time < end_time,
            AuctionError::StartedAuction
        );

        let a_mint = &ctx.accounts.mint;

        a_pool.auction_id = auction_id;
        a_pool.start_time = start_time;
        a_pool.end_time = end_time;
        // a_pool.min_nft_count = min_nft_count;
        a_pool.mint = a_mint.to_account_info().key();
        a_pool.min_price = min_price;
        a_pool.count = 0;
        a_pool.state = 0;
        token::transfer(ctx.accounts.transfer_context(), 1)?;
        Ok(())
    }

    pub fn edit_auction(
        ctx: Context<EditAuctionContext>, 
        start_time: u32,
        end_time: u32, 
        // min_nft_count: u32,
        min_price: u64
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_mut()?;
        let current_time = get_current_time()?;

        require!(
            start_time < end_time,
            AuctionError::StartedAuction
        );
        
        require!(
            current_time < a_pool.start_time,
            AuctionError::StartedAuction
        );

        a_pool.start_time = start_time;
        a_pool.end_time = end_time;
        // a_pool.min_nft_count = min_nft_count;
        a_pool.min_price = min_price;

        Ok(())
    }

    pub fn delete_auction(
        ctx: Context<DeleteAuctionContext>
    ) -> Result<()> {
        {
            let a_pool = ctx.accounts.pool.load()?;
            let a_mint = &ctx.accounts.mint;

            let current_time = get_current_time()?;

            require!(
                current_time < a_pool.start_time || ( a_pool.count == 0 &&  current_time > a_pool.end_time) ,
                AuctionError::StartedAuction
            );

            let clone_auction_id = a_pool.auction_id;
            let (_pool, bump) = Pubkey::find_program_address(
                &[POOL_SEED.as_ref(), 
                &a_pool.auction_id.to_le_bytes(), 
                a_pool.mint.as_ref()], 
                ctx.program_id
            );
            
            let seeds = &[POOL_SEED.as_bytes(), &clone_auction_id.to_le_bytes(), a_mint.to_account_info().key.as_ref(), &[bump]];
            let signer = &[&seeds[..]];

            token::transfer(
                ctx.accounts.transfer_context().with_signer(signer), 
                1
            )?;
        }

        {
            let a_admin = &ctx.accounts.admin;
            ctx.accounts.pool.close(a_admin.to_account_info())?;
        }
        Ok(())
    }

    pub fn create_bid(ctx: Context<CreateBidContext>, price: u64) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_mut()?;
        let a_bidder = &ctx.accounts.bidder;

        let current_time = get_current_time()?;

        require!(price >= a_pool.min_price, AuctionError::InvalidPrice);
        require!(
            current_time >= a_pool.start_time && 
            current_time <= a_pool.end_time, 
            AuctionError::OutOfAuction
        );
        require!(
            (a_pool.count as usize) < MAX_BID_COUNT, 
            AuctionError::OverMaxCount
        );

        // require!(nft_count >= a_pool.min_nft_count, AuctionError::InsufficientNft);

        token::transfer(ctx.accounts.transfer_context(), price)?;

        let result: bool = a_pool.add_bid(a_bidder.to_account_info().key(), price)?;

        require!(result == true, AuctionError::CreateBidError);

        a_pool.count += 1;
        msg!("count {}", a_pool.count);
        Ok(())
    }

    pub fn update_bid(ctx: Context<UpdateBidContext>, price: u64) -> Result<()> {
        let a_bidder = &ctx.accounts.bidder;
        
        let current_time = get_current_time()?;
        let mut old_price: u64 = 0;
        
        {
            let mut a_pool = ctx.accounts.pool.load_mut()?;
            require!(price >= a_pool.min_price, AuctionError::InvalidPrice);
            require!(
                current_time >= a_pool.start_time && 
                current_time <= a_pool.end_time, 
                AuctionError::OutOfAuction
            );
            
            old_price = a_pool.update_bid(a_bidder.to_account_info().key(), price)?;
            require!(old_price != 0, AuctionError::UpdateBidError);
            
            // require!(nft_count >= a_pool.min_nft_count, AuctionError::InsufficientNft);
        }
        {
            let a_pool = ctx.accounts.pool.load()?;
            if price > old_price {
                token::transfer(ctx.accounts.transfer_context(), price - old_price)?;
            }
            else if price < old_price {
                let (_pool, bump) = Pubkey::find_program_address(
                    &[POOL_SEED.as_ref(), 
                    &a_pool.auction_id.to_le_bytes(), 
                    a_pool.mint.key().as_ref()], 
                    ctx.program_id
                );

                let seeds = &[POOL_SEED.as_bytes(), &a_pool.auction_id.to_le_bytes(), a_pool.mint.as_ref(), &[bump]];
                let signer = &[&seeds[..]];
                token::transfer(
                    ctx.accounts.reverse_transfer_context().with_signer(signer), 
                    old_price - price
                )?;
            }
        }

        Ok(())
    }

    pub fn cancel_bid(ctx: Context<CancelBidContext>) -> Result<()> {
        let mut price: u64 = 0;
        {
            let mut a_pool = ctx.accounts.pool.load_mut()?;
            let a_bidder = &ctx.accounts.bidder;

            let current_time = get_current_time()?;

            require!(
                current_time >= a_pool.start_time && 
                current_time <= a_pool.end_time, 
                AuctionError::OutOfAuction
            );

            price = a_pool.cancel_bid(a_bidder.to_account_info().key())?;
            require!(price > 0, AuctionError::CancelBidError);
            a_pool.count -= 1;

        }
        {
            let a_pool = ctx.accounts.pool.load()?;
            let (_pool, bump) = Pubkey::find_program_address(
                &[POOL_SEED.as_ref(), 
                &a_pool.auction_id.to_le_bytes(), 
                a_pool.mint.as_ref()], 
                ctx.program_id
            );
            
            let seeds = &[POOL_SEED.as_bytes(), &a_pool.auction_id.to_le_bytes(), a_pool.mint.as_ref(), &[bump]];
            let signer = &[&seeds[..]];
    
            token::transfer(
                ctx.accounts.transfer_context().with_signer(signer), 
                price
            )?;
        }

        Ok(())
    }

    pub fn claim_bid(ctx: Context<ClaimBidContext>) -> Result<()> {
        let (mut price, mut claimed) = (0, false);
        {
            let mut a_pool = ctx.accounts.pool.load_mut()?;
            let a_bidder = &ctx.accounts.bidder;
    
            let current_time = get_current_time()?;
    
            require!(
                current_time >= a_pool.end_time, 
                AuctionError::NotFinishAuction
            );
    
            if a_pool.state == 0 {
                let result = a_pool.set_winner()?;
                require!(result, AuctionError::SetWinnerError);
                a_pool.state = 1;
            }
    
            (price, claimed) = a_pool.claim_bid(a_bidder.to_account_info().key())?;
            require!(price > 0, AuctionError::ClaimBidError);
            require!(!claimed, AuctionError::AlreadyClaimed);
        }

        {
            let a_pool = ctx.accounts.pool.load()?;
            let (_pool, bump) = Pubkey::find_program_address(
                &[POOL_SEED.as_ref(), 
                &a_pool.auction_id.to_le_bytes(), 
                a_pool.mint.as_ref()], 
                ctx.program_id
            );
            
            let seeds = &[POOL_SEED.as_bytes(), &a_pool.auction_id.to_le_bytes(), a_pool.mint.as_ref(), &[bump]];
            let signer = &[&seeds[..]];
    
            token::transfer(
                ctx.accounts.transfer_context().with_signer(signer), 
                price
            )?;
    
            Ok(())
        }
    }

    pub fn claim_prize(ctx: Context<ClaimPrizeContext>) -> Result<()> {
        {
            let mut a_pool = ctx.accounts.pool.load_mut()?;
            let a_bidder = &ctx.accounts.bidder;
    
            let current_time = get_current_time()?;
    
            require!(
                current_time >= a_pool.end_time, 
                AuctionError::NotFinishAuction
            );
    
            require!(
                a_pool.state != 2, 
                AuctionError::AlreadyClaimedPrize
            );
    
            if a_pool.state == 0 {
                let result = a_pool.set_winner()?;
                require!(result, AuctionError::SetWinnerError);
                a_pool.state = 2;
            }
    
            let result = a_pool.claim_prize(a_bidder.to_account_info().key())?;
            require!(result, AuctionError::NotWinner);
        }

        {
            let a_pool = ctx.accounts.pool.load()?;
            let (_pool, bump) = Pubkey::find_program_address(
                &[POOL_SEED.as_ref(), 
                &a_pool.auction_id.to_le_bytes(), 
                a_pool.mint.as_ref()], 
                ctx.program_id
            );
            
            let seeds = &[POOL_SEED.as_bytes(), &a_pool.auction_id.to_le_bytes(), a_pool.mint.as_ref(), &[bump]];
            let signer = &[&seeds[..]];
    
            token::transfer(
                ctx.accounts.transfer_context().with_signer(signer), 
                1
            )?;
    
        }
        Ok(())
    }
}
