import { localhostContracts, brokDevContracts, brokStageContracts, brokProdContracts } from "@brok/captable";
import { ethers } from "ethers";
import debug from "debug";
const log = debug("brok:api:contants");

if (!process.env.PRIVATE_KEY) {
	throw new Error("Please set PRIVATE_KEY in your .env file");
}

const Networks = {
	ARBITRUM_GOERLI: 421613,
	ARBITRUM_SEPOLIA: 421614,
	ARBITRUM_ONE: 42161,
	LOCALHOST: 31337,
} as const;

const StartBlocks = {
	[Networks.ARBITRUM_GOERLI]: 49339985,
	[Networks.LOCALHOST]: 0,
	[Networks.ARBITRUM_SEPOLIA]: 628100,
	[Networks.ARBITRUM_ONE]: 142000000,
} as const;

const ContractAddresses = {
	[Networks.ARBITRUM_GOERLI]: brokStageContracts,
	[Networks.LOCALHOST]: localhostContracts,
	[Networks.ARBITRUM_ONE]: brokProdContracts,
} as const;

export type CurrentNetwork = typeof Networks[keyof typeof Networks];
export const DEFAULT_NETWORK = (() => {
	switch (process.env.BROK_ENV) {
		case "localhost":
			return Networks.LOCALHOST;
		case "brokStage":
			return Networks.ARBITRUM_GOERLI;
		case "brokProd":
			return Networks.ARBITRUM_ONE;
		default:
			throw new Error(`Invalid BROK_ENV: ${process.env.BROK_ENV}`);
	}
})();
export const RPC_URL = (() => {
	switch (DEFAULT_NETWORK) {
		case Networks.LOCALHOST:
			return process.env.RPC_LOCAL;
		case Networks.ARBITRUM_ONE:
			return process.env.RPC_ONE;
		case Networks.ARBITRUM_GOERLI:
			return process.env.RPC_GOERLI;
		default:
			throw new Error(`Invalid RPC URL for network: ${process.env.BROK_ENV}`);
	}
})();

log("Using network: %s, with RPC: %s", DEFAULT_NETWORK, RPC_URL);
export const START_BLOCK = StartBlocks[DEFAULT_NETWORK];
export const CONTRACT_ADDRESSES = ContractAddresses[DEFAULT_NETWORK];
export const WALLET = new ethers.Wallet(process.env.PRIVATE_KEY);

if (!('CAP_TABLE_REGISTRY' in CONTRACT_ADDRESSES)) {
	throw new Error(`CAP_TABLE_REGISTRY is not defined for the network: ${DEFAULT_NETWORK}`);
}


export const GET_PROVIDER = () => {
	return new ethers.providers.JsonRpcProvider({
		// biome-ignore lint/style/noNonNullAssertion: <explanation>
		url: RPC_URL!,
	});
};

const SIGNATURE = WALLET.signMessage("This is just to create an stealth address");
// export const STEALTH_KEYS = async () => signatureToStealthKeys(await SIGNATURE);
// export const SPEND_KEY = async () => (await STEALTH_KEYS()).spend;
// export const VIEW_KEY = async () => (await STEALTH_KEYS()).view;

// BR - 0x0a665B1Bc813cAE9fcDd2Eb7E25b8E55A5F35f23
// TODO legg til BR og Fagsystem public address
export const CONTROLLERS = ["0x5eada5BFB4A57C08dC6A957A5A11488152F2CB5E"];
export const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");
export const THE_GRAPH_URL = process.env.THE_GRAPH_URL
