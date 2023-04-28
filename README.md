![Issues](https://img.shields.io/github/issues/brreg/brok)

# About
BRØK SDK is a browser and node library to manage cap tables within the BRØK ecosystem.

Cap tables consist of "immutable" data like transactions and balances, handled on a blockchain. It also consist of more personal information about the owner, which is handled on Ceramic. You can interact with your assets through a wallet (some signing functionality). There are also some services to make data more indexable and quickly accessible. All this is packaged in this library for easier access.

<!-- # Getting started

## Work in progress!

Download image from url  
Swagger documentation url -->

<!-- Install library from [npm](https://www.npmjs.com/package/@brok/sdk)

```npm i @brok/sdk```

... or yarn
```yarn add @brok/sdk```

... or pnpm
`pnpm i @brok/sdk`

Then init the SDK

```ts
  const sdk = await SDK.init({
    ceramicUrl: 'https://ceramic-clay.3boxlabs.com',
    ethereumRpc: 'https://goerli-rollup.arbitrum.io/rpc',
    secret: 'test test test test test test test test test test test junk',
    theGraphUrl:
      'https://api.thegraph.com/subgraphs/name/broklab/captable_dev_11',
    env: 'brokDev',
  });
```
Read more about inputs to init SDK in [documentation](https://demo-docs-site.onrender.com/sdk-documentation)

You can see examples here:

- [browser example nextjs](https://stackblitz.com/edit/nextjs-j6bqhx?file=pages%2Findex.js)
- [server example nodejs](https://stackblitz.com/edit/node-bzd6sj?file=index.js) -->


# Instances
You can read current deployments from [npm captable](https://www.npmjs.com/package/@brok/captable)

CapTableRegistry.sol

- dev brokDEV: `0xaC7349fc43fEc778f1FA2475b3F850Ca17163557`
- stage brokStage: `0x5f97A62c01FAe8280344ec7Eb505ADf8397D9a1C`
- prod brokProd: `0x4e33Adb3A77B5685E351A61f6bFb20d9dfF71E76`

Blockchain index [TheGraph](https://api.thegraph.com/subgraphs/name/broklab/captable_dev_11)


# Development

## Requirements

- [Node](https://nodejs.org/en/blog/release/v16.14.2/)
- [pnpm](https://pnpm.io/installation) 
- [Hardhat](https://hardhat.org/hardhat-runner/docs/getting-started#installation)
- [Podman](https://podman.io/getting-started/installation)
- [Podman Compose](https://github.com/containers/podman-compose)

## Note on differences between Docker and Podman
There are some minor yet significant differences between Docker and Podman, with regards to this project, the difference lies in the way they map the address to their respective hosts.  
with `Podman`, the host address is added automatically as `host.containers.internal`, (see the `/etc/hosts` file inside the container)  
while `Docker` needs to manually add this network mapping in `docker compose`, using `host.docker.internal` as the name of the host

Currently this project is using Podman, if you want to switch to Docker, do the following changes in `packages/graph/podman-compose/the-graph.yaml`
```diff
  graph-node:
+   extra_hosts:
+     - host.docker.internal:host-gateway
    environment:
-     ethereum: 'mainnet:http://host.containers.internal:8545'
+     ethereum: 'mainnet:http://host.docker.internal:8545'
```

### 👩‍💻 Running locally with VSCode tasks (preferred) 
In VScode, run task `deploy-local`  (⇧⌘B workbench.action.tasks.runTask) (Ctrl⇧P Task:Run task).  

This will do the following:
- Compile smartcontracts with Hardhat in `./packages/captable/contracts`
- Starts a ethereum blockchain on localhost with Hardhat
- Add demo data to newly deployed local blockchain, from `./packages/captable/tasks/demo-cap-table.ts`
- Starts The Graph servers, with spec from `./packages/graph`
- Starts API Server from `./packages/api`

State is only stored runtime, so if you stop and rerun the deployment script, all changes will be lost!  
If you want to have data thats persists between runtime, add them to `./packages/captable/tasks/demo-cap-table.ts`

<!-- ## Deployments
Release packages of SDK and CapTable (you can choose what to publish update on with changeset)
```
pnpm changeset
pnpm changeset version
pnpm install
# commit the changes, need to update lockfiles.
pnpm publish -r
``` -->

### Deploy TheGraphCapTable service

Make sure @brok/graph package is using desired @brok/captable version in package.json

```bash
pnpm --filter @brok/graph deploy:brokDev # deploy:brokLocal deploy:brokStage deploy:brokProd
```

<!-- ### Deploy frontend and servers

Will deploy by instructions of render.yaml file. 

## Packages
- SDK [NPM](https://www.npmjs.com/package/@brok/sdk)
- Captable [NPM](https://www.npmjs.com/package/@brok/captable)

So SDK and Captable are NPM packages that needs to be published for changes to propagate. 
Graph, demo-server and demo-frontend needs to be deployed to their environments to propagate changes. -->


## Environment variables 

The main environments variables that you need to familiarize with:
- An Ethereum RPC (We recommend [alchemyapi.io](https://dashboard.alchemyapi.io/) or [Infura](https://infura.io/))
- A ceramic node [https://ceramic.network/](https://ceramic.network/)
- An Ethereum secret (seed phrase). You can generate one with [Ethers](https://docs.ethers.io/v5/)
- The Graph API indexing captable contracts [thegraph.com](https://thegraph.com/en/)

### Environment setup
1. Copy .env.example to .env in packages/captable, packages/demo-frontend and packages/demo-server. There is a make command for this.
1. Get yourself an Ethereum RPC and Ethereum secret and put these into /.env and ./packages/captable/.env
1. Then you should be able to generate the SDK for any chain.

SDK will look for environment variable BROK_ENVIRONMENT to determine which contracts to choose. Set this environments in your runtime.
- local localhost - Will use local blockchain
- dev brokDev - Will use Arbitrum Goerli
- stage brokStage - Will use Arbitrum Goerli
- prod brokProd - Will use Arbitrum mainnet (not currently)

To create an approved CapTable, the wallet for fagsystem must first be authorized by BRREG. Contact us.
## Ugly hacks
- Problems with Key DID provider secp256k1 so we are deriving ED25519 from secp256k1 private key.



# Windows
Before install replace the following scripts in packages/captable/package.json
```
"prebuild": "if not exist tasksCopy mkdir tasksCopy & copy /Y tasks\\* tasksCopy & rmdir /Q /S tasks & mkdir tasks & type nul > tasks\\index.ts",
"postbuild": "xcopy /Y tasksCopy\\* tasks & rmdir /Q /S tasksCopy".
```