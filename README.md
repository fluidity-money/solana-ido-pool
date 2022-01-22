
# Fluidity Solana IDO Pool

Fluidity is a stablecoin for people who can’t afford to leave their
money idle generating interest. **Fluidity rewards users when they actually
use it**.

This codebase contains the offchain worker implementation for Fluidity.

It takes messages relayed from the upstream server operated by Fluidity
fluidity-random, scans for winning transactions and calls the contract
when a winner is seen with the transaction and a merkle proof.

The repo contains the IDO contract ** better words **  


## Environment Variables (Solana)
​
| Name                            | Description                                                                  |
|---------------------------------|------------------------------------------------------------------------------|
| `FLU_IDO_RPC_ADDR`              | From docker compose                                                          | 
| `FLU_IDO_AUTH_PATH`             | From docker compose                                                          | 
| `FLU_GOV_AMOUNT`                | From docker compose                                                          | 
| `FLU_IDO_START_TIME`            | From docker compose                                                          | 
| `FLU_IDO_DEPOSIT_TIME`          | From docker compose                                                          | 
| `FLU_IDO_IDO_TIME`              | From docker compose                                                          | 
| `FLU_IDO_ESCROW_TIME`           | From docker compose                                                          | 
| `FLU_IDO_USDC_MINT`             | From source                                                                  |
| `FLU_IDO_NAME`                  | From source                                                                  |
| `FLU_IDO_WATERMELON_MINT`       | From source                                                                  |
| `FLU_IDO_WATERMELON_AMOUNT`     | From source                                                                  |
| `FLU_IDO_START`                 | From source                                                                  |
| `FLU_IDO_END_DEPOSITS`          | From source                                                                  |
| `FLU_IDO_END_IDO`               | From source                                                                  |
| `FLU_IDO_END_ESCROW`            | From source                                                                  |
| `ANCHOR_WALLET`                 | Is this even required?                                                       |


## Prerequisites

-

## Building

- 

## Building (Docker)

- `docker build .`	
	

// this gets accessed when we connect the provider but with a much worse error message
if (!process.env[`ANCHOR_WALLET`]) throw new Error(`Env ANCHOR_WALLET not found, set it to the path of your wallet!`);

// mint account for usdc
const idoUsdcMint = mustEnv(`FLU_IDO_USDC_MINT`, pkFromString);

// mint account for gov tokens
const idoWatermelonMint = mustEnv(`FLU_IDO_WATERMELON_MINT`, pkFromString);

// account that holds gov tokens we transfer from, must be owned by our wallet
const idoWatermelonSource = mustEnv(`FLU_IDO_WATERMELON_SOURCE`, pkFromString);

const idoName = process.env[`FLU_IDO_NAME`]!;
if (!idoName) throw new Error(`Env FLU_IDO_NAME not set!`);
if (idoName.length > 10) throw new Error(`Ido name ${idoName} too long, must be <= 10 chars!`);

const idoWatermelonAmount = mustEnv(`FLU_IDO_WATERMELON_AMOUNT`, bnFromString);

const startIdo = mustEnv(`FLU_IDO_START`, bnFromString);
const endDeposits = mustEnv(`FLU_IDO_END_DEPOSITS`, bnFromString);
const endIdo = mustEnv(`FLU_IDO_END_IDO`, bnFromString);
const endEscrow = mustEnv(`FLU_IDO_END_ESCROW`, bnFromString);