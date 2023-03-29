import { CapTableRegistry__factory } from "@brok/captable";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from "../../../contants";
import { getStealthAddress } from "../../../utils/stealth";
import debug from "debug";
import { ethers } from "ethers";
import ApiRequestLogger from "../../../utils/apiRequestLogger";

type Data = {};
const log = debug("brok:api:shareholder:verify");

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	ApiRequestLogger(req, log)
	switch (req.method) {
		case "POST":
			log(`HTTP ${req.method} ${req.url}\nbody:`, req.body)
			try {
				if(!("address" in req.body)){
					return res.status(400).end("No address in body")	
				}
				let address : string | undefined = undefined
				try {
					 address  = ethers.utils.getAddress(req.body.address);
				} catch (error) {
					return res.status(400).end("Invalid address in body")	
				}
				if(!address){
					return res.status(400).end("Unknown error while parsing address in body")
				}

				// connect to CapTableRegistry
				// check if fagsystem is aproved to do changes
				// check if address from request body is verified, e.a. exsist in a whitelist in VC registry
				// if address is not whitelisted, the address will be added to the whitelist
				// returns ---

				const wallet = WALLET.connect(GET_PROVIDER());
				log("wallet:", wallet.address);
				const registry = new CapTableRegistry__factory(wallet).attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);
				const operatorRole =  await registry.OPERATOR_ROLE();
				const isAuthorized = await registry.hasRole(operatorRole, wallet.address);
				log("checkAuth:", isAuthorized);
				if (!isAuthorized) {
					return res.status(500).json({
						status: "fail",
						message: "Wallet is not authorized to perform transactions, are this enterprise system wallet registerd system in BRØK?",
						isAuthorized: isAuthorized
					})
				}
				const isVerfiedFirstCheck = await registry.checkAuthenticatedOnce(address);
				log("isVerfied:", isVerfiedFirstCheck);
				if (isVerfiedFirstCheck) {
					return res.status(200).json({
						status: "success",
						message: "You are allready verified",
						isVerfied: isVerfiedFirstCheck
					})
				}else {
					const tx = await registry.setAuthenticatedPerson(address);
					log("tx:", tx);
					const receipt = await tx.wait();
					log("receipt:", receipt);
				}
				const isVerfiedSecondCheck = await registry.checkAuthenticatedOnce(address);
				log("isVerfied:", isVerfiedSecondCheck);
				if (isVerfiedSecondCheck) {
					return res.status(200).json({
						status: "success",
						message: "You are now verified",
						isVerfied: isVerfiedSecondCheck
					})
				}else {
					return res.status(500).json({
						status: "fail",
						message: "Something went wrong, please try again later",
						isVerfied: isVerfiedSecondCheck
					})
				}

			}catch(error){
				log("error:", error);
				return res.status(500).json({
					status: "fail",
					message: "Something went wrong, please try again later",
					error: error
				})
			}
		case "GET":
			log(`HTTP ${req.method} ${req.url}\nquery:`, req.query)
			if ("address" in req.query) {
				const address = req.query.address?.toString()
				const registry = new CapTableRegistry__factory().attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY).connect(GET_PROVIDER());
				const isVerfied = await registry.checkAuthenticatedOnce(address!);
				return res.status(200).json({ isVerfied })
			}

			return res.status(500).json({ krise: true })
		default:
			res.setHeader("Allow", ["GET", "POST"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}
