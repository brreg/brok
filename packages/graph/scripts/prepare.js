const brok = require("@brok/captable");
const yaml = require("js-yaml");
const fs = require("fs");

const args = process.argv.slice(2);
const env = args[0];
const subgraphConfig = yaml.load(fs.readFileSync("subgraph.example.yaml", "utf8"));

// set address
for (const dataSource of subgraphConfig.dataSources) {
	dataSource.source.address = brok[`${env}Contracts`].CAP_TABLE_REGISTRY;
	console.log("use this address for CapTableRegistry", dataSource.source.address);
	if (env === "localhost") {
		dataSource.source.startBlock = 1;
	}
	if (env === "brokStage" || env === "brokDev") {
		dataSource.source.startBlock = 49339985;
	}
	if (env === "brokProd") {
		dataSource.source.startBlock = 142000000;
	}
}
// set chain
if (env === "localhost") {
	for (const dataSource of subgraphConfig.dataSources) {
		dataSource.network = "mainnet";
	}
	for (const template of subgraphConfig.templates) {
		template.network = "mainnet";
	}
} else if (env === "brokStage") {
	for (const dataSource of subgraphConfig.dataSources) {
		dataSource.network = "arbitrum-goerli";
	}
	for (const template of subgraphConfig.templates) {
		template.network = "arbitrum-goerli";
	}
} else if (env === "brokProd") {
	for (const dataSource of subgraphConfig.dataSources) {
		dataSource.network = "arbitrum-one";
	}
	for (const template of subgraphConfig.templates) {
		template.network = "arbitrum-one";
	}
} else {
	throw new Error("could not determine network")
}

fs.writeFileSync("subgraph.yaml", yaml.dump(subgraphConfig));
