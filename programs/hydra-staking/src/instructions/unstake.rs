use crate::constants::*;
use crate::events::*;
use crate::state::pool_state::PoolState;
use crate::utils::price::calculate_price;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Burn, Mint, Token, TokenAccount, Transfer};
use hydra_math_rs::programs::staking::hydra_staking::calculate_pool_tokens_for_withdraw;

#[derive(Accounts)]
pub struct UnStake<'info> {
    #[account(
        seeds = [ POOL_STATE_SEED, token_mint.key().as_ref(), redeemable_mint.key().as_ref() ],
        bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    #[account(
        constraint = token_mint.key() == pool_state.token_mint,
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = redeemable_mint.key() == pool_state.redeemable_mint,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = redeemable_from_authority,
        associated_token::mint = token_mint,
        associated_token::authority = redeemable_from_authority
    )]
    /// the token account to withdraw from
    pub user_to: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, token_mint.key().as_ref(), redeemable_mint.key().as_ref() ],
        bump ,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = redeemable_from.mint == pool_state.redeemable_mint,
    )]
    pub redeemable_from: Box<Account<'info, TokenAccount>>,

    /// the authority allowed to transfer from token_from
    #[account(mut)]
    pub redeemable_from_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> UnStake<'info> {
    pub fn calculate_price(&self) -> u64 {
        calculate_price(&self.token_vault, &self.redeemable_mint, &self.pool_state)
    }

    pub fn into_burn_redeemable(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            from: self.redeemable_from.to_account_info(),
            authority: self.redeemable_from_authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_from_token_vault_to_user(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.token_vault.to_account_info(),
            to: self.user_to.to_account_info(),
            authority: self.token_vault.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<UnStake>, amount: u64) -> Result<()> {
    let total_tokens = ctx.accounts.token_vault.amount;
    let total_redeemable_token_supply = ctx.accounts.redeemable_mint.supply;

    let old_price = ctx.accounts.calculate_price();
    msg!("old_price: {}", old_price);

    // burn redeemable tokens
    token::burn(ctx.accounts.into_burn_redeemable(), amount)?;

    // determine user share of vault
    let token_share =
        calculate_pool_tokens_for_withdraw(amount, total_tokens, total_redeemable_token_supply);

    let token_mint_key = ctx.accounts.pool_state.token_mint;
    let redeemable_mint_key = ctx.accounts.pool_state.redeemable_mint;
    let seeds = &[
        TOKEN_VAULT_SEED,
        token_mint_key.as_ref(),
        redeemable_mint_key.as_ref(),
        &[ctx.accounts.pool_state.token_vault_bump],
    ];
    let signer = [&seeds[..]];

    // transfer from the vault to user
    let mut cpi_tx = ctx.accounts.into_transfer_from_token_vault_to_user();
    cpi_tx.signer_seeds = &signer;
    token::transfer(cpi_tx, token_share)?;

    (&mut ctx.accounts.token_vault).reload()?;
    (&mut ctx.accounts.redeemable_mint).reload()?;

    let new_price = ctx.accounts.calculate_price();
    msg!("new_price: {}", new_price);
    emit!(PriceChange {
        old_base_per_quote_native: old_price,
        new_base_per_quote_native: new_price,
    });

    Ok(())
}
