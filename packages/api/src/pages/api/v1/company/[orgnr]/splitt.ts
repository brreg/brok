import { CapTable } from "@brok/captable";
import debug from "debug";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";
import { ApiRequestLogger, ErrorResponse } from "../../../../../utils/api";
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R } from "../../../../../utils/blockchain";
import { GET_PROVIDER, WALLET } from "../../../../../contants";

const log = debug("brok:api:v1:company:[id]/splitt");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);

		switch (req.method) {
			case "POST": {
				// Find info about company
				const { orgnr } = parseQuery(req.query);
				const { aksjeklasser, mottakerAdresser, antall } = req.body;

				const captable = await findCapTableWithOrgnr(orgnr);

				if (!captable) {
					throw new ApiError(404, `Could not find any company with orgnr ${orgnr} in BRØK`);
				}

				const wallet = WALLET.connect(GET_PROVIDER());
				const captable_RW = captable.connect(wallet);

				// Convert each element in aksjeklasser array to bytes32 format
				const partitions = aksjeklasser.map((klass: string) => ethers.utils.formatBytes32String(klass));

				// Issue
				await captable_RW.splitt(partitions, mottakerAdresser, antall, "0x11");

				const sum: number = antall.reduce(
					(accumulator: number, currentValue: string) => accumulator + parseInt(currentValue, 10),
					0,
				);

				return res.status(200).json({
					message: `Successfully issued ${sum} new shares to ${mottakerAdresser.length} addresses`,
				});
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
