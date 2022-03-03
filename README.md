# Hydra Protocol

[![anchor-build](https://github.com/hydraswap-io/hydra-protocol/actions/workflows/anchor-build.yml/badge.svg)](https://github.com/hydraswap-io/hydra-protocol/actions/workflows/anchor-build.yml)

## Overview

Hydra Protocol, formally known as [HydraSwap](https://www.hydraswap.io) is a Solana-based decentralized exchange that is
dedicated to providing users with a CEX-level trading experience. Empowered by its game-changing Hydra Market Maker (
HMM) core, it is a high-performance AMM focused on maximizing the returns for liquidity providers. Our vision is to
attract more liquidity into decentralized exchanges and bring the trading experience of the average DeFi user to the
same level as centralized exchanges.

## Note

- Hydra Protocol is in active development and all API are subject to change
- This is currently experimental and unaudited software. Use at your own risk.

## Programs / Contracts

All programs can be found in `./programs` and all integration tests can be found in `./tests`

- hydra-farming
- hydra-multisig
- hydra-pools
- hydra-staking
- hydra-vesting

## Contributing

### Install dependencies

#### Installing rust:

Direction can be found [here](https://www.rust-lang.org/tools/install)

```
$ rustc --version
rustc 1.58.0 (02072b482 2022-01-11)
```

#### Installing solana cli tools:

Directions can be found [here](https://docs.solana.com/cli/install-solana-cli-tools)

```
$ solana --version
solana-cli 1.9.6 (src:781609b2; feat:2191737503)
```

#### Installing NodeJs

Direction can be found [here](https://nodejs.org/en/)

```
$ node --version
v17.1.0
```

#### Installing Anchor:

Directions can be found [here](https://project-serum.github.io/anchor/getting-started/installation.html).

You can also use our own fork by running `make install_anchor`

```
$ anchor --version
anchor-cli 0.22.0
```

### Build

`anchor build`

### Build TypeScript components

To build the javascript components you need to have built and deployed anchor to a local `solana-test-validator` so that
your IDLs contain programIds.

After `anchor deploy` you can run:

`yarn build`

We would like to make this experience more seamless.

#### Dependencies

wasm-pack via:

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install wasm-bindgen-cli
```

### Deploy

`anchor deploy`

### Test

`anchor test`

### Migrate

`make migrate`

### How tos

- [How to create a wasm package](./docs/how_to_wasm.md)
