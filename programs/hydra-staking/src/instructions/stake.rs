use crate::constants::*;
use crate::events::*;
use crate::state::pool_state::PoolState;
use crate::utils::price::calculate_price;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, Transfer};
use hydra_math_rs::programs::staking::hydra_staking::calculate_pool_tokens_for_deposit;

#[derive(Accounts)]
pub struct Stake<'info> {
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
        mut,
        constraint = user_from.mint == pool_state.token_mint,
        constraint = user_from.owner == user_from_authority.key()
    )]
    /// the token account to withdraw from
    pub user_from: Box<Account<'info, TokenAccount>>,

    /// the authority allowed to transfer from token_from
    #[account(mut)]
    pub user_from_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, token_mint.key().as_ref(), redeemable_mint.key().as_ref() ],
        bump,
        constraint = token_vault.key() == pool_state.token_vault,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user_from_authority,
        associated_token::mint = redeemable_mint,
        associated_token::authority = user_from_authority
    )]
    pub redeemable_to: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Stake<'info> {
    pub fn calculate_price(&self) -> u64 {
        calculate_price(&self.token_vault, &self.redeemable_mint, &self.pool_state)
    }

    pub fn into_mint_redeemable(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.redeemable_to.to_account_info(),
            authority: self.token_vault.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_from_user_to_token_vault(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_from.to_account_info(),
            to: self.token_vault.to_account_info(),
            authority: self.user_from_authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let total_token_vault = ctx.accounts.token_vault.amount;
    let total_redeemable_tokens = ctx.accounts.redeemable_mint.supply;

    let old_price = ctx.accounts.calculate_price();
    msg!("old_price: {}", old_price);

    let token_mint_key = ctx.accounts.pool_state.token_mint;
    let redeemable_mint_key = ctx.accounts.pool_state.redeemable_mint;

    let seeds = &[
        TOKEN_VAULT_SEED,
        token_mint_key.as_ref(),
        redeemable_mint_key.as_ref(),
        &[ctx.accounts.pool_state.token_vault_bump],
    ];
    let signer = [&seeds[..]];

    // // On first stake.
    if total_token_vault == 0 || total_redeemable_tokens == 0 {
        let mut cpi_tx = ctx.accounts.into_mint_redeemable();
        cpi_tx.signer_seeds = &signer;
        token::mint_to(cpi_tx, amount)?;
    } else {
        let mint_redeemable_amount: u64 =
            calculate_pool_tokens_for_deposit(amount, total_token_vault, total_redeemable_tokens);

        let mut cpi_tx = ctx.accounts.into_mint_redeemable();
        cpi_tx.signer_seeds = &signer;
        token::mint_to(cpi_tx, mint_redeemable_amount)?;
    }

    // transfer the users token's to the vault
    token::transfer(
        ctx.accounts.into_transfer_from_user_to_token_vault(),
        amount,
    )?;

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
