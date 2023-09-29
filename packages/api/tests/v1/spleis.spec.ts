import { expect, test } from "@playwright/test";
import axios from "axios";
import { ConnectToCapTable_R } from "../../src/utils/blockchain";
import { DEFAULT_PARTITION, JONNY, NINA, commonTestSetup, jsonHeader } from "../test-setup";

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

test("should execute a share spleis", async ({ request, baseURL }) => {
	const spleisForhold = 1 / 2;
	const mottakere = [NINA.IDENTIFIER, JONNY.IDENTIFIER];
	const navaerendeBalanse = [30000, 3333];
	const captable = await ConnectToCapTable_R(captableAddress);
	const ninaBalance = await captable.balanceOfByPartition(DEFAULT_PARTITION, ninaWalletAddress);

	const antall = [
		calculateSharesToBeDeleted(navaerendeBalanse[0], spleisForhold),
		calculateSharesToBeDeleted(navaerendeBalanse[1], spleisForhold)
	];

	// Utføre spørringen for aksjesplitt
	const data = JSON.stringify({ mottakere, antall });
	const res = await axios.post(`${baseURL}/api/v1/company/${orgnr}/spleis`, data, jsonHeader);

	expect(res.status).toBe(200);

	// Valider aksjeantallet og forholdet etter splitten. 
	const balance = await captable.balanceOfByPartition(DEFAULT_PARTITION, ninaWalletAddress);
	expect(balance.toString()).toBe((ninaBalance * spleisForhold).toString());
});

const calculateSharesToBeDeleted = (oldShares: number, splitRatio: number) => {
	const newShares = Math.round(oldShares * splitRatio);
	return oldShares - newShares;
}
