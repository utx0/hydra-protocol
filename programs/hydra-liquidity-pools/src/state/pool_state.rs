use crate::state::fees::Fees;
use crate::utils::pyth::PythSettings;
use anchor_lang::prelude::*;
use derivative::Derivative;
use std::io::Write;

#[account]
#[derive(Default, Derivative, Debug)]
pub struct PoolState {
    pub authority: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_y_vault: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub lp_token_mint: Pubkey,
    pub pool_state_bump: u8,
    pub token_x_vault_bump: u8,
    pub token_y_vault_bump: u8,
    pub lp_token_vault_bump: u8,
    pub lp_token_mint_bump: u8,
    pub compensation_parameter: u8, // expects 0, 100, 125, or 150
    pub fees: Fees,
    pub pyth: Option<PythSettings>,
    #[derivative(Default(value = "false"))]
    pub debug: bool,
    pub reserved: PoolStateReserve,
}

impl PoolState {
    pub fn update_oracle_price(&mut self, new_price: i64, valid_slot: u64) {
        if let Some(p) = &mut self.pyth {
            p.update_price(new_price, valid_slot)
        }
    }
}

const POOL_STATE_RESERVE_SIZE: usize = 448;

#[derive(Clone, Debug)]
pub struct PoolStateReserve([u8; POOL_STATE_RESERVE_SIZE]);

impl AnchorSerialize for PoolStateReserve {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for PoolStateReserve {
    fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self([0u8; POOL_STATE_RESERVE_SIZE]))
    }
}

impl Default for PoolStateReserve {
    fn default() -> Self {
        PoolStateReserve {
            0: [0u8; POOL_STATE_RESERVE_SIZE],
        }
    }
}
