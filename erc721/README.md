# ERC721

A cosmwasm contract for non-fungible asset. Non-fungible asset are used in games like [Cryptokitties](https://www.cryptokitties.co/), [Gods Unchained](https://godsunchained.com/), and collectibles like [knownorigin](https://knownorigin.io/) 


## Interface

### Burn

A token owner burns the token with id by sending asset to ZERO_ADDRESS. 

### Mint

A token contract owner mints the nft token and sends it to a recipient. 

### Approve

A token owner gives access to a certain person to transfer his tokens. 

### SetApprovalForAll

A token owner sets his ownership for all to an operator.

### TransferFrom

Transfers nft token from the tx sender to the recipient. sender has to be one of three cases:
1. a token owner
In this case, token owner transfers his token to the recipient
2. approved
In this case, one approved from the token owner transfers the owner's token to the recipient
3. operator
In this case. operator for the owner transfers the owner's token to the recipient

### SetTokenURI

Optionally a token owner can put more information as URI for his token. For URI format, check [here](https://discuss.status.im/t/eip831-uri-format-for-ethereum-in-status/1402).

## Compilation

The suggest way to build an image is this (in the root directory):

```shell script
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/contracts/erc721/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer:0.8.0 ./contracts/erc721
```

