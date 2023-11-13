/**
 * This API endpoint serves to interact with BRØK.
 *
 * ## Features
 *
 * 1. **Modular Code**: Utilizes utility functions for blockchain interactions, making it modular and reusable.
 * 2. **Extensive Logging**: Uses the debug package for logging to make debugging easier.
 * 3. **Robust Error Handling**: Leverages custom ApiError class for expressive error responses.
 * 4. **Data Validation**: Validates incoming POST request data with `parseBody` function.
 * 5. **Transaction Confirmation**: After adding a CapTable, it confirms the action using the `confirmAddedToRegistry` function.
 *
 * ## Supported HTTP Methods
 *
 * - `GET`: Fetches all the existing CapTables from the registry.
 * - `POST`: Registers a new CapTable to the registry.
 *
 * ## Functions
 *
 * - `createCapTableRecord(name: string, orgnr: string)`: Handles the smart contract interaction to create a new CapTable.
 * - `addCapTableRecordToCapTableRegistry(capTableAddress: string, orgnr: string)`: Adds the new CapTable address to the registry.
 * - `getDetailsFromCapTables(captables: string[])`: Retrieves details for a list of CapTable addresses.
 * - `parseBody(body: any)`: Validates incoming request data.
 *
 * ## Error Handling
 *
 * Custom ApiError class is used to throw meaningful errors.
 */

import { CapTable__factory } from "@brok/captable";
import debug from "debug";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from "../../../../contants";
import { ApiRequestLogger, ErrorResponse } from "../../../../utils/api";
import {
	ConnectToCapTableRegistry_R,
	ConnectToCapTableRegistry_RW,
	ConnectToCapTable_R,
	handleRPCError,
} from "../../../../utils/blockchain";
import { getAllForetak } from "../../../../utils/navnetjener";

export type ForetakResponse = {
	capTableAddress: string;
	capTableDeployTransactionHash: string;
	capTableRegistryTransactionHash: string;
};

const log = debug("brok:api:v1:company");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);

		res.setHeader('Access-Control-Allow-Origin', '*');
		res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
		res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');


		if (req.method === 'OPTIONS') {
			return res.status(200).end();
		}

		switch (req.method) {
			case "GET": {
				// Get all captables from registry
				// Get page from query
				const page = req.query.page ? parseInt(req.query.page.toString()) : 0;

				// Page is starting at 0
				const foretakList = await getAllForetak(page);

				log(`HTTP Response 200, return ${foretakList.length} foretak`);
				return res.status(200).json({ foretakList });
			}

			case "POST": {
				// // Register a new captable for company
				const { name, orgnr } = parseBody(req.body);
				log(`HTTP Request POST, creating captable for ${name} with orgnr ${orgnr}`);

				const { capTableAddress, capTableDeployTransactionHash } = await createCapTableRecord(name, orgnr);

				if (!capTableAddress) {
					throw new ApiError(500, "Captable address is not set");
				}

				const transactionHash = await addCapTableRecordToCapTableRegistry(capTableAddress, orgnr);
				log("HTTP Response 200, created captable with transactionHash", transactionHash);

				return res.status(200).json(<ForetakResponse>{
					capTableAddress: capTableAddress,
					capTableDeployTransactionHash: capTableDeployTransactionHash,
					capTableRegistryTransactionHash: transactionHash,
				});


				// // getAddress of orgnr captable
				// const wallet = WALLET.connect(GET_PROVIDER());
				// const addNonce = await wallet.getTransactionCount('pending');

				// const registry = await ConnectToCapTableRegistry_RW();
				// const contractAdr = await registry.getAddress("915772137");
				// console.log("contract address", contractAdr);
				// return res.status(200).json({ contractAdr });


				// // Operator of captable
				// const wallet = WALLET.connect(GET_PROVIDER());
				// const addNonce = await wallet.getTransactionCount('pending');

				// const registry = await ConnectToCapTableRegistry_RW();
				// const operatorAdr = await  registry.getOperatorForCapTable("0x181C5c2bE786B9224Dd24eD954d4C3dfeCf0F512");
				// console.log("operatorAdr", operatorAdr);
				// return res.status(200).json({ operatorAdr });
			}

			default:
				res.setHeader("Allow", ["GET", "POST"]);
				res.status(405).end(`Method ${req.method} Not Allowed`);
		}
	} catch (error) {
		ErrorResponse(error, log, res);
	}
}

