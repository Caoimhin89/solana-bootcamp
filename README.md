* Learning Solana
A series of projects undertaken with the main of learning to write Web3 applications for the Solana blockchain.


Check current config:
```
solana config get
```

NB: The solana CLI uses mainnet-beta by default. This can be changed by running one of the following commands:
```
solana config set --url mainnet-beta
solana config set --url devnet
solana config set --url localhost
solana config set --url testnet
```

or using abbreviations:
```
solana config set -um    # For mainnet-beta
solana config set -ud    # For devnet
solana config set -ul    # For localhost
solana config set -ut    # For testnet
```

## Dev Setup
First, make sure you're connecting to the devnet

```
solana config set -ud
``

Next, request an airdrop of devnet SOL
```
solana airdrop 2
```

Check your balance
```
solana balance
```

## Run Local Validator
The Solana CLI comes with the test validator built-in. Running a local validator will allow you to deploy and tet your programs locally.

In a terminal, run the following command to start a local validator:

```
solana-test-validator
```

Make sure to update the Solana CLI config to localhost before issuing commands:

```
solana config set -ul
```# solana-bootcamp
