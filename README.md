# oct-cli-rs

This project aims to provide an automated tool to deal with daily maintenance tasks of octopus network.

# Usage
You can run oct-cli in command and finish some functions interactively, eg:

![](./docs/example.gif)


## Anchor upgrade

```shell
USAGE:
    oct-cli anchor-upgrade [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    mainnet    Provide data for the server https://rpc.mainnet.near.org
    testnet    Provide data for the server https://rpc.testnet.near.org

```
**example**
1. How to upgrade anchor contract from v2.1.0 to v2.2.0 in account: anchorxsb.testnet:
```shell
./oct-cli anchor-upgrade testnet select-private manual-select-accounts --account-ids anchorxsb.testnet upgrade /Users/xushenbao/project/blockchian/octopus/oct-cli-rs/res/appchain_anchor_v2.0.0.wasm new '{"appchain_id": "appchain_id", "appchain_registry": "appchain_registry", "oct_token": "oct_token"}'
```

