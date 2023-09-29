import { expect, test } from "@playwright/test";
import { ConnectToCapTable_R } from "../../src/utils/blockchain";
import { DEFAULT_PARTITION, JONNY, NINA, commonTestSetup, jsonHeader } from "../test-setup";
import axios from "axios";
import { getAmountOfSharesForOwner } from "../../src/utils/navnetjener";

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


test("should execute a capital reduction", async ({ request, baseURL }) => {
	// Jonny innløser sine aksjer, som deretter blir slettet.
	const aksjeeiere = [JONNY.IDENTIFIER];
	const antall = [3333];
	const captable = await ConnectToCapTable_R(captableAddress);

	// Utføre spørringen for aksjesplitt
	const data = JSON.stringify({ aksjeeiere, antall });
	const res = await axios.post(`${baseURL}/api/v1/company/${orgnr}/kapitalnedsettelse`, data, jsonHeader);

	expect(res.status).toBe(200);

	// Valider aksjeantallet og forholdet etter splitten. 
	// TODO Find a way to wait for graphql to be ready
	await new Promise(resolve => setTimeout(resolve, 2000));

	const jonnyNewBalance = await getAmountOfSharesForOwner(orgnr, JONNY.IDENTIFIER);
	expect(jonnyNewBalance.toString()).toBe('0');
});

const calculateNewlyIssuedShares = (oldShares: number, splitRatio: number) => {
	const newShares = Math.round(oldShares * splitRatio);
	return newShares - oldShares;
}