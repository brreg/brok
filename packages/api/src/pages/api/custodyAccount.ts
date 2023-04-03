import type { NextApiRequest, NextApiResponse } from "next";
import { GET_PROVIDER, SPEND_KEY, WALLET } from "../../contants";
import { getStealthAddress } from "../../utils/stealth";
import debug from "debug";
import { ApiRequestLogger } from "../../utils/api";

type Data = {};
const log = debug("brok:api:custodyAccounts")

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	ApiRequestLogger(req, log)
	switch (req.method) {
		case "GET":
			// show wallet for user
			res.status(200).json({});
			res.end();
			break;
		case "POST":
			const spendKey = await SPEND_KEY();
			const stealthAddress = getStealthAddress(spendKey.publicKey.slice(2), BigInt(23));
			// create wallet

			// input:
			//    - private key?

			//

			// output:
			//    - wallet
			res.status(200).json({
				address: stealthAddress,
			});
			res.end();
			break;
		default:
			res.setHeader("Allow", ["GET", "POST"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}
