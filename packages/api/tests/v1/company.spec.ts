/**
 * Testing the captable API endpoints for creating and fetching cap tables.
 *
 * Helper functions:
 * @function CreateNewCapTable
 * @function FindCapTableWithAddress
 * @function GenerateRandomCompanyName
 * @function GenerateRandomOrgnr
 */

import { expect, test } from "@playwright/test";
import { ethers } from "ethers";
import { WALLET } from "../../src/contants";
import { ConnectToCapTable_R } from "../../src/utils/blockchain";
import { CreateNewCapTable, GenerateRandomCompanyName, GenerateRandomOrgnr, sjekkMottakere } from "../utils";
import { BulkLookupResponse, WalletInfo, WalletRecordInNavnetjener, createWalletRecord } from "../../src/utils/navnetjener";
import { DEFAULT_PARTITION, JONNY, NINA, jsonHeader } from "../test-setup";
import axios from "axios";

// Annotate entire file as serial.
test.describe.configure({ mode: "serial" });

const org1 = GenerateRandomOrgnr().toString();
let captableAddress: string;

test("should find all captables registered", async ({ request, baseURL }) => {
	await CreateNewCapTable();

	const res = await request.get(`${baseURL}/api/v1/company/`, {
		// TODO Test som oppretter captable
		headers: {
			"Content-Type": "application/json",
		},
	});

	expect(res).toBeOK();
	const json = await res.json();

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
	expect("foretakList" in json, "json object should have property foretakList").toBe(true);
	expect(json.foretakList.length, "json property success should be true").toBeGreaterThan(0);
});

// TODO Before creating a captable, a "director of the board" have to be sent as input or be created so that the wallet vault/API has it
// The "director of the board" is the one who signs the create captable transaction, or is added as controller and/or owner afterwords
test("should create a new captable and find it", async ({ request, baseURL }) => {
	const res = await request.post(`${baseURL}/api/v1/company/`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({
			name: GenerateRandomCompanyName(),
			orgnr: org1,
		}),
	});

	expect(res).toBeOK();
	const json = await res.json();

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
	expect("capTableAddress" in json, "json object should have property capTableAddress").toBe(true);
	expect(json.capTableAddress.length, "json capTableAddress should be longer than zero").toBeGreaterThan(0);

	captableAddress = json.capTableAddress;
});

// TODO Move request to utils and only use the function here
// TODO Only use axios reuest.post. Now we're mixing both
test('should return filtered list with walletAddress as null', async ({ request, baseURL }) => {
	const identifiers = [NINA.IDENTIFIER, JONNY.IDENTIFIER];

	const res = await sjekkMottakere(request, baseURL, org1, identifiers);

	const missingWallets = await res.json();
	expect(missingWallets.length).toBe(identifiers.length);

	// All returned wallets should have walletAddress as null
	for (const wallet of missingWallets) {
		expect(wallet.walletAddress).toBeNull();
	}
});

let ninaWalletAddress: string;
// TODO Har jeg tatt høyde for scenarioet hvor brukeren finnes i navnetjeneren, men ikke i selskapet? Nei, det navn og sånt hentes på nytt her, men dataen fra navnetjener kan gjennbrukes og kun ny wallet opprettes
test('should create new wallet records in navnetjener', async ({ request, baseURL }) => {
	const ninaWalletRecord: WalletRecordInNavnetjener =
	{
		OwnerPersonFirstName: NINA.FIRSTNAME,
		OwnerPersonLastName: NINA.LASTNAME,
		OwnerPersonFnr: NINA.IDENTIFIER,
		CapTableOrgnr: org1
	}

	const jonnyWalletRecord: WalletRecordInNavnetjener =
	{
		OwnerPersonFirstName: JONNY.FIRSTNAME,
		OwnerPersonLastName: JONNY.LASTNAME,
		OwnerPersonFnr: JONNY.IDENTIFIER,
		CapTableOrgnr: org1
	}

	const res = await axios.post<BulkLookupResponse>(`${baseURL}/api/v1/company/${org1}/opprett-lommeboker`, JSON.stringify([ninaWalletRecord, jonnyWalletRecord]), jsonHeader);

	console.dir(res.data, 5);

	expect(res).toBeTruthy();

	expect(res.data.wallets[0].identifier).toBe(NINA.IDENTIFIER);
	expect(res.data.wallets[0].walletAddress).toBeTruthy();

	ninaWalletAddress = res.data.wallets[0]?.walletAddress ?? "ERROR";
});

test('tmp test; no wallets should be missing', async ({ request, baseURL }) => {
	const identifiers = [NINA.IDENTIFIER, JONNY.IDENTIFIER];
	const res = await sjekkMottakere(request, baseURL, org1, identifiers);
	const missingWallets = await res.json();
	expect(missingWallets.length).toBe(0);
});

test("should populate captable with shareholders", async ({ request, baseURL }) => {
	const captable = await ConnectToCapTable_R(captableAddress);

	const mottakere = [NINA.IDENTIFIER, JONNY.IDENTIFIER];
	const antall = [30000, 3333];

	expect(await captable.isMinter(WALLET.address)).toBe(true);

	const bigNumberAntall = ethers.BigNumber.from(antall[0].toString());
	const granularity = await captable.granularity();
	expect(bigNumberAntall.mod(granularity).toString()).toBe("0");

	// Issue
	// TODO Gjør til util func
	const res = await request.post(`${baseURL}/api/v1/company/${org1}/kapitalforhoyelse`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({
			mottakere,
			antall,
		}),
	});

	// Verify

	const balance = await captable.balanceOfByPartition(DEFAULT_PARTITION, ninaWalletAddress);
	expect(balance.toString()).toBe(antall[0].toString());

	// Verify med navnetjener-API
	// const req: BulkLookupRequest = {
	// 	identifiers: [NINA.IDENTIFIER],
	// 	parentOrgnr: org1,
	// }
	// const balances = await balanceOfIdentifiers(req);
	// expect(balances.wallets[0].balance).toBe(antall[0].toString());
});

// TODO Tester for at mottaker er et selskap
test("should successfully transfer shares", async ({ request, baseURL }) => {
	const antall = 333;
	const captable = await ConnectToCapTable_R(captableAddress);
	const senderBalance = await captable.balanceOfByPartition(DEFAULT_PARTITION, ninaWalletAddress);

	// 1. Verify that the receiver has a wallet
	const resSjekkMottakere = await sjekkMottakere(request, baseURL, org1, [JONNY.IDENTIFIER]);
	// TODO Consider if the response should be verified

	// (2. Create wallet for receiver using createWalletRecord() if it doesn't exist)

	// 3. Perform the transfer
	const res = await request.post(`${baseURL}/api/v1/company/${org1}/overdragelse/${NINA.IDENTIFIER}/${JONNY.IDENTIFIER}/${antall}`);

	// Verify the response
	expect(res.status()).toBe(200);
	const responseBody = await res.json();
	expect(responseBody.message).toBe(
		`Successfully transferred ${antall} shares`,
	);

	const senderNewBalance = await captable.balanceOfByPartition(DEFAULT_PARTITION, ninaWalletAddress);
	expect(senderNewBalance.toString()).toBe((senderBalance - antall).toString());
});
