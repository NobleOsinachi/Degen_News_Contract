use anchor_lang::prelude::*;

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
        a_pool.reporter = a_reporter.to_account_info().key();
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

        let current_time = get_current_time()?;

        a_pool.news_id = news_id;
        a_pool.updated_at = current_time;
        a_pool.state = 1;
        Ok(())
    }

    pub fn delete_news(
        ctx: Context<DeleteNewsContext>
    ) -> Result<()> {
        
        let a_reporter = &ctx.accounts.reporter;
        ctx.accounts.pool.close(a_reporter.to_account_info())?;
        
        Ok(())
    }

    pub fn approve_news(
        ctx: Context<ApproveNewsContext>
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_init()?;

        a_pool.state = 2;
        Ok(())
    }

    pub fn publish_news(ctx: Context<PublishNewsContext>) -> Result<()> {
        
        let mut a_pool = ctx.accounts.pool.load_mut()?;
        let system_program = &ctx.accounts.system_program;

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
