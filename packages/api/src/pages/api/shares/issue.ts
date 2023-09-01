import { CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, WALLET } from "../../../contants";
import { handleRPCError } from "../../../utils/blockchain";
import debug from "debug";
import { ApiError } from "next/dist/server/api-utils";
import { ApiRequestLogger } from "../../../utils/api";

type Data = {};
const log = debug("brok:api:shares:issue");

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	ApiRequestLogger(req, log);
	switch (req.method) {
		case "GET":
			// show wallet for user
			res.status(200).json({ message: "ok" });
			res.end();
			break;
		case "POST":
			try {
				const { addressToReceiveTokens, capTableAddress, amount } = parseBody(req.body);

				const provider = GET_PROVIDER();
				const wallet = WALLET.connect(provider);
				const capTable = await new CapTable__factory(wallet).attach(capTableAddress);

				const capTableRegistry = await new CapTableRegistry__factory(wallet).attach(
					CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY,
				);

				try {
					const randomEthereumWallet = ethers.Wallet.createRandom();

					// TODO Consider if they need to be authenticated/verified
					// const authenticateStealth_transaction = await capTableRegistry.setAuthenticatedPerson(stealthAddress);
					// const receipt = await authenticateStealth_transaction.wait();

					log(
						`trying to issue ${amount} shares to ${randomEthereumWallet.address} for CapTable with address ${capTable.address}`,
					);
					const result = await capTable.issue(
						addressToReceiveTokens,
						ethers.utils.parseEther(amount.toString()),
						ethers.constants.HashZero,
					);

					log(
						`HTTP Response 200, successfully created transaction to ${randomEthereumWallet.address} with hash ${result.hash}`,
					);

					return res.status(200).json({
						transaction: {
							created: true,
							completed: false,
							hash: result.hash,
						},
						message: "created a new transaction",
					});
				} catch (error) {
					const e = handleRPCError(error);
					log(`HTTP Response 500, could not create transaction, see error for more details: ${e}`);
					return res.status(500).json({
						error: e,
						transaction: null,
						message: "could not create transaction, see error for more details",
					});
				}
			} catch (error) {
				if (error instanceof ApiError) {
					log(`HTTP Response ${error.statusCode}, ${error.message} ${error}`);

					return res.status(error.statusCode).json({
						error: error,
						transaction: null,
						message: error.message,
					});
				}

				log(`HTTP Response 500, could not create transaction, see error for more details: ${error}`);

				return res.status(500).json({
					error: error,
					transaction: null,
					message: "could not create transaction, see error for more details",
				});
			}
		default:
			log(`HTTP Response 405, Method ${req.method} Not Allowed`);
			res.setHeader("Allow", ["GET", "POST"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}

async function checkIfWalletAddressIsVerifiedToHoldBrokTokens(addressToReceiveTokens: string) {
	const registry = new CapTableRegistry__factory()
		.attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY)
		.connect(GET_PROVIDER());
	const isVerified = await registry.checkAuthenticatedOnce(addressToReceiveTokens);
	if (!isVerified) {
		throw new ApiError(
			400,
			`addressToReceiveTokens ${addressToReceiveTokens} is not registered in the system, try verifying the address by using /api/shareholder/verify`,
		);
	}
}

function parseBody(body: any): RequestBody {
	if (!("addressToReceiveTokens" in body && "capTableAddress" in body && "amount" in body)) {
		throw new ApiError(400, "missing addressToReceiveTokens, capTableAddress or amount in body");
	}

	const addressToReceiveTokens = body.addressToReceiveTokens.toString();
	const capTableAddress = body.capTableAddress.toString();
	const amount = parseInt(body.amount);
	if (typeof amount !== "number") {
		throw new ApiError(400, `amount must be a number, received ${typeof amount}`);
	}

	return { addressToReceiveTokens, capTableAddress, amount };
}

type RequestBody = {
	addressToReceiveTokens: string;
	capTableAddress: string;
	amount: number;
};