async function createCapTableRecord(name: string, orgnr: string) {
	let capTableAddress: string | undefined;
	let capTableDeployTransactionHash: string | undefined;

	try {
		const wallet = WALLET.connect(GET_PROVIDER());
		const transactionCount = await wallet.getTransactionCount();

		const deployTx = await new CapTable__factory(wallet).getDeployTransaction(name, orgnr, 1, CONTROLLERS, [
			DEFAULT_PARTITION,
		]);

		deployTx.maxFeePerGas = ethers.BigNumber.from("100000000");
		deployTx.maxPriorityFeePerGas = ethers.BigNumber.from("0");

		log(`created a new transaction to create captable with params name: ${name}, orgnr ${orgnr}, controlles: ${CONTROLLERS}, default partitions: ${DEFAULT_PARTITION}`)
		const signedTx = await wallet.sendTransaction(deployTx);
		capTableAddress = ethers.utils.getContractAddress({ from: wallet.address, nonce: transactionCount });
		log(`Captable deployed at ${capTableAddress} for org ${name} with tx ${signedTx.hash}`);
		capTableDeployTransactionHash = signedTx.hash;
	} catch (error) {
		const message = handleRPCError({ error });
		throw new ApiError(500, message);
	}

	return { capTableAddress, capTableDeployTransactionHash };
}

async function addCapTableRecordToCapTableRegistry(capTableAddress: string, orgnr: string) {
	let capTableRegistryTransactionHash: string | undefined;

	try {
		const wallet = WALLET.connect(GET_PROVIDER());
		const addNonce = await wallet.getTransactionCount('pending');

		const registry = await ConnectToCapTableRegistry_RW();
		log(`Adding captable ${capTableAddress} to registry with orgnr ${orgnr} and nonce ${addNonce}`);
		console.dir(registry);
		const signedTransaction = await registry.addCapTable(capTableAddress, orgnr, {
			nonce: addNonce,
			maxFeePerGas: ethers.BigNumber.from("100000000"),
			maxPriorityFeePerGas: ethers.BigNumber.from("0")
		});

		capTableRegistryTransactionHash = signedTransaction.hash;

		await new CapTable__factory(wallet)
			.attach(capTableAddress)
			.confirmAddedToRegistry(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);
	} catch (error) {
		const message = handleRPCError({ error });
		throw new ApiError(500, `Could not add CapTable to registry, ${message}`);
	}

	return capTableRegistryTransactionHash;
}

// TODO Include in endpoint or remove
async function getDetailsFromCapTables(captables: string[]): Promise<any[]> {
	const promise = captables.map(async (capTableAddress: string) => {
		const captable = await ConnectToCapTable_R(capTableAddress);

		return {
			orgnr: await captable.getOrgnr(),
			name: await captable.name(),
			ethAddress: capTableAddress,
		};
	});

	return Promise.all(promise);
}

function parseBody(body: any) {
	if (!("name" in body)) {
		throw new ApiError(400, "name missing");
	}

	if (!("orgnr" in body)) {
		throw new ApiError(400, "orgnr missing");
	}

	if (typeof body.name !== "string") {
		throw new ApiError(400, "name must be provided as a string");
	}

	if (body.name.length === 0) {
		throw new ApiError(400, "name cant be empty");
	}

	if (typeof body.orgnr !== "string") {
		throw new ApiError(400, 'orgnr must be provided as a string. e.g "112233445"');
	}

	if (body.orgnr.length !== 9) {
		throw new ApiError(400, "orgnr must be nine digits");
	}

	try {
		parseInt(body.orgnr.toString());
	} catch (error) {
		throw new ApiError(400, "orgnr must be a valid number");
	}

	const orgnr: string = body.orgnr.toString();
	const name: string = body.name.toString();

	return { name, orgnr };
}
