import { ERC5564Registry__factory } from "@brok/captable";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from "../../../contants";
import { formatPublicKeyForSolidityBytes, getStealthAddress } from "../../../utils/stealth";
import debug from "debug";
const log = debug("brok:shareholder:register");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	switch (req.method) {
		case "GET":
			// show wallet for user
			res.status(200).json({});
			res.end();
			break;
		case "POST":
			if (!("signature" in req.body)) {
				return res.status(400).end("No signature provided in body");
			}
			if (!("spendPublicKey" in req.body)) {
				return res.status(400).end("No spendPublicKey provided in body");
			}
			const { signature, spendPublicKey } = req.body;
			const isValidSignature = (sig: string) => ethers.utils.isHexString(sig) && sig.length === 132;
			if (!isValidSignature(signature)) {
				return res.status(400).end(`Invalid signature: ${signature}`);
			}
			const digest = ethers.utils.arrayify(ethers.utils.hashMessage(spendPublicKey));
			const recoveredAddress = ethers.utils.recoverAddress(digest, signature);

			const wallet = WALLET.connect(GET_PROVIDER());
			const registry = new ERC5564Registry__factory(wallet).attach(CONTRACT_ADDRESSES.ERC5564_REGISTRY);

			// get address from ethereum signature
			log(`Checking address: ${recoveredAddress} for stealth keys`);
			const currentKeys = await registry.stealthKeys(wallet.address, CONTRACT_ADDRESSES.SECP256K1_GENERATOR);
			if ("spendingPubKey" in currentKeys && currentKeys.spendingPubKey !== "0x") {
				log("Keys already registered, returning address");
				return res.status(200).json({
					success: true,
					message: "Keys already registered",
				});
			} else {
				log("Keys not registered, start registering");
				const spendPublicKeyParsed = formatPublicKeyForSolidityBytes(spendPublicKey);
				const viewPublicKeyParsed = "0x11";

				const tx = await registry.registerKeysOnBehalf(
					recoveredAddress,
					CONTRACT_ADDRESSES.SECP256K1_GENERATOR,
					"0x11",
					spendPublicKeyParsed,
					viewPublicKeyParsed,
				);
				const receipt = await tx.wait();
				return res.status(200).json({
					success: true,
					message: `Registered keys for ${recoveredAddress} with receipt ${receipt.transactionHash}`,
				});
			}
		default:
			res.setHeader("Allow", ["GET", "POST"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}

registerStealthKeys: async (spendPublicKey: string, viewPublicKey: string) => {
	try {
		log("registering keys...");
		const wallet = get().wallet;
		if (!wallet) {
			throw new Error("Wallet not set, can't register keys");
		}
		const registry = new ERC5564Registry__factory(wallet).attach(CONTRACT_ADDRESSES.ERC5564_REGISTRY);
		log("Checking address for keys", wallet.address);
		log("With generator", CONTRACT_ADDRESSES.SECP256K1_GENERATOR);
		const currentKeys = await registry.stealthKeys(wallet.address, CONTRACT_ADDRESSES.SECP256K1_GENERATOR);
		log("currentKeys", currentKeys);
		if (!currentKeys || currentKeys.spendingPubKey === "0x") {
			log("keys not registered, registering...");
			const spendPublicKeyParsed = formatPublicKeyForSolidityBytes(spendPublicKey);
			const viewPublicKeyParsed = formatPublicKeyForSolidityBytes(viewPublicKey);

			const tx = await registry.registerKeys(
				CONTRACT_ADDRESSES.SECP256K1_GENERATOR,
				spendPublicKeyParsed,
				viewPublicKeyParsed,
			);
			const receipt = await toast.promise(tx.wait(), {
				error: "Failed to register keys",
				success: "Registered keys",
				pending: "Registering keys",
			});
			log("Registered keys, receipt:", receipt);
		} else {
			log("keys already registered");
		}
	} catch (error) {
		log("Failed to register keys", error);
		return false;
	}
	return true;
};
