use crate::state::pool_state::PoolState;
use crate::utils::pyth::PythErrors::{
    InvalidAccount, InvalidAccountType, InvalidAccountVersion, InvalidMagicNumber,
    InvalidPriceAccount, PriceAccountMarkedInvalid,
};
use anchor_lang::prelude::*;
use pyth_client::PriceConf;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug)]
pub struct PythSettings {
    pub pyth_product_account: Pubkey,
    pub pyth_price_account: Pubkey,
    pub last_known_price: i64, // used to store the price as pyth can sometime return a None on: price_account.get_current_price() calls lacking enough valid publishes on a time slot.
    pub price_exponent: i32,
}

#[error_code]
pub enum PythErrors {
    #[msg("Pyth product account provided has an invalid MAGIC number")]
    InvalidMagicNumber,

    #[msg("Pyth product account is invalid")]
    InvalidAccount,

    #[msg("Pyth product account provided has an invalid account type")]
    InvalidAccountType,

    #[msg("Pyth product account provided has a different version than the Pyth client")]
    InvalidAccountVersion,

    #[msg("Pyth product price account is marked as invalid")]
    PriceAccountMarkedInvalid,

    #[msg("Pyth price account does not match the Pyth price account provided")]
    InvalidPriceAccount,
}

/// This function checks that the pyth product and pyth price account are a match so one can't spoof the price account
/// and therefore trick the hmm price oracle input.
pub fn pyth_accounts_security_check(
    remaining_accounts: &[AccountInfo],
) -> Result<Option<PythSettings>> {
    // checks the options pyth product and price accounts [0,1] have been passed into the contract
    if remaining_accounts.len() == 2 {
        let remaining_accounts = remaining_accounts.to_vec();
        let pyth_product_account = &remaining_accounts[0];
        let pyth_price_account = &remaining_accounts[1];

        // load product account
        let pyth_product_data = &pyth_product_account.try_borrow_data()?;
        let product_account =
            pyth_client::load_product(pyth_product_data).map_err(|_| InvalidAccount)?;

        // validate product account checks

        // pyth product account magic number check
        if product_account.magic != pyth_client::MAGIC {
            return Err(InvalidMagicNumber.into());
        }

        // pyth product account type check
        if product_account.atype != pyth_client::AccountType::Product as u32 {
            return Err(InvalidAccountType.into());
        }

        // pyth product account version check
        if product_account.ver != pyth_client::VERSION_2 {
            return Err(InvalidAccountVersion.into());
        }

        if !product_account.px_acc.is_valid() {
            return Err(PriceAccountMarkedInvalid.into());
        }

        let pyth_price_pubkey = Pubkey::new(&product_account.px_acc.val);
        if &pyth_price_pubkey != pyth_price_account.key {
            return Err(InvalidPriceAccount.into());
        }

        let pyth_price_data = &pyth_price_account.try_borrow_data()?;
        let price_account = pyth_client::load_price(pyth_price_data).map_err(|_| InvalidAccount)?;

        msg!("Pyth: accounts detected");
        return Ok(Some(PythSettings {
            pyth_product_account: pyth_product_account.key(),
            pyth_price_account: pyth_price_account.key(),
            last_known_price: price_account.agg.price,
            price_exponent: price_account.expo,
        }));
    }
    msg!("Pyth: no accounts detected");
    Ok(None)
}

pub struct PythPrice {}

// // TODO: ????
// pub fn get_price(pyth_price_account: &AccountInfo, pool_state: PoolState) -> Result<PriceConf> {
//     let price_account_data = &pyth_price_account.try_borrow_data()?;
//     let price_account = pyth_client::load_price(price_account_data).map_err(|_| InvalidAccount)?;
//
//     if let Some(price) = price_account.get_current_price() {
//         return Ok(price);
//     }
//     msg!("Returning last_known_price");
//     Ok(PriceConf {
//         price: &pool_state.pyth.unwrap().last_known_price.clone(),
//         conf: 0,
//         expo: &pool_state.pyth.unwrap().price_exponent.clone(),
//     })
// }

pub fn get_and_update_price(
    pyth_price_account: &AccountInfo,
    pool_state: PoolState,
) -> Result<u64> {
    Ok(0)
}
