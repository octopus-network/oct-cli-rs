# oct-cli-rs

This project aims to provide an automated tool to deal with daily maintenance tasks of octopus network.

# Usage
You can run oct-cli in command and finish some functions interactively, eg:

![](./docs/example.gif)

After the interactive program is executed, the command line parameters will be printed, and next time you can skip the interaction and use the command line directly.

## Deploy or Upgrade

```shell
USAGE:
    oct-cli deploy-or-upgrade [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    mainnet    
    testnet 
```
**example**
1. How to upgrade anchor contract from v2.1.0 to v2.2.0 in account: anchorxsb.testnet:
```shell
oct-cli deploy-or-upgrade testnet select-rpc block-pi select-accounts manual-select-accounts --account-ids anchorxsb.testnet upgrade /Users/xushenbao/project/blockchian/octopus/oct-cli-rs/res/appchain_anchor_v2.1.0.wasm migrate_state {}
```

## Clean up states

```shell
USAGE:
    oct-cli clean-state [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    mainnet    
    testnet 
```
**example**
1. Clean up `anchorxsb.testnet`:
```shell
oct-cli clean-state testnet select-rpc block-pi select-accounts manual-select-accounts --account-ids anchorxsb.testnet clean-state y
```

## Check 

### Usage

```shell
oct-cli-check-reward 

USAGE:
    oct-cli check-reward [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    mainnet    
    testnet 
```
### Example
1. Check unprofitable validator of all appchain in mainnet:
```shell
oct-cli check-reward mainnet select-rpc block-pi input-registry-account octopus-registry.near
```
The result is like:

![img.png](docs/check_unprofitable_validator_result.png)