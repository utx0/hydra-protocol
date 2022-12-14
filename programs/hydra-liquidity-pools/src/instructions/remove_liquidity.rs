use crate::constants::*;
use crate::events::liquidity_removed::LiquidityRemoved;
use crate::state::pool_state::PoolState;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Burn, Mint, Token, TokenAccount, Transfer};
use hydra_math_rs::programs::liquidity_pools::hydra_lp_tokens::*;

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(
        mut,
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
        has_one = token_x_vault,
        has_one = token_y_vault,
        has_one = lp_token_mint,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// the authority allowed to transfer token_a and token_b from the users wallet.
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = user_redeemable_lp_tokens.mint == pool_state.lp_token_mint,
        constraint = user_redeemable_lp_tokens.owner ==  user.key(),
    )]
    pub user_redeemable_lp_tokens: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_x_mint,
        associated_token::authority = user,
        constraint = user_token_x.mint == pool_state.token_x_mint,
        constraint = user_token_x.owner == user.key()
    )]
    /// the token account to send token_a's back to
    pub user_token_x: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_y_mint,
        associated_token::authority = user,
        constraint = user_token_y.mint == pool_state.token_y_mint,
        constraint = user_token_y.owner == user.key()
    )]
    ///  the token account to send token_b's back to
    pub user_token_y: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, pool_state.token_x_mint.as_ref(), pool_state.lp_token_mint.as_ref() ],
        bump,
        constraint = token_x_vault.key() == pool_state.token_x_vault,
    )]
    pub token_x_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, pool_state.token_y_mint.as_ref(), pool_state.lp_token_mint.as_ref() ],
        bump,
        constraint = token_y_vault.key() == pool_state.token_y_vault,
    )]
    pub token_y_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ LP_TOKEN_MINT_SEED, pool_state.token_x_mint.as_ref(), pool_state.token_y_mint.as_ref() ],
        bump,
        constraint = lp_token_mint.key() == pool_state.lp_token_mint,
    )]
    pub lp_token_mint: Box<Account<'info, Mint>>,

    /// token_a_mint. Eg BTC
    #[account(
        constraint = token_x_mint.key() == pool_state.token_x_mint,
    )]
    pub token_x_mint: Box<Account<'info, Mint>>,

    /// token_b_mint: Eg USDC
    #[account(
        constraint = token_y_mint.key() == pool_state.token_y_mint,
    )]
    pub token_y_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> RemoveLiquidity<'info> {
    pub fn credit_user_token_a_from_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        if self.pool_state.debug {
            msg!("Account balances before transfer...");
            msg!(
                "user_token_a_to_receive.amount: {}",
                self.user_token_x.amount
            );
            msg!("token_a_vault.amount: {}", self.token_x_vault.amount);
        }

        let cpi_accounts = Transfer {
            from: self.token_x_vault.to_account_info(),
            to: self.user_token_x.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn credit_user_token_b_from_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        if self.pool_state.debug {
            msg!("Account balances before transfer...");
            msg!(
                "user_token_b_to_receive.amount: {}",
                self.user_token_y.amount
            );
            msg!("token_b_vault.amount: {}", self.token_y_vault.amount);
        }

        let cpi_accounts = Transfer {
            from: self.token_y_vault.to_account_info(),
            to: self.user_token_y.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn burn_lp_tokens(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.lp_token_mint.to_account_info(),
            from: self.user_redeemable_lp_tokens.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn calculate_a_and_b_tokens_to_credit_from_lp_tokens(
        &self,
        lp_tokens_to_burn: u64,
    ) -> (u64, u64) {
        calculate_x_y(
            lp_tokens_to_burn,
            self.lp_token_mint.decimals,
            self.token_x_vault.amount,
            self.token_x_mint.decimals,
            self.token_y_vault.amount,
            self.token_y_mint.decimals,
            self.lp_token_mint.supply,
        )
    }
}

pub fn handle(ctx: Context<RemoveLiquidity>, lp_tokens_to_burn: u64) -> Result<()> {
    let seeds = &[
        POOL_STATE_SEED,
        ctx.accounts.pool_state.lp_token_mint.as_ref(),
        &[ctx.accounts.pool_state.pool_state_bump],
    ];
    let signer = [&seeds[..]];

    let (token_x_to_credit, token_y_to_credit) = ctx
        .accounts
        .calculate_a_and_b_tokens_to_credit_from_lp_tokens(lp_tokens_to_burn);

    if ctx.accounts.pool_state.debug {
        msg!("lp_tokens_to_burn: {}", lp_tokens_to_burn);
        msg!("token_x_to_credit: {}", token_x_to_credit);
        msg!("token_y_to_credit: {}", token_y_to_credit);
    }

    // burn lp tokens
    token::burn(ctx.accounts.burn_lp_tokens(), lp_tokens_to_burn)?;

    // transfer user_token_a to vault
    token::transfer(
        ctx.accounts
            .credit_user_token_a_from_vault()
            .with_signer(&signer),
        token_x_to_credit,
    )?;

    // transfer user_token_b to vault
    token::transfer(
        ctx.accounts
            .credit_user_token_b_from_vault()
            .with_signer(&signer),
        token_y_to_credit,
    )?;

    emit!(LiquidityRemoved {
        tokens_x_credited: token_x_to_credit,
        tokens_y_credited: token_y_to_credit,
        lp_tokens_burnt: lp_tokens_to_burn,
    });

    if ctx.accounts.pool_state.debug {
        msg!("lp_tokens_to_burn: {}", lp_tokens_to_burn);
        msg!("token_x_to_credit: {}", token_x_to_credit);
        msg!("token_y_to_credit: {}", token_y_to_credit);
    }

    Ok(())
}
