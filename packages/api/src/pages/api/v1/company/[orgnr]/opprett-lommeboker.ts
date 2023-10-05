import debug from "debug";
import type { NextApiRequest, NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";
import { ApiRequestLogger, ErrorResponse } from "../../../../../utils/api";
import { createWalletRecord } from "../../../../../utils/navnetjener";

const log = debug("brok:api:v1:company:[id]:opprett-lommeboker");

type Data = {};


export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);

		switch (req.method) {
			case "POST": {
				// Incoming data
				const { orgnr } = parseQuery(req.query);
				const newNameServiceRecords = req.body;

				// Check the data
				if (!orgnr) {
					return res.status(400).json({ error: 'orgnr is missing or invalid' });
				}

				// TODO noe validation av newNameServiceRecords som skal være av typen WalletRecordInNavnetjener[]

				const nsRes = await createWalletRecord(newNameServiceRecords);

				return res.status(200).json(nsRes);
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
