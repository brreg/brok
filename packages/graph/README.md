# Search indexing for CapTableRegistry BRØK

The Graph is an indexing protocol for organizing blockchain data and making it easily accessible with GraphQL.

The Graph makes it possible to query data that is difficult to query directly.

The Graph learns what and how to index Ethereum data based on subgraph descriptions, known as the subgraph manifest. The subgraph description defines the smart contracts of interest for a subgraph, the events in those contracts to pay attention to, and how to map event data to data that The Graph will store in its database.

Once you have written a `subgraph manifest`, you use the Graph CLI to store the definition in IPFS and tell the indexer to start indexing data for that subgraph.

This diagram gives more detail about the flow of data once a subgraph manifest has been deployed, dealing with Ethereum transactions:


## Project overview

| **Folder** | **Description** |
|--|--|
| `abis/` | ABIS files for CapTable and CapTableRegistry, ***NB: These files are generated and copy manually*** |
| `build/` | Generated files from `graph codegen && graph build`, based on the content of `subgraph.yaml` |
| `docker-compose/` | Specs for running TheGraph locally with Docker compose |
| `generated/` | Some manually generated files @jon ? |
| `podman-compose/` | Specs for running TheGraph locally with Podman compose
| `scripts/` | Takes constant variables from `packages/captable/deployment/` and creates `subgraph.yaml` |
| `src/` | Some more files copy @jon ? |
| `tests/` | I dont know @jon ? |

## Deployment prosess
To deploy subgraph to The Graph hosted services. update the deployment name in `package.json` and run one of the `deploy:*` commands