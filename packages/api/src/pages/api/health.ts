import { CapTableRegistry__factory } from "@brok/captable";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, WALLET } from "../../contants";
import debug from "debug";
import { ApiRequestLogger } from "../../utils/api";
import { checkIfNavnetjenerIsHealthy } from "../../utils/navnetjener";

type Data = {};
const log = debug("brok:api:health");

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	ApiRequestLogger(req, log);


	res.setHeader('Access-Control-Allow-Origin', '*');
	res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
	res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');


	if (req.method === 'OPTIONS') {
		return res.status(200).end();
	}


	switch (req.method) {
		case "GET":
			// show wallet for user
			try {
				const wallet = WALLET.connect(GET_PROVIDER());
				log("wallet:", wallet.address);
				console.log("CONTRACT ADDRESS", CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);
				const registry = new CapTableRegistry__factory(wallet).attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);
				const operatorRole = await registry.OPERATOR_ROLE();
				const isAuthorized = await registry.hasRole(operatorRole, wallet.address);
				const navnetjenerAlive = await checkIfNavnetjenerIsHealthy()
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
					"connected to navnetjener": navnetjenerAlive
				})
			} catch (error) {
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
