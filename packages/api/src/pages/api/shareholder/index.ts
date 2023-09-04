// import type { NextApiRequest, NextApiResponse } from "next";
// import { CONTRACT_ADDRESSES, GET_PROVIDER, WALLET } from '../../../contants';
// import { ApiRequestLogger } from "../../../utils/api";
// import debug from 'debug';
// import { ethers } from 'ethers';

// import { ApiError } from 'next/dist/server/api-utils';

// type Data = {};
// const log = debug('brok:api:shareholder:index');

// // create stealth
// export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
//   try {
//     ApiRequestLogger(req, log);
//     switch (req.method) {
//       case 'GET':
//         // show wallet for user
//         res.status(200).json({ message: 'ok' });
//         res.end();
//         break;
//       case 'POST': {
//         const { shareholderSosialSecurityNumber } = parseBody(req);
//         const randomWallet = ethers.Wallet.createRandom();
//         // const spendKey = await SPEND_KEY();

//         // // const sharedSecret = getSharedSecret(randomWallet.privateKey, BigInt(111));
//         // const stealthAddress = getStealthAddress(spendKey.publicKey, BigInt(111));

// 				// const stealthAddressRegistry = new ERC5564Registry__factory(WALLET.connect(GET_PROVIDER())).attach(CONTRACT_ADDRESSES.ERC5564_REGISTRY);

//         // create wallet

//         // input:
//         //    - private key?

//         //

//         // output:
//         //    - wallet
//         res.status(200).json({
//           stealthAddress,
//         });
//         res.end();
//         break;
//       }
//       default:
//         res.setHeader('Allow', ['GET', 'POST']);
//         res.status(405).end(`Method ${req.method} Not Allowed`);
//     }
//   } catch (error) {
// 		if (error instanceof ApiError) {
// 			log(`HTTP Response ${error.statusCode}, ${error.message} ${error}`)
// 			return res.status(error.statusCode).json({
// 				error: error,
// 				message: error.message
// 			})
// 		}
// 		log(`HTTP Response 500, could not do stealth address stuff, se error message: ${error}`)
// 		return res.status(500).json({
// 			error: error,
// 			message: "could not do stealth address stuff, se error message"
// 		})
// 	}
// }

// function parseBody(body: any): RequestBody {
//   if (!('shareholderSosialSecurityNumber' in body)) {
//     throw new ApiError(400, 'missing sharesReceiverPublicKey in body');
//   }

// 	const shareholderSosialSecurityNumber: number = parseInt(body.shareholderSosialSecurityNumber)
// 	if (typeof shareholderSosialSecurityNumber !== "number") {
// 		throw new ApiError(400, `shareholderSosialSecurityNumber must be a number, received ${typeof shareholderSosialSecurityNumber}`)
// 	}

// 	if (body.sharesReceiverPublicKey) {
// 		const sharesReceiverPublicKey: string = body.sharesReceiverPublicKey.toString();
// 		if (!ethers.utils.isAddress(sharesReceiverPublicKey)) {
// 			throw new ApiError(400, `${sharesReceiverPublicKey} is not a valid wallet`);
// 		}

// 		return { sharesReceiverPublicKey, shareholderSosialSecurityNumber };
// 	}

// 	return { shareholderSosialSecurityNumber }
// }

// type RequestBody = {
//   sharesReceiverPublicKey?: string;
// 	shareholderSosialSecurityNumber: number
// };
