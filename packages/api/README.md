# Brøk API
Creates a simplified interface for enterprise systems, so they can use BRØK without the need for any knowledge or system implementation of web3 technologies.

Built with Next.js framework.

Remember to complete the steps described [here](./../../README.md#development) before running this application the first time.

The chapter on .env describes how this application is configured to connect to other application.


## Application endpoints overview

``/api/health`` Checks the status for the external connection, and ethereum balance

``/api/checkTransaction`` Checks the status of a transaction

``/api/v1/company/`` Control the CapTable for a company

``/api/v1/person/`` Get the ownership information for a person or company owning shares

For more explanation, check out the [swagger file](./../../docs/swagger.yaml)


## External applications

This application depends on the following to work properly:
- an Ethereum network, either Hardhat network running locally, or Arbitrum Goerli/Sepolia/One
- The Graph subgraph for CapTableRegistry
- Navnetjener API

To start Hardhat network and The Graph services locally, run the vs code task `deploy-local`

To start Navnetjener, follow [these instructions](https://github.com/brreg/brok-navnetjener#development-setup)


## .env
The .env files uses the following values:

| **Value** | **Description** |
|--|--|
| `PRIVATE_KEY`     | The Private key for the wallet used by Brøk API, intended to be provided by Brønnøysundregistrene in the future |
| `RPC_LOCAL`       | Network address to the Hardhat network |
| `RPC_SEPOLIA`     | Network address to the Arbitrum Sepolia network |
| `RPC_GOERLI`      | Network address to the Arbitrum Goerli network |
| `RPC_ONE`         | Network address to the Arbitrum One network |
| `PORT`            | Port for the API |
| `NAVNETJENER_URL` | URL to Navnetjener API |
| `BROK_ENV`        | Selects Ethereum network, valid arguments: `localhost`, `brokStage` or `brokProd` |
| `DEBUG`           | Select what part of the application you want to debug, default to `brok*` for the whole application |
| `THE_GRAPH_URL`   | URL to The Graph subgraph |


# Development

Start dev server: `pnpm run dev`

Run all tests: `pnpm run test`