import { ERC5564Registry__factory } from "@brok/captable";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from "../../../contants";
import { formatPublicKeyForSolidityBytes, getStealthAddress } from "../../../utils/stealth";
import debug from "debug";
import ApiRequestLogger from "../../../utils/apiRequestLogger";
import { ApiError } from "next/dist/server/api-utils";
import { connectToStealthAddressFactory_RW } from "../../../utils/blockchain";

const log = debug("brok:shareholder:register");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	ApiRequestLogger(req, log)
	switch (req.method) {
		case "GET":
			// TODO

		case "POST":
      // Register new user's wallet, so stealth addresses can be created from it

      const { signature, spendPublicKey } = parseBody(req.body)
    
			const digest = ethers.utils.arrayify(ethers.utils.hashMessage(spendPublicKey));
      // a digested spendPublicKey + signature should return user wallet address
			const recoveredAddress = ethers.utils.recoverAddress(digest, signature);

			const registry = connectToStealthAddressFactory_RW()

			// get address from ethereum signature
			log(`Checking address: ${recoveredAddress} for stealth keys`);
			const currentKeys = await registry.stealthKeys(recoveredAddress, CONTRACT_ADDRESSES.SECP256K1_GENERATOR);
			log("current keys", currentKeys)
			if ("spendingPubKey" in currentKeys && currentKeys.spendingPubKey !== "0x") {
				log("Keys already registered, returning address");
				return res.status(200).json({
					success: true,
					message: "Keys already registered",
				});
			} else {
				log("Keys not registered, start registering");
				const spendPublicKeyParsed = formatPublicKeyForSolidityBytes(spendPublicKey);
				const viewPublicKeyParsed = "0x11"; // TODO - Start using view keys

				const tx = await registry.registerKeysOnBehalf(
					recoveredAddress,
					CONTRACT_ADDRESSES.SECP256K1_GENERATOR,
					"0x11",
					spendPublicKeyParsed,
					viewPublicKeyParsed,
				);
				const receipt = await tx.wait();
				log(`HTTP Response 200, Registered keys for ${recoveredAddress} with receipt ${receipt.transactionHash}`)
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

function parseBody(body: any): RequestBody {
  if (!("signature" in body)) {
    throw new ApiError(400, "No signature provided in body");
  }
  if (!("spendPublicKey" in body)) {
    throw new ApiError(400, "No spendPublicKey provided in body");
  }

  const signature: string = body.signature.toString()
  const spendPublicKey: string = body.spendPublicKey.toString()
  const isValidSignature = (sig: string) => ethers.utils.isHexString(sig) && sig.length === 132;
  if (!isValidSignature(signature)) {
    throw new ApiError(400, `Invalid signature: ${signature}`);
  }
  
  return { signature, spendPublicKey  }
}

type RequestBody = {
  signature: string;
	spendPublicKey: string
};