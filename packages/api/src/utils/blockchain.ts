import { ERC5564Registry__factory } from "@brok/captable";
import debug from "debug";
import { ApiError } from "next/dist/server/api-utils";
import { CONTRACT_ADDRESSES, GET_PROVIDER, WALLET } from "../contants";

const log = debug("brok:utils:blockchain");

export class EthereumError extends ApiError {
	constructor(statusCode: number, message: string) {
		super(statusCode, message)
	}
}

// rome-ignore lint/suspicious/noExplicitAny: <explanation>
export function handleRPCError(error: any): string {
	try {
		// Insufficient funds
		if ("error" in error && "message" in error.error) {
			if ("reason" in error.error) {
				log(error.error.reason);
				if (error.error.reason.includes("reverted with reason string")) {
					return error.error.reason.split("'")[1];
				}
				return error.error.reason;
			}
			if ("message" in error.error) {
				log(error.error.message);
				return error.error.message;
			}
		}
		log("Unknown error:", error);
		return `Unknown error: ${error}`;
	} catch (error) {
		log("Error parsing error message: ", error);
		return `Error parsing error message: ${error}`;
	}
}

/**
 * Returns stealth address registry as a ERC5564Registry object
 * 
 * Used to add 
 * 
 * Permissions: Read/Write
 * 
 * @returns ERC5564Registry object
 */
export function connectToStealthAddressFactory_RW() {
	const wallet = WALLET.connect(GET_PROVIDER());
	const registry = new ERC5564Registry__factory(wallet).attach(CONTRACT_ADDRESSES.ERC5564_REGISTRY);
	return registry
}