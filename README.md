# BRØK

## Overview

BRØK is a cutting-edge service developed for sharing shareholder information. It connects [shareholder registers](https://no.wikipedia.org/wiki/Aksjeeierbok) using blockchain technology, making ownership information easily accessible to [financial institutions](https://no.wikipedia.org/wiki/Finansinstitusjon), the press, [public agencies](https://no.wikipedia.org/wiki/Etat), and other service providers.

With 380,000 corporations maintaining their shareholder books, BRØK aims to create a single platform that enables the transparent and efficient exchange of information.

## Features

- **First-time publishing of information from the shareholder book**
- **Transfer of shares (existing/new shareholders)**
- **Managing shareholder information**
- **Information about the assignment/deletion of shares related to company events**
- **Information about share pledges**
- **Reading the latest version of the shareholder book**
<!-- - **Deactivation of a shareholder book (inactive), shareholder data deletion** -->

## Architecture

BRØK's architecture includes three key components:
- **Blockchain**: Rules defined by [smart contracts](https://no.wikipedia.org/wiki/Smart_kontrakt) and secure references to ownership. See `packages/captable`
- **TheGraph**: Enables efficient data queries from the blockchain. See `packages/graph`
- **API**: Allows secure data access and updates through standard web technology. See `packages/api`
- **BR Name Service**: Provides a [service for looking up BRØK addresses](https://github.com/brreg/brok-navnetjener).
- **Fagsystem DEMO**: Integrates with BRØK, enabling the publishing of ownership details. Without a front-end like this, BRØK is not useful. [Here's an example of a front-end for a fagsystem.](https://github.com/brreg/brok-fagsystem-demo)


# Quick setup

### Requirements

- curl
- Docker (or Podman) with docker-compose

`curl -sSL https://raw.githubusercontent.com/brreg/brok/v6/quick-setup.sh | bash`

### Next startup and shutdown

To start up next time use docker directly with:
`docker-compose -p brreg-bok-localhost -f docker-compose.yaml up`

And to shut down all the containers use:
`docker-compose -p brreg-bok-localhost -f docker-compose.yaml down`

# Development

### Requirements
- Node
- pnpm
- [Hardhat](https://hardhat.org/hardhat-runner/docs/getting-started#installation)
- Compatible container orchestration systems such as Docker or Podman

### Environment Variables Setup

#### The Captable package 

Firstly, create a `.env` file in `packages/captable` by copying the `.env.example` file:

```bash
cp packages/captable/.env.example packages/captable/.env
```

Then, update the `.env` file with the correct values for your environment.

Below are the key settings required for system operation. You should set these variables based on your specific configuration and requirements.

- **ETHERSCAN_API_KEY**: Your personal Etherscan API key is used for verifying smart contracts (e.g., `1234567890ABCDEF`).

- **SEED_DEV**, **SEED_STAGE**, **SEED_PROD**: Seed phrases for the development, staging, and production environments, respectively (e.g., `test test test test test test test test test test test junk`).

- **RPC_TESTNET**: The HTTP URL of the testnet Ethereum RPC node (e.g., `http://127.0.0.1:8545/`).

- **RPC_MAINNET**: The HTTP URL of the mainnet Ethereum RPC node with your personal Etherscan API key if using Infura or Alchemy (e.g., `https://arb.g.alchemy.com/v2/ABC123…`).

- **REPORT_GAS**: A flag to control the reporting of gas consumption (e.g., `false`).

- **DEV_ENTERPRISE_SYSTEM_ADDRESS**: The Ethereum address for the enterprise system in the development environment (e.g., `"0x0a123..."`).

**Caution**: Sensitive environment variables such as seed phrases and private keys should be kept secure. Never commit them into source control. 

#### The API package 

Create a `.env` file in `packages/api` by copying the `.env.example` file:

```bash
cp packages/api/.env.example packages/api/.env
```

Then, update the `.env` file with the correct values for your environment. If you need a new private key, you can generate one using the following command:

```bash
node packages/api/createWallet.js
```

Below are the necessary environment variables that need to be configured for the proper operation of the system. You should set these variables based on your specific configuration and requirements.

- **PRIVATE_KEY**: The hexadecimal representation of a private key used for signing transactions and managing an Ethereum account. (e.g., `0xSOME_PRIVATE_KEY_HEX`).

- **RPC_URL**: The HTTP URL of the Ethereum RPC node that your application communicates with. This URL connects your application to the Ethereum network (e.g., `"http://127.0.0.1:8545"`).

- **PORT**: The port number on which the application is set to run. This variable specifies the desired port for hosting the application (e.g., `4000`).

Please ensure that these environment variables are set correctly in your system. Misconfiguration of these variables may lead to unexpected behavior or errors in the application.

**Caution**: Sensitive environment variables such as seed phrases and private keys should be kept secure. Never commit them into source control. 

### Running Locally
Follow the detailed instructions to compile smart contracts (contractual rules programmed into the system), start the local Ethereum blockchain, add demo data, start The Graph servers, and launch the API Server.

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
In VScode, run task `deploy-local`  (⇧⌘P Task: Run task).  

This will do the following:
- Compile smart contracts with Hardhat in `./packages/captable/contracts`
- Initializes a local version of an Ethereum blockchain
- Populate the local network with example data from `./packages/captable/tasks/demo-cap-table.ts`
- Starts The Graph servers, with spec from `./packages/graph`
- Starts API Server from `./packages/api`

Note: Data is temporarily stored and will be erased if the system is restarted.
If you want to have data that persists between runtime, add them to `./packages/captable/tasks/demo-cap-table.ts`

#### Testing

* Ensure you're in the appropriate directory (`packages/api`) before initiating tests. Also, note that some kind of race condition exists in the tests, so one test file at a time has to be run individually for the time being.
* Also note that you need to have the `deploy-local` VSCode task running before running the tests.

## Workarounds
- To address key management issues, we've implemented a secure alternative approach (ED25519 is derived from secp256k1 private key).

# Windows
For Windows installations, please modify the following scripts in the specific package directory. 
```
In packages/captable/package.json:
"prebuild": "if not exist tasksCopy mkdir tasksCopy & copy /Y tasks\\* tasksCopy & rmdir /Q /S tasks & mkdir tasks & type nul > tasks\\index.ts",
"postbuild": "xcopy /Y tasksCopy\\* tasks & rmdir /Q /S tasksCopy".
```

## Local Container

### Podman
```
podman build --target graph -t brok-graph .
podman build --target hardhat -t brok-hardhat .
podman build --target api -t brok-api .
podman-compose -p symfoni_graph -f podman-compose.yaml up
```

### Docker
```
docker build --target graph -t brok-graph .
docker build --target hardhat -t brok-hardhat .
docker build --target api -t brok-api .
docker-compose -p symfoni_graph -f docker-compose.yaml up
```
