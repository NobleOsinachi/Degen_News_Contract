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

declare_id!("CKRFPaveQYEVgcArmPWeBrnxQESUBki8LJzLcnxujA6j");

#[program]
pub mod degen_news {
    use super::*;

    use anchor_lang::AccountsClose;

    pub fn create_news(
        ctx: Context<CreateNewsContext>, 
        news_id: u64,
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_init()?;

        let a_reporter = &ctx.accounts.reporter;
        let current_time = get_current_time()?;

        a_pool.news_id = news_id;
        a_pool.created_at = current_time;
        a_pool.updated_at = current_time;
        a_pool.state = 0;
        Ok(())
    }

    pub fn edit_news(
        ctx: Context<EditNewsContext>, 
        news_id: u64, 
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_init()?;

        let a_reporter = &ctx.accounts.reporter;
        let current_time = get_current_time()?;

        a_pool.news_id = news_id;
        a_pool.updated_at = current_time;
        a_pool.state = 1;
        Ok(())
    }

    pub fn delete_news(
        ctx: Context<DeleteNewsContext>
    ) -> Result<()> {
        
        let a_pool = ctx.accounts.pool.load()?;

        let a_reporter = ctx.accounts.reporter;
        let current_time = get_current_time()?;

        let clone_news_id = a_pool.news_id;
        let (_pool, bump) = Pubkey::find_program_address(
            &[POOL_SEED.as_ref(), 
            &a_pool.news_id.to_le_bytes(), 
            a_pool.reporter.as_ref()], 
            ctx.program_id
        );

        ctx.accounts.pool.close(a_reporter.to_account_info())?;
        
        Ok(())
    }

    pub fn approve_news(
        ctx: Context<ApproveNewsContext>, 
        news_id: u64,
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_init()?;

        a_pool.state = 2;
        Ok(())
    }

    pub fn publish_news(ctx: Context<PublishNewsContext>) -> Result<()> {
        
        let mut a_pool = ctx.accounts.pool.load_mut()?;
        let system_program = &ctx.accounts.system_program;

        let current_time = get_current_time()?;

        require!(
            a_pool.state == 2, 
            DegenNewsError::NotApprovedDegenNews
        );
        
        a_pool.state = 3;
        
        let cpi_ctx = CpiContext::new(
            system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.admin.to_account_info(),
                to: ctx.accounts.reporter.to_account_info()
            }
        );

        anchor_lang::system_program::transfer(cpi_ctx, FIXED_SOL)?;

        Ok(())
    }
}
