import { ERC5564Registry__factory } from "@brok/captable";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from "../../../contants";
import {
	formatPublicKeyForSolidityBytes,
	getStealthAddress,
	getAnnoncements,
	getSharedSecret,
	getRecoveryPrivateKey,
	signatureToStealthKeys,
} from "../../../utils/stealth";
import debug from "debug";
import { ApiError } from "next/dist/server/api-utils";
import { ConnectToStealthAddressFactory_RW } from "../../../utils/blockchain";
import { ErrorResponse, ApiRequestLogger } from "../../../utils/api";
import { getCapTablesForAddresses } from "../../../utils/the-graph";

const log = debug("brok:shareholder:register");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);
		switch (req.method) {
			case "GET": {
				// Find all shares belonging to the user, by analyzing users signature

				const { signature } = parseQuery(req.query);
				const { spend, view } = signatureToStealthKeys(signature);
				const stealthAddresses = await findStealthAddresses(spend.privateKey);

				if (stealthAddresses.length === 0) {
					log("HTTP Response 200, Could not find any stealth addresses");
					return res.status(200).json({
						success: true,
						message: "Could not find any stealth addresses with this signature",
					});
				}
				const graphData = await getCapTablesForAddresses(stealthAddresses);
				log(graphData);

				log("HTTP Response 200, found stealth addresses:", stealthAddresses);
				return res.status(200).json({
					success: true,
					message: "Found some addresses",
					stealthAddresses,
				});
			}
			// ---
			case "POST": {
				// Register new user's wallet, so stealth addresses can be created from it

				const { signature, spendPublicKey } = parseBody(req.body);

				// a digested spendPublicKey + signature should return user wallet address
				const digest = ethers.utils.arrayify(ethers.utils.hashMessage(spendPublicKey));
				const recoveredAddress = ethers.utils.recoverAddress(digest, signature);

				const isRegistered = await checkIfWalletIsRegisteredForStealth(recoveredAddress);

				if (isRegistered) {
					log("HTTP Response 200, keys already registered");
					return res.status(200).json({
						success: true,
						message: "Keys already registered",
					});
				}

				const receipt = await registerWalletForStealth(recoveredAddress, spendPublicKey);
				log(`HTTP Response 200, Registered keys for ${recoveredAddress} with receipt ${receipt.transactionHash}`);
				return res.status(200).json({
					success: true,
					message: `Registered keys for ${recoveredAddress} with receipt ${receipt.transactionHash}`,
				});
			}
			default:
				res.setHeader("Allow", ["GET", "POST"]);
				res.status(405).end(`Method ${req.method} Not Allowed`);
		}
	} catch (error) {
		ErrorResponse(error, log, res);
	}
}

async function checkIfWalletIsRegisteredForStealth(wallet: string): Promise<boolean> {
	const registry = ConnectToStealthAddressFactory_RW();

	log(`Checking address: ${wallet} for stealth keys`);
	const currentKeys = await registry.stealthKeys(wallet, CONTRACT_ADDRESSES.SECP256K1_GENERATOR);
	log("current keys", currentKeys);
	if ("spendingPubKey" in currentKeys && currentKeys.spendingPubKey !== "0x") {
		log("wallet already registered");
		return true;
	}
	log("wallet is not registered");
	return false;
}

async function registerWalletForStealth(wallet: string, spendPublicKey: string) {
	const registry = ConnectToStealthAddressFactory_RW();

	const spendPublicKeyParsed = formatPublicKeyForSolidityBytes(spendPublicKey);
	const viewPublicKeyParsed = "0x11"; // TODO - Start using view keys

	const tx = await registry.registerKeysOnBehalf(
		wallet,
		CONTRACT_ADDRESSES.SECP256K1_GENERATOR,
		"0x11",
		spendPublicKeyParsed,
		viewPublicKeyParsed,
	);
	const receipt = await tx.wait();

	return receipt;
}

async function findStealthAddresses(spendPrivateKey: string): Promise<string[]> {
	const stealthAddresses: string[] = [];
	const provider = GET_PROVIDER();
	const announcements = await getAnnoncements(provider);
	for (const announcement of announcements) {
		try {
			const stealthAddress = ethers.utils.getAddress(
				ethers.utils.hexStripZeros(announcement.args.stealthRecipientAndViewTag), // TODO - Make this more efficient with view keys and viewTag
			);
			const sharedSecret = getSharedSecret(spendPrivateKey.slice(2), `04${announcement.args.ephemeralPubKey.slice(2)}`);
			const stealthPrivateKey = getRecoveryPrivateKey(spendPrivateKey, sharedSecret);
			const stealthWallet = new ethers.Wallet(stealthPrivateKey);
			log("stealthAddress check", stealthWallet.address === stealthAddress, stealthWallet.address, stealthAddress);
			if (stealthWallet.address === stealthAddress) {
				// const tokenAsStealthWallet = token.connect(stealthWallet);
				stealthAddresses.push(ethers.utils.getAddress(stealthAddress));
			}
		} catch (error) {
			throw new ApiError(500, `Something went terribly wrong when checking for stealth addresses: ${error}`);
		}
	}
	return stealthAddresses;
}

function parseBody(body: any) {
	if (!("signature" in body)) {
		throw new ApiError(400, "No signature provided in body");
	}
	if (!("spendPublicKey" in body)) {
		throw new ApiError(400, "No spendPublicKey provided in body");
	}

	const signature: string = body.signature.toString();
	const spendPublicKey: string = body.spendPublicKey.toString();
	const isValidSignature = (sig: string) => ethers.utils.isHexString(sig) && sig.length === 132;
	if (!isValidSignature(signature)) {
		throw new ApiError(400, `Invalid signature: ${signature}`);
	}

	return { signature, spendPublicKey };
}

function parseQuery(
	query: Partial<{
		[key: string]: string | string[];
	}>,
) {
	if (!query.signature) {
		throw new ApiError(400, "missing signature");
	}
	if (typeof query.signature !== "string") {
		throw new ApiError(400, "signature must be a string");
	}
	const signature = query.signature;
	return { signature };
}
