import { CapTableRegistry__factory } from "@brok/captable";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from "../../../contants";
import { getStealthAddress } from "../../../utils/stealth";
import debug from "debug";
import { ethers } from "ethers";
const log = debug("brok:api:verify");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	switch (req.method) {
		case "POST":
			try {
				if(!("address" in req.body) || ethers.utils.){
					return res.status(400).end("No address in body")	
				}
				const wallet = WALLET.connect(GET_PROVIDER());
				log("wallet:", wallet.address);
				const registry = new CapTableRegistry__factory(wallet).attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);
				const operatorRole =  await registry.OPERATOR_ROLE();
				const isAuthorized = await registry.hasRole(operatorRole, wallet.address);
				log("checkAuth:", isAuthorized);
				if (!isAuthorized) {
					return res.status(500).json({
						status: "fail",
						message: "Wallet is not authorized to perform transactions, are you a registerd system in BRØK?",
						isAuthorized: isAuthorized
					})
				}
				return res.status(200).json({
					status: "ok",
					address: wallet.address,
					registryAddress: registry.address,
				})
			}catch(error){
				log("error:", error);
				return res.status(500).json({
					status: "fail",
					message: "Something went wrong, please try again later",
					error: error
				})
			}
		case "GET":
			if ("address" in req.body) {
				// connect vc registry -> set authenticate
				//
			}

			res.status(200).json({
				address: "TODO",
			});
			res.end();
			break;
		default:
			res.setHeader("Allow", ["GET", "POST"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}
