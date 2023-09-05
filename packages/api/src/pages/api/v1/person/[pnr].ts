import { debug } from "debug";
import { NextApiRequest, NextApiResponse } from "next";
import { ApiRequestLogger, ErrorResponse } from "../../../../utils/api";
import { ApiError } from "next/dist/server/api-utils";
import { getForetakByPnr } from "../../../../utils/navnetjener";

const log = debug("brok:api:v1:person");

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
	try {
		ApiRequestLogger(req, log);
		switch (req.method) {
			case "GET": {
				// Find all foretak for a person
        const { pnr } = parseQuery(req.query);
        const foretak = await getForetakByPnr(pnr);
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
	if (!query.pnr) {
		throw new ApiError(400, "missing pnr");
	}
	if (query.pnr.length !== 11 && process.env.NODE_ENV === "production") {
		throw new ApiError(400, "pnr must be eleven digits");
	}
	try {
		parseInt(query.pnr.toString());
	} catch (error) {
		throw new ApiError(400, "pnr must be a valid number");
	}
	const pnr: string = query.pnr.toString();
	return { pnr };
}