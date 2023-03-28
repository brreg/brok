import { CapTable, CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
import { ethers } from "ethers";
import debug from "debug";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, SPEND_KEY, WALLET } from "../../contants";
import { getStealthAddress } from "../../utils/stealth";
import { handleRPCError } from "../../utils/blockchain";

type Data = {};
const log = debug("brok:api:capTable");

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	switch (req.method) {
		case "GET":
			// show wallet for user
			res.status(200).json({});
			res.end();
			break;
		case "POST":
			// body should contain name ensure this values are set and correct
			const wallet = WALLET.connect(GET_PROVIDER());
			log("Input: name:", req.body.name);
			log("Input: orgnr:", req.body.orgnr);
			const name = req.body.name.toString()
			const orgnr = req.body.orgnr.toString();

			let capTableDeployTransactionHash: string | undefined;
			let capTableRegistryTransactionHash: string | undefined;
			let capTableAddress: string | undefined;
			try {
				const transactionCount = await wallet.getTransactionCount();
				const deployTx = await new CapTable__factory().getDeployTransaction(
					name,
					orgnr,
					ethers.utils.parseEther("1"),
					CONTROLLERS,
					[DEFAULT_PARTITION],
					CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY,
				);
				const signedTx = await wallet.sendTransaction(deployTx);
				capTableAddress = ethers.utils.getContractAddress({ from: wallet.address, nonce: transactionCount });
				log(`Captable should deploy at ${capTableAddress} for org ${name} with tx ${signedTx.hash}`);

				capTableDeployTransactionHash = signedTx.hash;
			} catch (error) {
				const message = handleRPCError({ error });
				return res
					.status(500)
					.json({
						error: `Could not create a new transaction for creating Captable for org ${name}`,
						message: message,
					});
			}

			try {
				if (!capTableAddress) {
					throw new Error("Captable address is not set");
				}
				log("capTableAddress", capTableAddress);
				const signedTx = await new CapTableRegistry__factory(wallet)
					.attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY)
					.addCapTable(capTableAddress, orgnr.toString());
				capTableRegistryTransactionHash = signedTx.hash;
			} catch (error) {
				const message = handleRPCError({ error });
				return res.status(500).json({ error: `Could not add Captable to registry`, message: message });
			}

			res.status(200).json({
				capTableAddress: capTableAddress,
				capTableDeployTransactionHash: capTableDeployTransactionHash,
				capTableRegistryTransactionHash: capTableDeployTransactionHash,
			});
			res.end();
			break;
		default:
			res.setHeader("Allow", ["GET", "POST"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}
