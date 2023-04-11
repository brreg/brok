import { CapTable, CapTable__factory, ERC5564Registry__factory } from "@brok/captable";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import {
	CONTRACT_ADDRESSES,
	CONTROLLERS,
	DEFAULT_PARTITION,
	GET_PROVIDER,
	SPEND_KEY,
	WALLET,
} from "../../../../contants";
import {
	formatPublicKeyForSolidityBytes,
	getStealthAddress,
	getAnnoncements,
	getSharedSecret,
	getRecoveryPrivateKey,
	signatureToStealthKeys,
} from "../../../../utils/stealth";
import debug from "debug";
import { ApiError } from "next/dist/server/api-utils";
import {
	ConnectToCapTableRegistry_R,
	ConnectToCapTableRegistry_RW,
	ConnectToCapTable_R,
	ConnectToStealthAddressFactory_RW,
	handleRPCError,
} from "../../../../utils/blockchain";
import { ErrorResponse, ApiRequestLogger } from "../../../../utils/api";

const log = debug("brok:api:v1:company");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);
		switch (req.method) {
			case "GET": {
				// Find all companies
				const capTableRegistry = await ConnectToCapTableRegistry_R();
				const allCapTablesAddresses = await capTableRegistry.getCapTableList();
				const allCapTables = await getDetailsFromCapTables(allCapTablesAddresses);

				log(`found ${allCapTables.length} CapTables`);
				log(`HTTP Response 200, return list with ${allCapTables.length} captables`);
				return res.status(200).json({ allCapTables });
			}
			case "POST": {
				// Register a new captable for company
				const { name, orgnr } = parseBody(req.body);
				const { capTableAddress, capTableDeployTransactionHash } = await createCapTableRecord(name, orgnr);

				if (!capTableAddress) {
					throw new ApiError(500, "Captable address is not set");
				}
				const transactionHash = await addCapTableRecordToCapTableRegistry(capTableAddress, orgnr);

				log("HTTP Response 200, created captable with transactionHash", transactionHash);
				return res.status(200).json({
					capTableAddress: capTableAddress,
					capTableDeployTransactionHash: capTableDeployTransactionHash,
					capTableRegistryTransactionHash: transactionHash,
				});
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
		const deployTx = await new CapTable__factory().getDeployTransaction(
			name,
			orgnr,
			ethers.utils.parseEther("1"),
			CONTROLLERS,
			[DEFAULT_PARTITION],
			CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY,
		);
		const signedTx = await wallet.sendTransaction(deployTx);
		capTableAddress = ethers.utils.getContractAddress({ from: wallet.address, nonce: transactionCount });
		log(`Captable should deploy at ${capTableAddress} for org ${name} with tx ${signedTx.hash}`);

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
		const registry = await ConnectToCapTableRegistry_RW();
		const signedTransaction = await registry.addCapTable(capTableAddress, orgnr);
		capTableRegistryTransactionHash = signedTransaction.hash;
	} catch (error) {
		const message = handleRPCError({ error });
		throw new ApiError(500, `Could not add CapTable to registry, ${message}`);
	}

	return capTableRegistryTransactionHash;
}

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
