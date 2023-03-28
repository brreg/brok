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
        const transaction = await provider.getTransaction(req.query.transactionHash.toString())
        const transactionReceipt = await provider.getTransactionReceipt(req.query.transactionHash.toString())

        const transactionDetails = {
          to: transactionReceipt.to,
          from: transactionReceipt.from,
          transactionHash: transactionReceipt.transactionHash,
          confirmations: transactionReceipt.confirmations,
          status: transactionReceipt.status,
        }
        
        if (transaction.confirmations > 0 && transactionReceipt.status) {
          const completed = true
          const succeeded = true
          const message = "transaction completed successfully"
          log("\nhttp return 200", "\ncompleted", completed, "\nsucceeded", succeeded, "\nmessage", message, "\ntransactionDetails", transactionDetails)
          return res.status(200).json({
            completed,
            succeeded,
            message,
            transactionDetails
          })
        }
        
        if (false) {
          const completed = true
          const succeeded = false
          const message = "transaction is still waiting to be completed"
          log("\nhttp return 200", "\ncompleted", completed, "\nsucceeded", succeeded, "\nmessage", message, "\ntransactionDetails", transactionDetails)
          return res.status(200).json({
            completed: true,
            succeeded: false,
            message: "transaction failed to complete",
            errorMessage: "TODO"
          })
        }
        
        const completed = false
        const succeeded = null
        const message = "transaction is still waiting to be completed"
        log("\nhttp return 200", "\ncompleted", completed, "\nsucceeded", succeeded, "\nmessage", message, "\ntransactionDetails", transactionDetails)
				return res.status(200).json({
					completed,
          succeeded,
          message,
          transactionDetails
				})
			}catch(error){
        const completed = true
        const succeeded = false
				log("error:", error);
				return res.status(500).json({
					completed,
          succeeded,
					message: "Something went wrong, please try again later",
					error: error
				})
			}
		default:
			res.setHeader("Allow", ["GET"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}