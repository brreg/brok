import { expect, test } from "@playwright/test";
import { ethers } from "ethers";
import { WALLET } from "../../src/contants";
import { ConnectToCapTable_R } from "../../src/utils/blockchain";
import { CreateNewCapTable, GenerateRandomCompanyName, GenerateRandomOrgnr } from "../utils";
import { WalletRecordInNavnetjener, createWalletRecord } from "../../src/utils/navnetjener";
import axios from "axios";
import { ForetakResponse } from "../../src/pages/api/v1/company";

// Annotate entire file as serial.
test.describe.configure({ mode: "serial" });

// Global variables
const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");
const org1 = GenerateRandomOrgnr().toString();
let captableAddress: string;

// TODO Dette er copy/paste fra navnetjener.spec.ts; bør heller gjenbrukes
const JONNY = {
	IDENTIFIER: "18998612345",
	FIRSTNAME: "Jonny",
	LASTNAME: "Bravo",
};
const NINA = {
	IDENTIFIER: "15097600002",
	FIRSTNAME: "Nina",
	LASTNAME: "Pedersen",
};

// TODO Testdataen burde kanskje bare være klargjort allerede istedenfor å måtte kjøre alle disse første testene. Dette gjelder fra
// START
test("should create a new captable", async ({ baseURL }) => {
	const data = {
		name: GenerateRandomCompanyName(),
		orgnr: org1,
	};

	const res = await axios.post<ForetakResponse>(`${baseURL}/api/v1/company/`, data);
	expect(res.status).toBe(200);

	captableAddress = res.data.capTableAddress;
});

let ninaWalletAddress: string;
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

	const res = await createWalletRecord([ninaWalletRecord, jonnyWalletRecord]);
	expect(res).toBeTruthy();

	expect(res.wallets[0].identifier).toBe(NINA.IDENTIFIER);
	expect(res.wallets[0].walletAddress).toBeTruthy();

	ninaWalletAddress = res.wallets[0]?.walletAddress ?? "ERROR";
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
	const res = await request.post(`${baseURL}/api/v1/company/${org1}/kapitalforhoyelse`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({
			mottakere,
			antall,
		}),
	});
});
// END


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
	const res = await request.post(`${baseURL}/api/v1/company/${org1}/splitt`, {
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
	return oldShares * (splitRatio - 1);
}