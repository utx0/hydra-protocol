mod errors;
mod events;
mod instructions;
pub mod state;
mod utils;

use instructions::add_first_liquidity::*;
use instructions::add_liquidity::*;
use instructions::initialize::*;
use instructions::remove_liquidity::*;
use instructions::swap::mint_addresses_security_check;
use instructions::swap::*;
use state::fees::Fees;
use utils::pyth::pyth_accounts_security_check;
use utils::pyth::pyth_price_account_security_check;

use anchor_lang::prelude::*;

declare_id!("BBjT5U42SuA6FcVZEofPgjAVZahvtWzHaQ8pJHyKkC5T");

#[cfg(any(feature = "localnet", feature = "devnet", feature = "testnet"))]
pub const DEBUG_MODE: bool = true;

#[cfg(feature = "mainnet")]
pub const DEBUG_MODE: bool = false;

pub mod constants {
    pub const LP_TOKEN_VAULT_SEED: &[u8] = b"lp_token_vault_seed";
    pub const LP_TOKEN_MINT_SEED: &[u8] = b"lp_token_mint_seed";
    pub const TOKEN_VAULT_SEED: &[u8] = b"token_vault_seed";
    pub const POOL_STATE_SEED: &[u8] = b"pool_state_seed";
}

#[program]
pub mod hydra_liquidity_pools {
    use super::*;

    /// initialize a new empty pool
    #[access_control(pyth_accounts_security_check(&ctx.remaining_accounts))]
    pub fn initialize(
        ctx: Context<Initialize>,
        token_x_vault_bump: u8,
        token_y_vault_bump: u8,
        pool_state_bump: u8,
        lp_token_vault_bump: u8,
        lp_token_mint_bump: u8,
        compensation_parameter: u8,
        fees: Fees,
    ) -> Result<()> {
        instructions::initialize::handle(
            ctx,
            token_x_vault_bump,
            token_y_vault_bump,
            pool_state_bump,
            lp_token_vault_bump,
            lp_token_mint_bump,
            compensation_parameter,
            fees,
        )
    }

    /// add first liquidity deposit to new/empty pool
    pub fn add_first_liquidity(
        ctx: Context<AddFirstLiquidity>,
        token_x_to_debit: u64,
        token_y_to_debit: u64,
    ) -> Result<()> {
        instructions::add_first_liquidity::handle(ctx, token_x_to_debit, token_y_to_debit)
    }

    /// add subsequent liquidity deposit to an existing pool that has already been funded.
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        tokens_x_max_amount: u64, // slippage handling: token_a_amount * (1 + TOLERATED_SLIPPAGE) --> calculated client side
        tokens_y_max_amount: u64, // slippage handling: token_b_amount * (1 + TOLERATED_SLIPPAGE) --> calculated client side
        expected_lp_tokens: u64,
    ) -> Result<()> {
        instructions::add_liquidity::handle(
            ctx,
            tokens_x_max_amount,
            tokens_y_max_amount,
            expected_lp_tokens,
        )
    }

    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        lp_tokens_to_burn: u64, // calculate the % client side
    ) -> Result<()> {
        instructions::remove_liquidity::handle(ctx, lp_tokens_to_burn)
    }

    #[access_control(
        mint_addresses_security_check(&ctx)
        pyth_price_account_security_check(&ctx)
    )]
    pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        instructions::swap::handle(ctx, amount_in, minimum_amount_out)
    }
}
