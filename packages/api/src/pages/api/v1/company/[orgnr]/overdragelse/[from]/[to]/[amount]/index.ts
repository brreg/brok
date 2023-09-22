import { CapTable } from "@brok/captable";
import debug from "debug";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";
import { ApiRequestLogger, ErrorResponse } from "../../../../../../../../../utils/api";
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R } from "../../../../../../../../../utils/blockchain";
import { GET_PROVIDER, WALLET } from "../../../../../../../../../contants";
import { getWalletsForIdentifiers } from "../../../../../../../../../utils/navnetjener";

const log = debug("brok:api:v1:company:[id]:overdragelse:fra:[fnr/orgnr]:til:[fnr/orgnr]/antall/[antall]");

type Data = {};

// TODO Er det et problem at vi bruker 0x11 overalt?
export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);

		switch (req.method) {
			case "POST": {
				const { orgnr, from, to, amount } = parseQuery(req.query);
				const captable = await findCapTableWithOrgnr(orgnr);

				if (!captable) {
					throw new ApiError(404, `Could not find any company with orgnr ${orgnr} in BRØK`);
				}

				const wallet = WALLET.connect(GET_PROVIDER());
				const captable_RW = captable.connect(wallet);

				const aksjeklasse = "ordinære";
				const partition = ethers.utils.formatBytes32String(aksjeklasse);

				const resWallets = await getWalletsForIdentifiers([from, to], orgnr);
				const fromAdr = resWallets.wallets[0].walletAddress as string;
				const toAdr = resWallets.wallets[1].walletAddress as string;

				await captable_RW.operatorTransferByPartition(partition, fromAdr, toAdr, amount, "0x11", "0x11");

				return res.status(200).json({
					message: `Successfully transferred ${amount} shares`,
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
		orgnr: string;
		from: string;
		to: string;
		amount: string | number;
	}>,
) {
	if (!query.orgnr) {
		throw new ApiError(400, "missing orgnr");
	}
	if (typeof query.orgnr !== "string") {
		throw new ApiError(400, 'orgnr must be provided as a string');
	}

	if (!query.from || typeof query.from !== "string") {
		throw new ApiError(400, "missing or invalid 'from' parameter");
	}

	if (!query.to || typeof query.to !== "string") {
		throw new ApiError(400, "missing or invalid 'to' parameter");
	}

	if (!query.amount || (typeof query.amount !== "string" && typeof query.amount !== "number")) {
		throw new ApiError(400, "missing or invalid 'amount' parameter");
	}

	const orgnr: string = query.orgnr.toString();
	const from: string = query.from.toString();
	const to: string = query.to.toString();
	const amount: number = Number(query.amount);

	return { orgnr, from, to, amount };
}

