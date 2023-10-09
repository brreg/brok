import { debug } from "debug";
import { NextApiRequest, NextApiResponse } from "next";
import { ApiRequestLogger, ErrorResponse } from "../../../../utils/api";
import { ApiError } from "next/dist/server/api-utils";
import { getForetakOwnedByFnrOrOrgnr } from "../../../../utils/navnetjener";

const log = debug("brok:api:v1:person:[fnr]");

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
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
				// Find all foretak for a person
				const { fnr } = parseQuery(req.query);
				const foretak = await getForetakOwnedByFnrOrOrgnr(fnr);
				log(`HTTP Response 200, return ${foretak.length} foretak`);
				return res.status(200).json({ foretak });
			}
			default:
				res.setHeader("Allow", ["GET"]);
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
	if (!query.fnr) {
		throw new ApiError(400, "missing fnr");
	}
	if (query.fnr.length !== 11 && process.env.NODE_ENV === "production") {
		throw new ApiError(400, "fnr must be eleven digits");
	}
	try {
		parseInt(query.fnr.toString());
	} catch (error) {
		throw new ApiError(400, "fnr must be a valid number");
	}
	const fnr: string = query.fnr.toString();
	return { fnr };
}