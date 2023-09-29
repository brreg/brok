import { expect, test } from "@playwright/test";
import { ConnectToCapTable_R } from "../../src/utils/blockchain";
import { DEFAULT_PARTITION, JONNY, NINA, commonTestSetup } from "../test-setup";

let captableAddress: string;
let orgnr: string;
let ninaWalletAddress: string;

test.beforeAll(async ({ request, baseURL }) => {
	if (!baseURL)
		throw new Error("No baseURL");

	const result = await commonTestSetup(2, baseURL);
	captableAddress = result.captableAddress;
	orgnr = result.orgnr;
	ninaWalletAddress = result.ninaWalletAddress;
});


test("should execute a share split", async ({ request, baseURL }) => {
	const splittForhold = 3;
	const mottakere = [NINA.IDENTIFIER, JONNY.IDENTIFIER];
	const navaerendeBalanse = [30000, 3333];
	const captable = await ConnectToCapTable_R(captableAddress);
	const ninaBalance = await captable.balanceOfByPartition(DEFAULT_PARTITION, ninaWalletAddress);

	const antall = [
		calculateNewlyIssuedShares(navaerendeBalanse[0], splittForhold),
		calculateNewlyIssuedShares(navaerendeBalanse[1], splittForhold)
	];

	// Utføre spørringen for aksjesplitt
	const res = await request.post(`${baseURL}/api/v1/company/${orgnr}/splitt`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({
			mottakere,
			antall,
		}),
	});

	expect(res.status()).toBe(200);

	// Valider aksjeantallet og forholdet etter splitten. 
	const balance = await captable.balanceOfByPartition(DEFAULT_PARTITION, ninaWalletAddress);
	expect(balance.toString()).toBe((ninaBalance * splittForhold).toString());
});

const calculateNewlyIssuedShares = (oldShares: number, splitRatio: number) => {
	const newShares = Math.round(oldShares * splitRatio);
	return newShares - oldShares;
}