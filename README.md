# Uniswap

A cosmwasm contract which provides liquidity between Luna and token contracts based on UniswapV1
Unlike uniswapv1, this contract provides a platform where token contract owners can freely add liquidity to the token as long as it supports Transfer{} msg and deposits minimum amounts.

# Exchange

Here is brief overview of service process after contract instantiation.

![](https://p65.f3.n0.cdn.getcloudapp.com/items/YEupeBRY/uniswap.png?v=ae1411ba3d9944d86b1094a9071ec657)

## Interface


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
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/contracts/erc20/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer:0.8.0 ./contracts/erc20
```

# Test envs

## Accounts

Account1(Mnemonic)

satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn

Account2(Mnemonic) 

illness wing believe zone chair very glove predict isolate polar bulb slush
terra1a8fz0g4r64plsww6fflqzy7at2fv96629k5fre

## Contract deployment

### Erc20
```sh
terracli tx wasm instantiate 1 '{"name":"MyTerraToken","symbol":"MTT", "decimals": 18, "initial_balances":[{"address":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8","amount":"10000000"},{"address": "terra1a8fz0g4r64plsww6fflqzy7at2fv96629k5fre", "amount" : "10000000"}]}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```

contract_address: terra1pg606jw68d9mnh9czrgm7celc3rq9x5wrvj7gl


### UniswapV1

```sh
terracli tx wasm instantiate 2 '{"minimum_luna": "0", "owner": "terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```



uniswap v1 contract address: terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4
owner address: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8


1. execute add_liquidity

terracli tx wasm execute <uniswap contract address> '{"add_liquidity": { "luna_amount": "100", "token_address": <token_contract_address>, "token_amount": "10000", "token_id": "1"}}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block 

