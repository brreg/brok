import { CapTable, CapTableRegistry, CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
import debug from "debug";
import { ApiError } from "next/dist/server/api-utils";
import { CONTRACT_ADDRESSES, GET_PROVIDER, WALLET } from "../contants";

const log = debug("brok:utils:blockchain");

// TODO Consider removing
export class EthereumError extends ApiError {}

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
 * Give you access to the CapTable Registry
 *
 * Lets you read from the registry
 *
 * Permissions: Read
 *
 * @returns CapTableRegistry object
 */
export async function ConnectToCapTableRegistry_R(): Promise<CapTableRegistry> {
	const registry = new CapTableRegistry__factory()
		.attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY)
		.connect(GET_PROVIDER());
	return registry;
}

/**
 * Give you access to the CapTable Registry
 *
 * Lets you modify the content of the registry
 *
 * Permissions: Read/Write
 *
 * @returns CapTableRegistry object
 */
export async function ConnectToCapTableRegistry_RW(): Promise<CapTableRegistry> {
	const wallet = WALLET.connect(GET_PROVIDER());
	const registry = new CapTableRegistry__factory(wallet).attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);
	return registry;
}

/**
 * Gives you access to a CapTable
 *
 * Lets you read information about the CapTable
 *
 * Permissions: Read
 *
 * @param capTableAddress Hex address to CapTable
 * @returns CapTable object
 */
export async function ConnectToCapTable_R(capTableAddress: string): Promise<CapTable> {
	const captable = await new CapTable__factory().attach(capTableAddress).connect(GET_PROVIDER());
	return captable;
}
