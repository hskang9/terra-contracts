# Exchange

A cosmwasm contract which provides liquidity between Luna and token contracts based on Uniswap
Unlike uniswapv1, this contract provides a platform where dex contract owners can add liquidity to the token as long as it supports Transfer{} msg and deposits minimum amounts.

Here is brief overview of service process after contract instantiation.

![](https://p65.f3.n0.cdn.getcloudapp.com/items/YEupeBRY/uniswap.png?v=ae1411ba3d9944d86b1094a9071ec657)

## Interface


### AddLiquidity

An Exchange contract owner adds liquidity to LUNA with a token by depositing both LUNA and the token to the dex contract.
A Token is then registered in the Pair storage. Approval is required from the sender to the dex contract for executing this function. UniswapV1 contract enables token holders to liquidify to LUNA and vice versa in decentralized way.

### SwapToLuna

With a token_id and amount, a user sends token to the contract address and then gets Luna based on the ratio calculated from each reserve(e.g. LUNA/Token). Approval is required from the token holder to uniswapV1 contract to execute this function. The ratio is uploaded after the transaction.

### SwapToToken

With a token_id to receive and amount, a user sends luna to the contract and gets token from contract from the ratio which is based on each reserve(e.g. LUNA/Token). token must be registered.

### RemoveLiquidity

A dex contract owner removes liquidity to Luna with the token to the dex contract. Both deposited reserves are returned to the exchange contract owner.

## Compilation

The suggest way to build an image is this (in the root directory):

```shell script
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

```shell script
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
```shell script
terracli tx wasm store cw_erc20.wasm --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```

```shell script
terracli tx wasm instantiate 13 '{"name":"MyTerraToken","symbol":"MTT", "decimals": 18, "initial_balances":[{"address":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8","amount":"10000000"},{"address": "terra1a8fz0g4r64plsww6fflqzy7at2fv96629k5fre", "amount" : "10000000"}]}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```

contract_address: terra17twkkz0xdr9kzcwptzs7ew8y8tw936twkhpmu8



1. test transfer
```shell script
terracli tx wasm execute terra18x4r44npdzrk0k9pzvy7h4d38ep3rmadsewzsh '{"transfer":{"amount":"100000","recipient":"terra1uq0haxdf5cgg7s76frd3rtnr0trzaazyy00jqf"}}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```

2. test balances
```shell script
terracli query wasm contract-store  terra18x4r44npdzrk0k9pzvy7h4d38ep3rmadsewzsh '{"balance":{"address":"terra1uq0haxdf5cgg7s76frd3rtnr0trzaazyy00jqf"}}'
```




### UniswapV1

```shell script
terracli tx wasm store uniswap_v1.wasm --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```

```shell script
terracli tx wasm instantiate 21 '{"minimum_luna": "0", "owner": "terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```



uniswap v1 contract address:  terra1ya352k9svdcspkw43l5kpvmg9ecatkcg764tsr
owner address: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8

0. send luna to contract 
```shell script
terracli tx send test1 terra1ya352k9svdcspkw43l5kpvmg9ecatkcg764tsr 30000uluna --chain-id=localterra
```

1. test allowance
```shell script
terracli tx wasm execute terra18x4r44npdzrk0k9pzvy7h4d38ep3rmadsewzsh '{"approve":{"amount":"100000","spender":"terra1ya352k9svdcspkw43l5kpvmg9ecatkcg764tsr"}}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block
```


2. execute add_liquidity
```shell script
terracli tx wasm execute <uniswap contract address> '{"add_liquidity": { "luna_amount": "100", "token_address": <token_contract_address>, "token_amount": "10000", "token_id": "1"}}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block 
```

terracli tx wasm execute terra1ya352k9svdcspkw43l5kpvmg9ecatkcg764tsr '{"add_liquidity": { "luna_amount": "100", "token_address": "terra18x4r44npdzrk0k9pzvy7h4d38ep3rmadsewzsh", "token_amount": "10000", "token_id": "1"}}' --from test1 --chain-id=localterra --gas=auto --broadcast-mode=block 


2. test pairs and reserves

```shell script
terracli query wasm contract-store terra1ya352k9svdcspkw43l5kpvmg9ecatkcg764tsr '{"pair":{"token_id":"1"}}'
```

3. check balance 
```shell script
terracli query account terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
```
```shell script
terracli query account terra1m5xqrvpdlrj0ntlswf0zh75887df5eq2wg2pky
```

```shell script
terracli query wasm contract-store  terra18x4r44npdzrk0k9pzvy7h4d38ep3rmadsewzsh '{"balance":{"address":"terra1l3422gmvgv4zjrp28s7fukr92wj3seqacgq75u"}}'

```