# Uniswap

A cosmwasm contract which provides liquidity between Luna and token contracts based on UniswapV1
Unlike uniswapv1, this contract provides a platform where token contract owners can freely add liquidity to the token as long as it supports Transfer{} msg and deposits minimum amounts.

# Exchange

Here is brief overview of service process after contract instantiation.

![](https://p65.f3.n0.cdn.getcloudapp.com/items/YEupeBRY/uniswap.png?v=ae1411ba3d9944d86b1094a9071ec657)

## Interface
a
### AddLiquidity

A token contract owner adds liquidity to LUNA with his token by depositing both LUNA and token to the uniswapV1 contract.
A Token is then registered in the Pair storage. Approval is required from the owner to uniswapV1 contract to execute this function. UniswapV1 contract enables token holders to liquidify to LUNA and vice versa in decentralized way.

### SwapToLuna

With a token_id and amount, a user sends token to the contract address and then gets Luna based on the ratio calculated from each reserve(e.g. LUNA/Token). Approval is required from the token holder to uniswapV1 contract to execute this function. The ratio is uploaded after the transaction.

### SwapToToken

With a token_id to receive and amount, a user sends luna to the contract and gets token from contract from the ratio which is based on each reserve(e.g. LUNA/Token). token must be registered.

## Compilation

The suggest way to build an image is this (in the root directory):

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/contracts/uniswap_v1/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.8.0 ./contracts/uniswap_v1
```

# UCW20

UCW20 has two additional functions from the current cw20.

## Interface

### SetDex

the token owner sets dex so that the token contract can verify whether sender is allowed to execute `exchange` function

### DexApprove
`TODO: remove this function when sender is contract when a contract calls another contract`
the token owner approves from dex contract to trader with certain amount for swap.

## Compilation

The suggest way to build an image is this (in the root directory):

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/contracts/maker/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.8.0 ./contracts/maker
```
