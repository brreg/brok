import { localhostContracts, brokDevContracts } from "@brok/captable";
import { ethers } from "ethers";
import debug from "debug";
const log = debug("brok:api:contants");

if (!process.env.PRIVATE_KEY) {
	throw new Error("Please set PRIVATE_KEY in your .env file");
}
if (!process.env.RPC_URL) {
	throw new Error("Please set RPC_URL in your .env file");
}

const Networks = {
	ARBITRUM_GOERLI: 421613,
	LOCALHOST: 31337,
} as const;

const StartBlocks = {
	[Networks.ARBITRUM_GOERLI]: 5074309,
	[Networks.LOCALHOST]: 0,
} as const;

const ContractAddresses = {
	[Networks.ARBITRUM_GOERLI]: brokDevContracts,
	[Networks.LOCALHOST]: localhostContracts,
} as const;

export type CurrentNetwork = typeof Networks[keyof typeof Networks];
export const DEFAULT_NETWORK = process.env.NODE_ENV === "production" ? Networks.ARBITRUM_GOERLI : Networks.LOCALHOST;
log("Using network: %s", DEFAULT_NETWORK);
export const START_BLOCK = StartBlocks[DEFAULT_NETWORK];
export const CONTRACT_ADDRESSES = ContractAddresses[DEFAULT_NETWORK];
export const WALLET = new ethers.Wallet(process.env.PRIVATE_KEY);

export const GET_PROVIDER = () => {
	return new ethers.providers.JsonRpcProvider({
		// biome-ignore lint/style/noNonNullAssertion: <explanation>
		url: process.env.RPC_URL!,
	});
};

const SIGNATURE = WALLET.signMessage("This is just to create an stealth address");
// export const STEALTH_KEYS = async () => signatureToStealthKeys(await SIGNATURE);
// export const SPEND_KEY = async () => (await STEALTH_KEYS()).spend;
// export const VIEW_KEY = async () => (await STEALTH_KEYS()).view;

export const CONTROLLERS = ["0x0a665B1Bc813cAE9fcDd2Eb7E25b8E55A5F35f23"];
export const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");
export const THE_GRAPH_URL = "http://localhost:8000/subgraphs/name/brok/captable"; // TODO - FIX enviroment variable
