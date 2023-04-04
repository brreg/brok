import { CapTableRegistry__factory,  } from "@brok/captable";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from "../../contants";
import { getStealthAddress } from "../../utils/stealth";
import debug from "debug";
import { ApiRequestLogger } from "../../utils/api";

type Data = {};
const log = debug("brok:api:health");

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	ApiRequestLogger(req, log)
	switch (req.method) {
		case "GET":
			// show wallet for user
			try {
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
		default:
			res.setHeader("Allow", ["GET"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}
