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

declare_id!("5ddqLma91s4LzGLBwGrmzTBptRnaVnz9dXPF1KojV6Aw");

#[program]
pub mod campaign {
    use super::*;

    use anchor_lang::AccountsClose;

    pub fn create_campaign(
        ctx: Context<CreateCampaignContext>, 
        campaign_id: u64,
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_init()?;
        let system_program = &ctx.accounts.system_program;

        let a_advertiser = &ctx.accounts.advertiser;
        let current_time = get_current_time()?;

        a_pool.campaign_id = campaign_id;
        a_pool.advertiser = a_advertiser.to_account_info().key();
        a_pool.created_at = current_time;
        a_pool.updated_at = current_time;
        a_pool.state = 0;

        let cpi_ctx = CpiContext::new(
            system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.advertiser.to_account_info(),
                to: ctx.accounts.pool.to_account_info()
            }
        );

        anchor_lang::system_program::transfer(cpi_ctx, FIXED_SOL)?;

        Ok(())
    }

    pub fn edit_campaign(
        ctx: Context<EditCampaignContext>, 
        campaign_id: u64, 
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_mut()?;

        let current_time = get_current_time()?;

        a_pool.campaign_id = campaign_id;
        a_pool.updated_at = current_time;
        a_pool.state = 1;
        Ok(())
    }

    pub fn delete_campaign(
        ctx: Context<DeleteCampaignContext>
    ) -> Result<()> {
        
        let a_advertiser = &ctx.accounts.advertiser;
        ctx.accounts.pool.close(a_advertiser.to_account_info())?;
        
        Ok(())
    }

    pub fn approve_campaign(
        ctx: Context<ApproveCampaignContext>
    ) -> Result<()> {
        {
            let mut a_pool = ctx.accounts.pool.load_mut()?;
            a_pool.state = 2;
        }
        {
            let a_pool = ctx.accounts.pool.load()?;
            let system_program = &ctx.accounts.system_program;

            let (_pool, bump) = Pubkey::find_program_address(
                &[POOL_SEED.as_ref(), 
                &a_pool.campaign_id.to_le_bytes(), 
                a_pool.advertiser.as_ref()], 
                ctx.program_id
            );
            
            let seeds = &[POOL_SEED.as_bytes(), &a_pool.campaign_id.to_le_bytes(), a_pool.advertiser.as_ref(), &[bump]];
            let signer = &[&seeds[..]];

             let cpi_ctx = CpiContext::new_with_signer (
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.pool.to_account_info(),
                    to: ctx.accounts.admin.to_account_info()
                },
                signer
            );
            anchor_lang::system_program::transfer(cpi_ctx, FIXED_SOL)?;

        }

        Ok(())
    }

    pub fn deny_campaign(
        ctx: Context<DenyCampaignContext>
    ) -> Result<()> {
        {
            let mut a_pool = ctx.accounts.pool.load_mut()?;
            a_pool.state = 3;
        }
        {
            let a_pool = ctx.accounts.pool.load()?;
            let system_program = &ctx.accounts.system_program;

            let (_pool, bump) = Pubkey::find_program_address(
                &[POOL_SEED.as_ref(), 
                &a_pool.campaign_id.to_le_bytes(), 
                a_pool.advertiser.as_ref()], 
                ctx.program_id
            );
            
            let seeds = &[POOL_SEED.as_bytes(), &a_pool.campaign_id.to_le_bytes(), a_pool.advertiser.as_ref(), &[bump]];
            let signer = &[&seeds[..]];

             let cpi_ctx = CpiContext::new_with_signer (
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.pool.to_account_info(),
                    to: ctx.accounts.advertiser.to_account_info()
                },
                signer
            );
            anchor_lang::system_program::transfer(cpi_ctx, FIXED_SOL)?;

        }

        Ok(())
    }
}
