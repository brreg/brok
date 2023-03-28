import { CapTable, CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
import { ethers } from "ethers";
import debug from "debug";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, SPEND_KEY, WALLET } from "../../contants";
import { getStealthAddress } from "../../utils/stealth";
import { handleRPCError } from "../../utils/blockchain";

type Data = {};
const log = debug("brok:api:checkTransaction")

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
  switch (req.method) {
		case "GET":
			try {
        if (!req.query.transactionHash) {
          log("missing transactionHash in body")
          return res.status(400).json({ error: "missing txHash in body"})
        }
        const provider = GET_PROVIDER()
        const transactionHash = await provider.getTransaction(req.query.transactionHash.toString())
        // log("transactionHash:", transactionHash)
        const transactionReceipt = await provider.getTransactionReceipt(req.query.transactionHash.toString())

        log("\n-----------------\nTransaction\nstatus:", transactionReceipt,"\n-----------------")
        
        if (transactionHash.confirmations > 0 && transactionReceipt.status) {
          log("return 200, status completed")
          return res.status(200).json({
            status: "completed",
            message: "transaction completed successfully"
          })
        }
        
        if (false) {
          return res.status(200).json({
            status: "failed",
            message: "transaction completed successfully"
          })
        }
        
        log("return 200, status pending")
				return res.status(200).json({
					status: "pending",
          message: "transaction is still waiting to be completed"
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