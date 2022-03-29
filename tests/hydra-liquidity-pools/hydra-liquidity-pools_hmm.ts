import * as anchor from "@project-serum/anchor";
import config from "config-ts/global-config.json";
import { Keypair, PublicKey } from "@solana/web3.js";
import { SOLD_MINT_AMOUNT, USDD_MINT_AMOUNT } from "../constants";
import { HydraSDK } from "hydra-ts";
import { PoolFees } from "../../sdks/hydra-ts/src/liquidity-pools/types";
import assert from "assert";

function orderKeyPairs(a: Keypair, b: Keypair) {
  if (a.publicKey.toBuffer().compare(b.publicKey.toBuffer()) > 0) {
    return [b, a];
  }

  return [a, b];
}

describe("hydra-liquidity-pool-hmm", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  let sdk: HydraSDK;

  let soldMint: PublicKey;
  let usddMint: PublicKey;
  let soldAccount: PublicKey;
  let usddAccount: PublicKey;

  let poolState: PublicKey;
  let tokenXVault: PublicKey;
  let tokenYVault: PublicKey;

  let poolStateBump: number;
  let tokenXVaultBump: number;
  let tokenYVaultBump: number;

  let poolFees: PoolFees;

  const curveType = {
    hmm: {},
  };

  let pyth_solusd_product = new PublicKey(
    "ALP8SdU9oARYVLgLR7LrqMNCYBnhtnQz1cj6bwgwQmgj"
  );
  let pyth_solusd_price = new PublicKey(
    "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG"
  );

  before(async () => {
    sdk = HydraSDK.createFromAnchorProvider(
      provider,
      config.localnet.programIds
    );

    // Keys will be ordered based on base58 encoding
    const [soldMintPair, usddMintPair] = orderKeyPairs(
      Keypair.generate(),
      Keypair.generate()
    );

    const accounts = await sdk.liquidityPools.accounts.getAccountLoaders(
      soldMintPair.publicKey,
      usddMintPair.publicKey
    );

    [soldMint, soldAccount] = await sdk.common.createMintAndAssociatedVault(
      soldMintPair,
      SOLD_MINT_AMOUNT
    );

    [usddMint, usddAccount] = await sdk.common.createMintAndAssociatedVault(
      usddMintPair,
      USDD_MINT_AMOUNT
    );

    // get the PDA for the PoolState
    poolState = await accounts.poolState.key();
    poolStateBump = await accounts.poolState.bump();
    tokenXVault = await accounts.tokenXVault.key();
    tokenXVaultBump = await accounts.tokenXVault.bump();
    tokenYVault = await accounts.tokenYVault.key();
    tokenYVaultBump = await accounts.tokenYVault.bump();
    poolFees = {
      swapFeeNumerator: 1n,
      swapFeeDenominator: 500n,
      ownerTradeFeeNumerator: 0n,
      ownerTradeFeeDenominator: 0n,
      ownerWithdrawFeeNumerator: 0n,
      ownerWithdrawFeeDenominator: 0n,
      hostFeeNumerator: 0n,
      hostFeeDenominator: 0n,
    };
  });

  it("should not initialize a liquidity-pool with invalid pyth accounts", async () => {
    try {
      await sdk.liquidityPools.initialize(
        soldMint,
        usddMint,
        poolFees,
        curveType,
        Keypair.generate().publicKey, // spoofed product account
        Keypair.generate().publicKey // spoofed price account
      );
      assert.ok(false);
    } catch (err: any) {
      let errMsg = "Pyth product account is invalid";
      assert(err.toString().includes(errMsg));
    }
  });

  it("should not initialize a liquidity-pool with invalid pyth price accounts", async () => {
    try {
      await sdk.liquidityPools.initialize(
        soldMint,
        usddMint,
        poolFees,
        curveType,
        pyth_solusd_product,
        Keypair.generate().publicKey // spoofed price account
      );
      assert.ok(false);
    } catch (err: any) {
      let errMsg =
        "Pyth price account does not match the Pyth price account provided.";
      assert(err.toString().includes(errMsg));
    }
  });
  it("should initialize a liquidity-pool with hmm/pyth integration", async () => {
    const accounts = await sdk.liquidityPools.accounts.getAccountLoaders(
      soldMint,
      usddMint
    );

    await sdk.liquidityPools.initialize(
      soldMint,
      usddMint,
      poolFees,
      curveType,
      pyth_solusd_product,
      pyth_solusd_price
    );

    const poolStateInfo = await accounts.poolState.info();
    const poolStateAccount = poolStateInfo.data;

    assert.equal(
      poolStateAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
    assert.equal(
      poolStateAccount.tokenXVault.toString(),
      tokenXVault.toString()
    );
    assert.equal(
      poolStateAccount.tokenYVault.toString(),
      tokenYVault.toString()
    );

    assert.equal(poolStateAccount.tokenXMint.toString(), soldMint.toString());
    assert.equal(poolStateAccount.tokenYMint.toString(), usddMint.toString());
    assert.equal(
      poolStateAccount.lpTokenMint.toString(),
      (await accounts.lpTokenMint.key()).toString()
    );
    assert.equal(poolStateAccount.poolStateBump, poolStateBump);
    assert.equal(poolStateAccount.tokenXVaultBump, tokenXVaultBump);
    assert.equal(poolStateAccount.tokenYVaultBump, tokenYVaultBump);
  });
});
