import { localhostContracts, brokDevContracts, brokStageContracts, brokProdContracts } from "@brok/captable";
import { ethers } from "ethers";
import debug from "debug";
const log = debug("brok:api:contants");

if (!process.env.PRIVATE_KEY) {
	throw new Error("Please set PRIVATE_KEY in your .env file");
}

const Networks = {
	LOCALHOST: 31337,
	DEV_ARBITRUM_GOERLI: 421613,
	STAGE_ARBITRUM_ONE: 42161,
	PROD_ARBITRUM_ONE: 42161,
} as const;

const StartBlocks = {
	[Networks.LOCALHOST]: 0,
	[Networks.DEV_ARBITRUM_GOERLI]: 49339985,
	[Networks.STAGE_ARBITRUM_ONE]: 142000000,
	[Networks.PROD_ARBITRUM_ONE]: 142000000,
} as const;

const ContractAddresses = {
	[Networks.LOCALHOST]: localhostContracts,
	[Networks.DEV_ARBITRUM_GOERLI]: brokDevContracts,
	[Networks.STAGE_ARBITRUM_ONE]: brokStageContracts,
	[Networks.PROD_ARBITRUM_ONE]: brokProdContracts,
} as const;

export type CurrentNetwork = typeof Networks[keyof typeof Networks];
export const DEFAULT_NETWORK = (() => {
	switch (process.env.BROK_ENV) {
		case "localhost":
			return Networks.LOCALHOST;
		case "brokDev":
			return Networks.DEV_ARBITRUM_GOERLI
		case "brokStage":
			return Networks.STAGE_ARBITRUM_ONE;
		case "brokProd":
			return Networks.PROD_ARBITRUM_ONE;
		default:
			throw new Error(`Invalid BROK_ENV: ${process.env.BROK_ENV}`);
	}
})();
export const RPC_URL = (() => {
	switch (DEFAULT_NETWORK) {
		case Networks.LOCALHOST:
			return process.env.RPC_LOCAL;
		case Networks.DEV_ARBITRUM_GOERLI:
			return process.env.RPC_GOERLI;
		case Networks.STAGE_ARBITRUM_ONE:
			return process.env.RPC_ONE;
		case Networks.PROD_ARBITRUM_ONE:
			return process.env.RPC_ONE;
		default:
			throw new Error(`Invalid RPC URL for network: ${process.env.BROK_ENV}`);
	}
})();

log("Using network: %s, with RPC: %s", DEFAULT_NETWORK, RPC_URL);
export const START_BLOCK = StartBlocks[DEFAULT_NETWORK];
export const CONTRACT_ADDRESSES = ContractAddresses[DEFAULT_NETWORK];
export const WALLET = new ethers.Wallet(process.env.PRIVATE_KEY);

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
export const CONTROLLERS = ["0x0a665B1Bc813cAE9fcDd2Eb7E25b8E55A5F35f23"];
export const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");
export const THE_GRAPH_URL = process.env.THE_GRAPH_URL
