# BRØK

## Overview

BRØK is a cutting-edge service developed for sharing shareholder information. It connects shareholder registers using blockchain technology, making ownership information easily accessible for financial institutions, the press, public agencies, and other service providers.

With 380,000 corporations maintaining their shareholder books, BRØK aims to create a single platform that enables the transparent and efficient exchange of information.

## Features

- **First-time publishing of information from the shareholder book**
- **Transfer of shares (existing/new shareholders)**
- **Information about a shareholder, changes, and deletion**
- **Information about the assignment/deletion of shares related to company events**
- **Information about share pledges**
- **Reading the latest version of the shareholder book**
<!-- - **Deactivation of a shareholder book (inactive)** -->

## Architecture

BRØK's architecture includes three key components:
- **Blockchain**: Contains immutable structures, business logic, and anonymous pointers to owners. See packages/captable
- **API**: Provides a REST API for reading and writing to the blockchain. See packages/api
- **TheGraph**: Provides a GraphQL API for reading from the blockchain. See packages/graph
- **BR Name Service**: Provides a service for looking up BRØK addresses. See [https://github.com/brreg/brok-navnetjener](https://github.com/brreg/brok-navnetjener)
- **Fagsystem**: Integrates with BRØK, enabling the publishing of ownership details. Without a front-end like this, BRØK is not useful. Here's an example of a front-end for a fagsystem: [https://github.com/brreg/brok-fagsystem-demo](https://github.com/brreg/brok-fagsystem-demo).


## Development

### Requirements
- Node
- pnpm
- [Hardhat](https://hardhat.org/hardhat-runner/docs/getting-started#installation)
- Either Docker with Docker Compose or Podman with Podman Compose

### Environment Variables Setup

#### The Captable package 

Firstly, create a `.env` file in `packages/captable` by copying the `.env.example` file:

```bash
cp packages/captable/.env.example packages/captable/.env
```

Then, update the `.env` file with the correct values for your environment.

Below are the necessary environment variables that need to be configured for proper operation of the system. You should set these variables based on your specific configuration and requirements.

- **ETHERSCAN_API_KEY**: Your personal Etherscan API key used for verifying smart contracts (e.g., `1234567890ABCDEF`).

- **SEED_DEV**, **SEED_STAGE**, **SEED_PROD**: Seed phrases for the development, staging, and production environments respectively (e.g., `test test test test test test test test test test test junk`).

- **RPC_TESTNET**: The HTTP URL of the testnet Ethereum RPC node (e.g., `http://127.0.0.1:8545/`).

- **RPC_MAINNET**: The HTTP URL of the mainnet Ethereum RPC node with your personal Etherscan API key if using Infura or Alchemy (e.g., `https://arb.g.alchemy.com/v2/ABC123…`).

- **REPORT_GAS**: A flag to control the reporting of gas consumption (e.g., `false`).

- **DEV_ENTERPRISE_SYSTEM_ADDRESS**: The Ethereum address for the enterprise system in the development environment (e.g., `"0x0a123..."`).

#### The API package 

Create a `.env` file in `packages/api` by copying the `.env.example` file:

```bash
cp packages/api/.env.example packages/api/.env
```

Then, update the `.env` file with the correct values for your environment. If you need a new private key, you can generate one using the following command:

```bash
node packages/api/createWallet.js
```

Below are the necessary environment variables that need to be configured for proper operation of the system. You should set these variables based on your specific configuration and requirements.

- **PRIVATE_KEY**: The hexadecimal representation of a private key used for signing transactions and managing an Ethereum account. (e.g., `0xSOME_PRIVATE_KEY_HEX`).

- **RPC_URL**: The HTTP URL of the Ethereum RPC node that your application communicates with. This URL connects your application to the Ethereum network (e.g., `"http://127.0.0.1:8545"`).

- **PORT**: The port number on which the application is set to run. This variable specifies the desired port for hosting the application (e.g., `4000`).

Please ensure that these environment variables are set correctly in your system. Misconfiguration of these variables may lead to unexpected behavior or errors in the application.

### Running Locally
Follow the detailed instructions to compile smart contracts, start the local Ethereum blockchain, add demo data, start The Graph servers, and launch the API Server.

First, install dependencies:

```bash
pnpm install
```

Build the smart contracts:

```bash
pnpm --filter @brok/captable hardhat compile --force
pnpm --filter @brok/captable build
```

#### 👩‍💻 Running locally with VSCode tasks (preferred) 
In VScode, run task `deploy-local`  (⇧⌘P Task:Run task).  

This will do the following:
- Compile smartcontracts with Hardhat in `./packages/captable/contracts`
- Starts a ethereum blockchain on localhost with Hardhat
- Add demo data to newly deployed local blockchain, from `./packages/captable/tasks/demo-cap-table.ts`
- Starts The Graph servers, with spec from `./packages/graph`
- Starts API Server from `./packages/api`

State is only stored runtime, so if you stop and rerun the deployment script, all changes will be lost!  
If you want to have data thats persists between runtime, add them to `./packages/captable/tasks/demo-cap-table.ts`

#### Testing

* Note that you need to cd into the `packages/api` directory before running the API tests. Also note that some kind of race condition exists in the tests, so one and one file have to be run individually for the time being.
* Also note that you need to have the `deploy-local` VSCode task running before running the tests.

## Ugly hacks
- Problems with Key DID provider secp256k1 so we are deriving ED25519 from secp256k1 private key.

# Windows
Before install replace the following scripts in packages/captable/package.json
```
"prebuild": "if not exist tasksCopy mkdir tasksCopy & copy /Y tasks\\* tasksCopy & rmdir /Q /S tasks & mkdir tasks & type nul > tasks\\index.ts",
"postbuild": "xcopy /Y tasksCopy\\* tasks & rmdir /Q /S tasksCopy".
```