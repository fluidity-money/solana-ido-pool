
# Fluidity Solana IDO Pool

This repo contains contracts that implement an on-chain KPI auction, in which users purchase options that are then redeemable for tokens at a rate determined by how well KPI targets are being met.

The auction comprises of three main phases. In the first phase, users are able to buy or sell options 1-1 with USDc. In the second phase, users can exchange options back for USDc, but are unable to purchase any more. In the third, which is the main phase of the auction, users can exchange their options for governance tokens, at a variable rate determined by how well the program KPIs are being met.

Most of this code is forked from the [Blockworks Foundation ido-pool](https://github.com/blockworks-foundation/ido-pool) implementation.

# Program commands

Note that this program is implemented using the [Anchor](https://github.com/project-serum/anchor) framework, and uses its calling convention. Anchor IDL files can be generated using Anchor's build commands.

## initialize\_pool(name, bumps, num\_ido\_tokens, times)

Creates and initializes a new IDO with a given name, `num_ido_tokens` governance tokens available for auction, and the durations of each phase as described in the `times` struct.

### Accounts

| Name                       | Mutable | Signer                  | Description                                                                                |
|----------------------------|---------|-------------------------|--------------------------------------------------------------------------------------------|
| ido\_authority             | yes     | yes                     | Account that manages the IDO.                                                              |
| ido\_authority\_watermelon | yes     | owned by ido\_authority | SPL token account that holds governance tokens, to be moved into the IDO's escrow account. |
| ido\_account               | init    | pda                     | PDA to hold the IDO data.                                                                  |
| usdc\_mint                 | no      | no                      | SPL mint account of the USDc token.                                                        |
| redeemable\_mint           | init    | pda                     | SPL mint account of the option token, initialized by this instruction.                     |
| watermelon\_mint           | no      | owned by ido\_authority | SPL mint account of the governance tokens.                                                 |
| pool\_watermelon           | init    | pda                     | Escrow account to hold governance tokens.                                                  |
| pool\_usdc                 | init    | pda                     | Escrow account to hold USDc.                                                               |
| system\_program            | no      | no                      | Solana system program sysvar.                                                              |
| token\_program             | no      | no                      | Solana token program sysvar.                                                               |
| rent                       | no      | no                      | Solana rent sysvar.                                                                        |


## init\_user\_redeemable()

Initializes an account to hold a user's options.

### Accounts

| Name             | Mutable | Signer                   | Description                            |
|------------------|---------|--------------------------|----------------------------------------|
| user\_authority  | yes     | yes                      | Owner of the options account.          |
| user\_redeemable | init    | owned by user\_authority | Address of the SPL account to create.  |
| ido\_account     | no      | pda                      | PDA to hold the IDO data.              |
| redeemable\_mint | no      | pda                      | SPL mint account of the option tokens. |
| system\_program  | no      | no                       | Solana system program sysvar.          |
| token\_program   | no      | no                       | Solana token program sysvar.           |
| rent             | no      | no                       | Solana rent sysvar.                    |

## exchange\_usdc\_for\_redeemable(amount)

Purchases options with USDc.

### Accounts

| Name             | Mutable | Signer                   | Description                                   |
|------------------|---------|--------------------------|-----------------------------------------------|
| user\_authority  | yes     | yes                      | Account purchasing the options.               |
| user\_usdc       | mut     | owned by user\_authority | SPL token account for the user's USDc.        |
| user\_redeemable | mut     | owned by user\_authority | SPL token account for the user's options.     |
| ido\_account     | no      | pda                      | PDA to hold the IDO data.                     |
| usdc\_mint       | no      | pda                      | SPL mint account of the option tokens.        |
| redeemable\_mint | no      | pda                      | SPL mint account of the option tokens.        |
| pool\_usdc       | yes     | pda                      | Escrow account for the contract to hold USDc. |
| token\_program   | no      | no                       | Solana token program sysvar.                  |

## init\_escrow\_usdc()

Initializes an escrow account to hold a user's USDc after they sell options.

### Accounts

| Name             | Mutable | Signer | Description                                               |
|------------------|---------|--------|-----------------------------------------------------------|
| user\_authority  | yes     | yes    | Account purchasing the options.                           |
| escrow\_usdc     | init    | pda    | Address of the user's SPL token escrow account to create. |
| ido\_account     | no      | pda    | PDA to hold the IDO data.                                 |
| usdc\_mint       | no      | pda    | SPL mint account of the option tokens.                    |
| redeemable\_mint | no      | pda    | SPL mint account of the option tokens.                    |
| system\_program  | no      | no     | Solana system program sysvar.                             |
| token\_program   | no      | no     | Solana token program sysvar.                              |
| rent             | no      | no     | Solana rent sysvar.                                       |

## exchange\_redeemable\_for\_usdc(amount)

Sells options back to USDc, placing the USDc in the user's escrow account.

### Accounts

| Name             | Mutable | Signer                   | Description                                       |
|------------------|---------|--------------------------|---------------------------------------------------|
| user\_authority  | yes     | yes                      | Account purchasing the options.                   |
| escrow\_usdc     | mut     | pda                      | Escrow SPL token account to hold the user's USDc. |
| user\_redeemable | mut     | owned by user\_authority | SPL token account for the user's options.         |
| ido\_account     | no      | pda                      | PDA to hold the IDO data.                         |
| usdc\_mint       | no      | pda                      | SPL mint account of the option tokens.            |
| redeemable\_mint | no      | pda                      | SPL mint account of the option tokens.            |
| pool\_usdc       | yes     | pda                      | Escrow account for the contract to hold USDc.     |
| token\_program   | no      | no                       | Solana token program sysvar.                      |

## exchange\_redeemable\_for\_watermelon(amount)

Exchanges options for governance tokens at the current exchange rate.

### Accounts

| Name             | Mutable | Signer                   | Description                                                |
|------------------|---------|--------------------------|------------------------------------------------------------|
| user\_authority  | yes     | yes                      | Account exchanging the options.                            |
| user\_watermelon | yes     | owned by user\_authority | SPL token account for the user's governance tokens.        |
| user\_redeemable | mut     | owned by user\_authority | SPL token account for the user's options.                  |
| ido\_account     | no      | pda                      | PDA to hold the IDO data.                                  |
| watermelon\_mint | no      | pda                      | SPL mint account of the governance tokens.                 |
| redeemable\_mint | no      | pda                      | SPL mint account of the option tokens.                     |
| pool\_watermelon | yes     | pda                      | Escrow account for the contract to hold governance tokens. |
| token\_program   | no      | no                       | Solana token program sysvar.                               |

## withdraw\_pool\_usdc()

(admin) Withdraws USDc used to purchase options from the contract.

### Accounts

| Name                 | Mutable | Signer                  | Description                                   |
|----------------------|---------|-------------------------|-----------------------------------------------|
| ido\_authority       | yes     | yes                     | IDO admin account.                            |
| ido\_authority\_usdc | mut     | owned by ido\_authority | SPL token account to hold the owner's USDc.   |
| ido\_account         | no      | pda                     | PDA to hold the IDO data.                     |
| usdc\_mint           | no      | pda                     | SPL mint account of the option tokens.        |
| pool\_usdc           | yes     | pda                     | Escrow account for the contract to hold USDc. |
| token\_program       | no      | no                      | Solana token program sysvar.                  |

## withdraw\_from\_escrow(amount)

Withdraws USDc from a user's escrow account into their normal USDc wallet.

### Accounts

| Name            | Mutable | Signer                   | Description                              |
|-----------------|---------|--------------------------|------------------------------------------|
| payer           | no      | yes                      | Payer for the transaction.               |
| user\_authority | no      | yes                      | Owner of the escrow account.             |
| user\_usdc      | mut     | owned by user\_authority | SPL token account to withdraw USDc into. |
| escrow\_usdc    | mut     | pda                      | USDc escrow account of the user.         |
| ido\_account    | no      | pda                      | PDA to hold the IDO data.                |
| usdc\_mint      | no      | pda                      | SPL mint account of the option tokens.   |
| token\_program  | no      | no                       | Solana token program sysvar.             |

## update\_exchange\_rate(num, denom)

(admin) Sets the current KPI-based exchange rate to the given rational number.

### Accounts

| Name           | Mutable | Signer | Description               |
|----------------|---------|--------|---------------------------|
| ido\_authority | yes     | yes    | IDO admin account.        |
| ido\_account   | no      | pda    | PDA to hold the IDO data. |

## Environment Variables (Solana)

For the deployment scripts:

| Name                        | Description                                                                             |
|-----------------------------|-----------------------------------------------------------------------------------------|
| `ANCHOR_WALLET`             | Path to the `id.json` for the `ido_authority` account.                                  |
| `FLU_IDO_USDC_MINT`         | Public key of the USDc mint account.                                                    |
| `FLU_IDO_WATERMELON_MINT`   | Public key of the governance token mint account.                                        |
| `FLU_IDO_WATERMELON_SOURCE` | Public key of the account to take governance tokens from, must be owned by your wallet. |
| `FLU_IDO_WATERMELON_AMOUNT` | Number of governance tokens to take, in raw decimals.                                   |
| `FLU_IDO_NAME`              | Name of the IDO.                                                                        |
| `FLU_IDO_START`             | Unix timestamp of when to start the IDO.                                                |
| `FLU_IDO_END_DEPOSITS`      | Unix timestamp of when to end deposits.                                                 |
| `FLU_IDO_END_IDO`           | Unix timestamp of when to end USDc withdrawls and allow exchange for governance tokens. |
| `FLU_IDO_END_ESCROW`        | Unix timestam of when to allow users to withdraw their USDc from escrow.                |
| `FLU_IDO_RPC_ADDR`          | Address of the Solana RPC node to connect to.                                           |

For the docker environment:

| Name                   | Description                                                                                                    |
|------------------------|----------------------------------------------------------------------------------------------------------------|
| `FLU_IDO_START_TIME`   | Number of seconds to wait before starting the IDO.                                                             |
| `FLU_IDO_DEPOSIT_TIME` | Number of seconds to wait before entering the withdrawl only phase (from zero, not from the start of the IDO). |
| `FLU_IDO_IDO_TIME`     | Number of seconds to wait before entering the governance token exchange phase.                                 |
| `FLU_IDO_ESCROW_TIME`  | Number of seconds to wait before allowing users to withdraw tokens from escrow.                                |

## Prerequisites

- [anchor](https://github.com/project-serum/anchor)

## Building

- `anchor build`

## Building (Docker)

- `docker build .`

## Running

- `anchor deploy`
- `npx scripts/deploy.ts`

## Running (Docker)

- `docker-compose up`

