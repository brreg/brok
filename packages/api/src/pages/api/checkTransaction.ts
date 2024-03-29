import debug from "debug";
import type { NextApiRequest, NextApiResponse } from "next";
import { GET_PROVIDER } from "../../contants";
import { ApiRequestLogger } from "../../utils/api";

type Data = {};
const log = debug("brok:api:checkTransaction");

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	ApiRequestLogger(req, log);

	res.setHeader("Access-Control-Allow-Origin", "*");
	res.setHeader("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
	res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization");

	if (req.method === "OPTIONS") {
		return res.status(200).end();
	}

	switch (req.method) {
		case "GET":
			try {
				if (!req.query.transactionHash) {
					log("missing transactionHash in body");
					return res.status(400).json({ error: "missing txHash in body" });
				}

				const provider = GET_PROVIDER();
				const transaction = await provider.getTransaction(req.query.transactionHash.toString());
				const transactionReceipt = await provider.getTransactionReceipt(req.query.transactionHash.toString());

				const transactionDetails = {
					to: transactionReceipt.to,
					from: transactionReceipt.from,
					transactionHash: transactionReceipt.transactionHash,
					confirmations: transactionReceipt.confirmations,
					status: transactionReceipt.status,
				};

				if (transaction.confirmations > 0 && transactionReceipt.status) {
					const completed = true;
					const succeeded = true;
					const message = "transaction completed successfully";
					log(
						"\nhttp return 200",
						"\ncompleted",
						completed,
						"\nsucceeded",
						succeeded,
						"\nmessage",
						message,
						"\ntransactionDetails",
						transactionDetails,
					);

					return res.status(200).json({
						completed,
						succeeded,
						message,
						transactionDetails,
					});
				}

				// TODO Consider removing
				if (false) {
					const completed = true;
					const succeeded = false;
					const message = "transaction is still waiting to be completed";
					log(
						"\nhttp return 200",
						"\ncompleted",
						completed,
						"\nsucceeded",
						succeeded,
						"\nmessage",
						message,
						"\ntransactionDetails",
						transactionDetails,
					);
					return res.status(200).json({
						completed: true,
						succeeded: false,
						message: "transaction failed to complete",
						errorMessage: "TODO",
					});
				}

				const completed = false;
				const succeeded = null;
				const message = "transaction is still waiting to be completed";
				log(
					"\nhttp return 200",
					"\ncompleted",
					completed,
					"\nsucceeded",
					succeeded,
					"\nmessage",
					message,
					"\ntransactionDetails",
					transactionDetails,
				);

				return res.status(200).json({
					completed,
					succeeded,
					message,
					transactionDetails,
				});
			} catch (error) {
				const completed = true;
				const succeeded = false;
				// log("error:", error);

				return res.status(500).json({
					completed,
					succeeded,
					message: "Something went wrong, please try again later",
					error: error,
				});
			}
		default:
			res.setHeader("Allow", ["GET"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}
