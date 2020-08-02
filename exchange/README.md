# Exchange

A cosmwasm contract which provides liquidity between Luna and token contracts based on Uniswap
Unlike uniswapv1, this contract provides a permissionless and tokenless platform where users can add liquidity with the channel between Luna and [the token contract](../erc20).

A liquidity provider can freely set up a liquidity channel between an erc20 token and luna. Once a channel is registered with deposited luna and erc20 token in the contract, channel_id is retrieved and the user of this smart contract can swap from luna to the erc20 token in the channel and vice versa. 

Liquidity provider takes 0.3% fee on each swap transaction and deposited assets can be retrieved when the liquidity provider sends `RemoveLiquidity` transaction to the contract. After that, the channel is disappeared, and the deposited assets in the channel is sent to the liquidity provider.




## Interface


### AddLiquidity

A liquidity provider adds liquidity to LUNA with a token by depositing both LUNA and the token to the dex contract.
A Token is then registered in the Pair storage. Approval is required from the sender to the dex contract for executing this function. The contract enables token holders to liquidify to LUNA and vice versa in decentralized way.

### SwapToLuna

With a token_id and amount arguments, a user sends token to the dex contract and then gets Luna based on the ratio calculated from each reserve(e.g. LUNA/Token). Approval is required from the token holder to the dex contract to execute this function. The ratio is updated after each transaction.

### SwapToLunaOutput

Buys maximum amount of luna with limited amount of tokens in an existing channel. 

### SwapToToken

With a token_id to receive and amount arguments, a user sends luna to the dex contract and gets token from contract from the ratio which is based on each reserve(e.g. LUNA/Token). token must be registered. The ratio is updated after each transaction.

### SwapToTokenOutput

Buys maximum amount of tokens with limited amount of luna in an existing channel.

### RemoveLiquidity

A liquidity provider removes liquidity between Luna and the token in the dex contract. Both deposited reserves are returned to the liquidity provider's account.

## Compilation

The suggest way to build an image is this (in the root directory):

```shell script
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/contracts/exchange/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.8.0 ./contracts/exchange
```

