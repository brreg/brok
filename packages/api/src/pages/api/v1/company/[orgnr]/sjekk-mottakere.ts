import { CapTable } from "@brok/captable";
import debug from "debug";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";
import { ApiRequestLogger, ErrorResponse } from "../../../../../utils/api";
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R } from "../../../../../utils/blockchain";
import { getForetakByOrgnr, getWalletsForIdentifiers } from "../../../../../utils/navnetjener";

const log = debug("brok:api:v1:company:[id]:sjekk-mottakere");

type Data = {};


const isValidID = (id: string): boolean => {
	return (id.length === 11 || id.length === 9);
};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);
		switch (req.method) {
			case "POST": {
				// Incoming data
				const { orgnr } = parseQuery(req.query);
				const { mottkerIDer } = req.body;

				// Check the data
				if (!orgnr) {
					return res.status(400).json({ error: 'orgnr is missing or invalid' });
				}

				const invalidIDs = mottkerIDer.filter((id: string) => !isValidID(id));
				if (invalidIDs.length > 0) {
					const errorMessages = invalidIDs.map((id: string) => `${id} in mottkarIDer is invalid`);
					return res.status(400).json({ error: errorMessages.join(". ") });
				}

				// Prepare request data
				//   const identifiers = [NINA.IDENTIFIER, JONNY.IDENTIFIER]; // Nina (should be there) and Jonny (shoudn't be there)

				const walletResponse = await getWalletsForIdentifiers(mottkerIDer, orgnr);

				console.dir(walletResponse, 5);

				// Only return wallets that are not already registered
				const filteredRes = walletResponse.wallets.filter(wallet => wallet.walletAddress === null);

				return res.status(200).json(filteredRes);
			}

			default:
				res.setHeader("Allow", ["POST"]);
				res.status(405).end(`Method ${req.method} Not Allowed`);
		}
	} catch (error) {
		ErrorResponse(error, log, res);
	}
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
