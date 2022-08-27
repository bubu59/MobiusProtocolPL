use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
  // discriminator + pubkey * 3 + u64 * 2 
  #[account(init, 
            payer = fundraiser,
            space = 8 + (32 * 3) + (8 * 2) + 200,
    )]
  pub fundraiser_config: Box<Account<'info, Fundraiser>>,

  #[account(mut)]
  pub fundraiser: Signer<'info>,  

  #[account(mut)]
  pub fundraiser_sol_token_account: Box<Account<'info, TokenAccount>>,

  #[account(
        init,
        seeds = [b"vault", fundraiser_config.key().as_ref()],
        bump, 
        payer = fundraiser,
        token::mint = sol_mint,
        token::authority = token_vault
  )]
  pub token_vault: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub sol_mint : Box<Account<'info, Mint>>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct JoinCampaign<'info> {
    #[account(mut)]
    pub fundraiser_config: Box<Account<'info, Fundraiser>>,

    #[account(mut)]
    pub contributor: Signer<'info>,

    // TO-CHANGE AND REMOVE AFTER:
    // Add in "player-fund" as seed too
    #[account(
        init, 
        payer = contributor,
        space = 8 + (32 * 2) + (8 * 1) + 1,
    )]
    pub contributor_config: Box<Account<'info, Contributor>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct StdContribute<'info> {

    //Init contributor program PDA
    #[account(mut)]
    pub contributor_config: Box<Account<'info, Contributor>>,

    #[account(mut)]
    pub fundraiser_config: Box<Account<'info, Fundraiser>>,

    #[account(mut)]
    pub contributor_token_account: Box<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        seeds = [b"vault", fundraiser_config.key().as_ref()],
        bump
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub sol_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub contributor: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    ///CHECK: do not read or write to this program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FundraiserWithdrawal<'info> {   
    
    #[account(mut)]
    pub fundraiser: Signer<'info>,

    #[account(mut)]
    pub fundraiser_config: Box<Account<'info, Fundraiser>>,
    
    #[account(
        mut,
        seeds = [b"vault", fundraiser_config.key().as_ref()],
        bump
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub sol_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub fundraiser_sol_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    ///CHECK: do not read or write to this program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct YieldContribute<'info> {
    #[account(init,
    payer = contributor, 
    space = 50,
    )]
    pub contributor_config: Box<Account<'info, Contributor>>,

    #[account(mut)]
    pub fundraiser_config: Box<Account<'info, Fundraiser>>,
    ///Source liquidity token account. [writable]
    ///                $authority can transfer $liquidity_amount.
    #[account(mut)]
    pub fundraiser: AccountInfo<'info>,
    //Destination collateral token account. [writable]
    #[account(mut)]
    pub contributor_token_account: Account<'info, TokenAccount>,
    //Reserve account. [writable]
    #[account(mut)]
    pub solend_reserve: AccountInfo<'info>,
    //Reserve liquidity supply SPL Token account. [writable]
    #[account(mut)]
    pub reserve_liqudiity: AccountInfo<'info>,
    ///Reserve collateral SPL Token mint. [writable]
    #[account(mut)]
    pub reserve_mint: AccountInfo<'info>,
    ///Lending market account.
    pub lending_market_account: AccountInfo<'info>,
    ///Derived lending market authority.
    pub lending_market_auth: AccountInfo<'info>,
    ///User transfer authority ($authority) [signer].
    #[account(mut)]
    pub contributor: Signer<'info>,
    //Clock sysvar.
    //pub clock: Clock<'info>,
    //Token program id.
    #[account(init,
        payer = contributor, 
        space = 50,)]
    pub collateral_token_vault: Box<Account<'info, TokenAccount>>,
    pub token_program: AccountInfo<'info>,
    pub token_vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct YieldWithdraw<'info> {
    #[account(init,
    payer = contributor, 
    space = 50,
    )]
    pub contributor_config: Box<Account<'info, Contributor>>,

    #[account(mut)]
    pub fundraiser_config: Box<Account<'info, Fundraiser>>,
    ///Source liquidity token account. [writable]
    ///                $authority can transfer $liquidity_amount.
    #[account(mut)]
    pub fundraiser: AccountInfo<'info>,
    //Destination collateral token account. [writable]
    #[account(mut)]
    pub contributor_token_account: Account<'info, TokenAccount>,
    //Reserve account. [writable]
    #[account(mut)]
    pub solend_reserve: AccountInfo<'info>,
    //Reserve liquidity supply SPL Token account. [writable]
    #[account(mut)]
    pub reserve_liqudiity: AccountInfo<'info>,
    ///Reserve collateral SPL Token mint. [writable]
    #[account(mut)]
    pub reserve_mint: AccountInfo<'info>,
    ///Lending market account.
    pub lending_market_account: AccountInfo<'info>,
    ///Derived lending market authority.
    pub lending_market_auth: AccountInfo<'info>,
    ///User transfer authority ($authority) [signer].
    #[account(mut)]
    pub contributor: Signer<'info>,
    //Clock sysvar.
   // pub clock: Clock<'info>,
    //Token program id.
    pub token_program: AccountInfo<'info>,
    pub token_vault: Box<Account<'info, TokenAccount>>,
    pub collateral_token_vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
}
