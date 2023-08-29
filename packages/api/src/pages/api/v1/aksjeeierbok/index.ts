import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
// import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from '../../../../../contants';
import debug from "debug";
import { ApiError } from "next/dist/server/api-utils";
// import { ConnectToCapTableRegistry_R, ConnectToCapTable_R, ConnectToStealthAddressFactory_RW } from '../../../../../utils/blockchain';
import { ErrorResponse, ApiRequestLogger } from "../../../../utils/api";
// import { getCapTableWithOrgnr } from '../../../../../utils/the-graph';
import { CapTable, ERC5564Registry__factory, CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
// import { CapTable, CapTableRegistry } from "../../../../../../captable/typechain-types/index";
import {
	ConnectToCapTableRegistry_R,
	ConnectToCapTableRegistry_RW,
	ConnectToCapTable_R,
	handleRPCError,
} from "../../../../utils/blockchain";

const log = debug("brok:api:v1:company:[id]");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);
		switch (req.method) {
			// case "GET": {
			// 	// Find info about company
			// 	const { orgnr } = parseQuery(req.query);
			// 	const captable = await getCapTableWithOrgnr(orgnr);
			// 	log("captable:", captable);
			// 	if (!captable) {
			// 		throw new ApiError(404, `Could not find any company with orgnr ${orgnr} in BRØK`);
			// 	}

			// 	// const name = await captable.name()
			// 	// const totalSupply = ethers.utils.formatEther(await captable.totalSupply())

			// 	log("HTTP Response 200, return captable");
			// 	return res.status(200).json({
			// 		captable,
			// 	});
			// }
			case "PUT": {
				// Update captable info for company
			}
			case "DELETE": {
				// Delete company
			}
			case "POST": {
				// Create new cap table

				// Input er
				// - foretak
				// - orgnr
				// - list of strings (operators)
				// - [list of name]
				// - [list of pnr]
				// - [list of Antall aksjer]
				// ***
				// - [aksjeklasser (antall? navn? antall aksjer?) utsetter til v2?]
				// - [list of partition til hver av adressene over, men hvis partisjoner ikke er i v1…]
				// - [list of receving addresses (men de skal vel genereres her uansett)]

				// const { foretaksNavn, orgnr, operators, aksjeeiere } = req.body;

				// 1. Utilize the provided code
				const { foretaksNavn, orgnr, operators, aksjeeiere } = {
					foretaksNavn: "Eksempel AS",
					orgnr: "123456789",
					operators: ["0x388C818CA8B9251b393131C08a736A67ccB19297", "0x4675C7e5BaAFBFFbca748158bEcBA61ef3b0a263"],
					aksjeeiere: [
						{
							pnr: "01010112345",
							name: "Ola Nordmann",
							antallAksjer: 1000,
						},
						{
							pnr: "02020212345",
							name: "Kari Nordmann",
							antallAksjer: 500,
						},
					],
				};

				const wallet = ethers.Wallet.createRandom(); // TODO Må hente walleten til styreleder
				const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");
				const CONTROLLERS = operators.map((operator) => ethers.utils.getAddress(operator));

				// 2. Deploy a new CapTable contract with the data
				const capTableFactory = new CapTable__factory(wallet);
				const capTable = await capTableFactory.deploy(foretaksNavn, orgnr, 1, CONTROLLERS, [DEFAULT_PARTITION]); // Assuming these are the correct parameters
				await capTable.deployed();

				// 3. Add the captable to the registry
				const capTableRegistry = await ConnectToCapTableRegistry_RW(); // Connect to the registry with write access
				await capTableRegistry.addCapTable(capTable.address); // Assuming this is the correct function to add a CapTable

				// 4. Issue shares to the shareholders
				for (const shareholder of aksjeeiere) {
					const amount = ethers.utils.parseEther(shareholder.antallAksjer.toString());
					await capTable.issueShares(shareholder.pnr, amount, DEFAULT_PARTITION); // Assuming this is the correct function to issue shares
				}

				console.log("CapTable deployment and share issuance successful!");
			}
			default:
				res.setHeader("Allow", ["GET", "PUT", "DELETE", "POST"]);
				res.status(405).end(`Method ${req.method} Not Allowed`);
		}
	} catch (error) {
		ErrorResponse(error, log, res);
	}
}

// async function findCapTableWithOrgnr(orgnr: string): Promise<CapTable | undefined> {
//   const captable = await getCapTableWithOrgnr(orgnr)

//   const capTableRegistry = await ConnectToCapTableRegistry_R();
//   const allCapTablesAddresses = await capTableRegistry.getCapTableList();
//   for (const address of allCapTablesAddresses) {
//     const captable = await ConnectToCapTable_R(address);
//     const nr = await captable.getOrgnr()
//     if (orgnr === nr) {
//       return captable
//     }
//   }
// }

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
	if (!query.orgnr) {
		throw new ApiError(400, "missing orgnr");
	}

	// if (typeof query.orgnr !== 'string') {
	//   throw new ApiError(400, 'orgnr must be provided as a string. e.g "112233445"');
	// }

	// if (query.orgnr.length !== 9) {
	//   throw new ApiError(400, 'orgnr must be nine digits');
	// }

	try {
		parseInt(query.orgnr.toString());
	} catch (error) {
		throw new ApiError(400, "orgnr must be a valid number");
	}

	const orgnr: string = query.orgnr.toString();
	return { orgnr };
}
