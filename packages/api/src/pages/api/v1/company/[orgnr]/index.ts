import { CapTable } from "@brok/captable";
import debug from "debug";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";
import { ApiRequestLogger, ErrorResponse } from "../../../../../utils/api";
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R } from "../../../../../utils/blockchain";
import { getForetakByOrgnr } from "../../../../../utils/navnetjener";

const log = debug("brok:api:v1:company:[id]");
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
				// Find info about company
				const { orgnr } = parseQuery(req.query);
				const foretak = await getForetakByOrgnr(orgnr);
				log(`HTTP Response 200, return foretak with orgnr ${orgnr}`);
				return res.status(200).json({ foretak });
			}

			case "PUT": {
				// TODO Update captable info for company
			}

			case "DELETE": {
				// TODO Delete company
			}

			default:
				res.setHeader("Allow", ["GET", "PUT", "DELETE"]);
				res.status(405).end(`Method ${req.method} Not Allowed`);
		}
	} catch (error) {
		ErrorResponse(error, log, res);
	}
}

async function findCapTableWithOrgnr(orgnr: string): Promise<CapTable | undefined> {
	const capTableRegistry = await ConnectToCapTableRegistry_R();
	const allCapTablesAddresses = await capTableRegistry.getCapTableList();

	for (const address of allCapTablesAddresses) {
		const captable = await ConnectToCapTable_R(address);
		const nr = await captable.getOrgnr();
		if (orgnr === nr) {
			return captable;
		}
	}
}

function parseBody(body: any) {
	if (!("signature" in body)) {
		throw new ApiError(400, "No signature provided in body");
	}

	if (!("spendPublicKey" in body)) {
		throw new ApiError(400, "No spendPublicKey provided in body");
	}

	const signature: string = body.signature.toString();
	const spendPublicKey: string = body.spendPublicKey.toString();
	const isValidSignature = (sig: string) => ethers.utils.isHexString(sig) && sig.length === 132;

	if (!isValidSignature(signature)) {
		throw new ApiError(400, `Invalid signature: ${signature}`);
	}

	return { signature, spendPublicKey };
}

function parseQuery(
	query: Partial<{
		[key: string]: string | string[];
	}>,
) {
	if (!query.orgnr) {
		throw new ApiError(400, "missing orgnr");
	}
	if (typeof query.orgnr !== "string") {
		throw new ApiError(400, 'orgnr must be provided as a string. e.g "112233445"');
	}
	if (query.orgnr.length !== 9) {
		throw new ApiError(400, "orgnr must be nine digits");
	}
	try {
		parseInt(query.orgnr.toString());
	} catch (error) {
		throw new ApiError(400, "orgnr must be a valid number");
	}
	const orgnr: string = query.orgnr.toString();
	return { orgnr };
}
