# UniswapV1

UniswapV1 implementation on cosmwasm

# Exchange

## Interface

### AddLiquidity

A token contract owner adds liquidity to LUNA with his token by depositing both LUNA and token to the uniswapV1 contract.
A Token is then registered in the Pair storage. Approval is required from the owner to uniswapV1 contract to execute this function. UniswapV1 contract enables token holders to liquidify to LUNA and vice versa in decentralized way.

### SwapToLuna

With a token_id and amount, a user sends token to the contract address and then gets Luna based on the ratio calculated from each reserve(e.g. LUNA/Token). Approval is required from the token holder to uniswapV1 contract to execute this function. The ratio is uploaded after the transaction.

### SwapToToken

With a token_id to receive and amount, a user sends luna to the contract and gets token from contract from the ratio which is based on each reserve(e.g. LUNA/Token). token must be registered.









# UCW20

UCW20 has two additional functions from the current cw20.

## Interface

### SetDex

sets dex so that the token contract can verify whether sender is allowed to execute `exchange` function

### Exchange

transfer token from exchange contract to recipient without approval after checking exchange's balance  

## Compilation

The suggest way to build an image is this (in the root directory):

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/contracts/maker/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.8.0 ./contracts/maker
```

