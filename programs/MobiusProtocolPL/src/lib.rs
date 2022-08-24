use anchor_lang::prelude::*;

use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer, SetAuthority};
use spl_token::instruction::AuthorityType;

pub mod context;
use context::*;

pub mod state;

declare_id!("5izPbb651w3ZUTgNZnUpmF2bRdzmePAwz4xcnK4NNbEx");

#[program]
pub mod mobius_protocol_pl {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"authority-seed";

    pub fn create_fundraiser(
        ctx: Context<CreateCampaign>, 
        goal: u64
    ) -> Result<()> {
        let fundraiser_config = &mut ctx.accounts.fundraiser_config;
        //sets fundraiser config 
        fundraiser_config.token_vault = ctx.accounts.token_vault.key();
        fundraiser_config.fundraiser = ctx.accounts.fundraiser.key();
        fundraiser_config.sol_qty = 0;
        fundraiser_config.fundraiser_sol_token_account = ctx.accounts.fundraiser_sol_token_account.key();
        fundraiser_config.goal = goal;
        //set authority     
        let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
            &[ESCROW_PDA_SEED, ctx.accounts.fundraiser_config.to_account_info().key.as_ref()],
            ctx.program_id,
        );
        let cpi_accounts = SetAuthority {
            account_or_mint: ctx.accounts.token_vault.to_account_info().clone(),
            current_authority: ctx.accounts.fundraiser_config.to_account_info().clone(),
        };
        token::set_authority(
            CpiContext::new(ctx.accounts.token_program.clone(), cpi_accounts),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;
        Ok(())
    }

    pub fn join_fundraiser(
        ctx: Context<JoinCampaign>,  
    ) -> Result<()> {
        let contributor_config = &mut ctx.accounts.contributor_config;
        contributor_config.fundraiser_config = ctx.accounts.fundraiser_config.key();
        contributor_config.contributor = ctx.accounts.contributor.key();
        contributor_config.sol_contributions = 0;
        Ok(())
    }

    pub fn std_contribute(
        ctx: Context<StdContribute>,
        amount: u64
    ) -> Result<()> {

        let contributor_token_amount = ctx.accounts.contributor_token_account.amount;

        if contributor_token_amount > amount {
                
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.contributor_token_account.to_account_info(),
                    to: ctx.accounts.token_vault.to_account_info(),
                    authority: ctx.accounts.contributor.to_account_info(),
                },
            );

            token::transfer(cpi_ctx, amount).map_err(|err| println!("{:?}", err)).ok();
        }

        ctx.accounts.contributor_config.sol_contributions += amount;
        ctx.accounts.fundraiser_config.sol_qty += amount;
        Ok(())
    }

    pub fn fundraiser_withdrawal(
        ctx: Context<FundraiserWithdrawal>, 
        amount: u64, 
    ) -> Result<()> {

        let token_vault_qty = ctx.accounts.token_vault.amount;

        let (_vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
            &[ESCROW_PDA_SEED, ctx.accounts.fundraiser_config.to_account_info().key.as_ref()],
            ctx.program_id,
        );
        let seeds = &[&ESCROW_PDA_SEED[..], &[_vault_authority_bump]];
        let signer = [&seeds[..]];

        if token_vault_qty >= amount { 
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.fundraiser_sol_token_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
                &signer,
            );

            token::transfer(cpi_ctx, amount).map_err(|err| println!("{:?}", err)).ok();
        }    

        ctx.accounts.fundraiser_config.sol_qty -= amount;
        Ok(())
    }

}