use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::solana_sdk::transaction::Transaction;
use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::system_program;

use crate::Config;

pub fn execute_init_tx<'a>(rpc_client: RpcClient, config: Config) -> anyhow::Result<()> {
    // println!("running pool init");
    // let program_id = hydra_liquidy_pools::ID;
    // let pool = Keypair::new();
    //
    // println!("program_id: {}", program_id);
    //
    // let ix = Instruction {
    //     program_id: program_id,
    //     accounts: hydra_liquidy_pools::accounts::Initialize {
    //         pool: pool.pubkey(),
    //         user: config.keypair.pubkey(),
    //         system_program: system_program::ID,
    //     }
    //     .to_account_metas(Some(true)),
    //     data: hydra_liquidy_pools::instruction::InitPool { data: 64 }.data(),
    // };
    //
    // let mut transaction = Transaction::new_with_payer(&[ix], Some(&config.keypair.pubkey()));
    // let blockhash = rpc_client.get_latest_blockhash()?;
    // transaction.try_sign(&[&config.keypair, &pool], blockhash)?;
    //
    // println!("JSON RPC URL: {}", config.json_rpc_url);
    //
    // let sig = rpc_client
    //     .send_and_confirm_transaction(&transaction)
    //     .unwrap();
    // println!("txhash: {}", sig);
    //
    Ok(())
}

pub fn execute_deposit_tx(_rpc_client: RpcClient, _config: Config) -> anyhow::Result<()> {
    Ok(())
}

pub fn execute_withdraw_tx(_rpc_client: RpcClient, _config: Config) -> anyhow::Result<()> {
    Ok(())
}

pub fn execute_swap_tx(_rpc_client: RpcClient, _config: Config) -> anyhow::Result<()> {
    Ok(())
}
