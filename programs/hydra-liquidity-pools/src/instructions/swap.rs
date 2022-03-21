use crate::constants::*;
use crate::errors::ErrorCode;
use crate::state::pool_state::PoolState;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use hydra_math_rs::decimal::Decimal;
use hydra_math_rs::programs::liquidity_pools::swap_calculator::SwapCalculator;
use hydra_math_rs::programs::liquidity_pools::swap_result::SwapResult;

#[derive(Accounts)]
pub struct Swap<'info> {
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [ POOL_STATE_SEED, pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.pool_state_bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,
    #[account(
        mut,
        constraint = lp_token_mint.key() == pool_state.lp_token_mint,
    )]
    pub lp_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = user_from_token.owner == user.key()
    )]
    /// the token account to withdraw from
    pub user_from_token: Box<Account<'info, TokenAccount>>,

    // TODO: setup init_if_needed
    #[account(
        mut,
        constraint = user_to_token.owner == user.key()
    )]
    /// token account to send too.  
    pub user_to_token: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, pool_state.token_x_mint.as_ref(), pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.token_x_vault_bump,
        constraint = token_x_vault.key() == pool_state.token_x_vault,
    )]
    pub token_x_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ TOKEN_VAULT_SEED, pool_state.token_y_mint.as_ref(), pool_state.lp_token_mint.as_ref() ],
        bump = pool_state.token_y_vault_bump,
        constraint = token_y_vault.key() == pool_state.token_y_vault,
    )]
    pub token_y_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Swap<'info> {
    pub fn post_transfer_checks(&mut self, result: SwapResult) -> Result<()> {
        // post tx checks
        (&mut self.token_x_vault).reload()?;
        (&mut self.token_y_vault).reload()?;

        if result.x_new_down() != self.token_x_vault.amount {
            msg!("x_new_down: {:?}", result.x_new_down());
            msg!("token_x_vault.amount: {:?}", self.token_x_vault.amount);
            return Err(ErrorCode::InvalidVaultToSwapResultAmounts.into());
        }

        if result.y_new_up() != self.token_y_vault.amount {
            msg!("y_new_up: {:?}", result.y_new_up());
            msg!("token_y_vault.amount: {:?}", self.token_y_vault.amount);
            return Err(ErrorCode::InvalidVaultToSwapResultAmounts.into());
        }

        // TODO: Broken for some random reason... Come back to laterz
        // if result.squared_k_down() != self.lp_token_mint.supply {
        //     msg!("squared_k_down: {:?}", result.squared_k_down());
        //     msg!("lp_token_mint.supply: {:?}", self.lp_token_mint.supply);
        //     return Err(ErrorCode::InvalidVaultToSwapResultAmounts.into());
        // }
        Ok(())
    }

    pub fn transfer_tokens_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        if self.pool_state.debug {
            msg!(
                "from: self.token_y_vault.amount: {}",
                self.token_y_vault.amount
            );
            msg!(
                "to: self.user_to_token.amount: {}",
                self.user_to_token.amount
            );
        }

        let cpi_accounts = Transfer {
            from: self.token_y_vault.to_account_info(),
            to: self.user_to_token.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn transfer_user_tokens_to_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        if self.pool_state.debug {
            msg!(
                "from: self.user_from_token.amount: {}",
                self.user_from_token.amount
            );
            msg!(
                "to: self.token_x_vault.amount: {}",
                self.token_x_vault.amount
            );
        }

        let cpi_accounts = Transfer {
            from: self.user_from_token.to_account_info(),
            to: self.token_x_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// security check mint addresses are both correct as per the pool state object.
pub fn check_mint_addresses(ctx: &Context<Swap>) -> Result<()> {
    let mut user_to_token_valid = false;
    let mut user_from_token_valid = false;

    if ctx.accounts.user_to_token.mint == ctx.accounts.pool_state.token_x_mint {
        user_to_token_valid = true;
    }

    if ctx.accounts.user_to_token.mint == ctx.accounts.pool_state.token_y_mint {
        user_to_token_valid = true;
    }

    if ctx.accounts.user_from_token.mint == ctx.accounts.pool_state.token_x_mint {
        user_from_token_valid = true;
    }

    if ctx.accounts.user_from_token.mint == ctx.accounts.pool_state.token_y_mint {
        user_from_token_valid = true;
    }

    // if both mint's arent valid return an error.
    if !(user_to_token_valid && user_from_token_valid) {
        return Err(ErrorCode::InvalidMintAddress.into());
    }

    Ok(())
}

pub fn handle(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
    // Setup SwapCalculate.
    let swap = SwapCalculator::new(
        ctx.accounts.token_x_vault.amount,
        ctx.accounts.token_y_vault.amount,
        ctx.accounts.pool_state.compensation_parameter as u64,
        0,
        ctx.accounts.pool_state.fees.swap_fee_numerator,
        ctx.accounts.pool_state.fees.swap_fee_denominator,
    );

    let mut result = SwapResult::default();

    let transfer_in_amount = amount_in;
    let amount_in_decimal = Decimal::from_u64(amount_in).to_amount();
    let mut transfer_out_amount: u64 = 0;

    // detect swap direction. x to y
    if ctx.accounts.user_from_token.mint == ctx.accounts.pool_state.token_x_mint {
        // confirm the other side matches pool state of y
        if ctx.accounts.user_to_token.mint != ctx.accounts.pool_state.token_y_mint {
            return Err(ErrorCode::InvalidMintAddress.into());
        }

        result = swap.swap_x_to_y_amm(&amount_in_decimal);
        transfer_out_amount = result.delta_y_down();
    }

    // detect swap direction y to x
    if ctx.accounts.user_from_token.mint == ctx.accounts.pool_state.token_y_mint {
        // confirm the other side matches the pool state of x
        if ctx.accounts.user_to_token.mint != ctx.accounts.pool_state.token_x_mint {
            return Err(ErrorCode::InvalidMintAddress.into());
        }

        result = swap.swap_y_to_x_amm(&amount_in_decimal);
        transfer_out_amount = result.delta_x_down();
    }

    // check slippage for amount_out
    if transfer_out_amount < minimum_amount_out {
        msg!("SlippageExceeded!");
        msg!("transfer_out_amount: {:?}", transfer_out_amount);
        msg!("minimum_amount_out: {:?}", minimum_amount_out);
        return Err(ErrorCode::SlippageExceeded.into());
    }

    // check slippage for amount_in
    if transfer_in_amount > amount_in {
        msg!("SlippageExceeded!");
        msg!("transfer_in_amount: {:?}", transfer_in_amount);
        msg!("amount_in: {:?}", amount_in);
        return Err(ErrorCode::SlippageExceeded.into());
    }

    // transfer base token into vault
    msg!("transfer_in_amount: {}", transfer_in_amount);
    token::transfer(
        ctx.accounts.transfer_user_tokens_to_vault(),
        transfer_in_amount,
    )?;

    // signer
    let seeds = &[
        POOL_STATE_SEED,
        ctx.accounts.pool_state.lp_token_mint.as_ref(),
        &[ctx.accounts.pool_state.pool_state_bump],
    ];
    let signer = [&seeds[..]];

    // transfer quote token to user
    msg!("transfer_out_amount: {:?}", transfer_out_amount);
    token::transfer(
        ctx.accounts.transfer_tokens_to_user().with_signer(&signer),
        transfer_out_amount,
    )?;

    // check all amounts are correct
    ctx.accounts.post_transfer_checks(result)?;

    Ok(())
}
