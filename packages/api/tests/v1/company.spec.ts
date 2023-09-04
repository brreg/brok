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
import { CreateNewCapTable, GenerateRandomCompanyName, GenerateRandomOrgnr } from "../utils";

// Annotate entire file as serial.
test.describe.configure({ mode: "serial" });

const org1 = GenerateRandomOrgnr().toString();
const user1 = ethers.Wallet.createRandom();
const user2 = ethers.Wallet.createRandom();
let captableAddress: any;

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
	expect("allCapTables" in json, "json object should have property allCapTables").toBe(true);
	expect(json.allCapTables.length, "json property success should be true").toBeGreaterThan(0);
});

test("should create a new captable and find it", async ({ request, baseURL }) => {
	// TODO Before creating a captable, a "director of the board" have to be sent as input or be created so that the wallet vault/API has it
	// The "director of the board" is the one who signs the create captable transaction, or is added as controller and/or owner afterwords

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

test("should populate captable with shareholders", async ({ request, baseURL }) => {
	// TODO Before issuing, shareholders have to be sent as input or be created so that the wallet vault/API has them

	const captable = await ConnectToCapTable_R(captableAddress);

	const aksjeklasser = ["ordinære"];
	const mottakerAdresser = [user1.address];
	const antall = [1000];

	expect(await captable.isMinter(WALLET.address)).toBe(true);
	expect((antall[0] % (await captable.granularity())).toString()).toBe("0");

	// Issue
	const res = await request.post(`${baseURL}/api/v1/company/${org1}/kapitalforhoyelse`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({
			orgnr: org1,
			aksjeklasser,
			mottakerAdresser,
			antall,
		}),
	});

	// Verify
	const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");
	const partitions = [DEFAULT_PARTITION];

	const balance = await captable.balanceOfByPartition(partitions[0], user1.address);
	expect(balance.toString()).toBe(antall[0].toString());
});

test("should successfully transfer shares", async ({ request, baseURL }) => {
	const orgnr = org1;
	const aksjeklasse = "ordinære";
	const sender = user1.address;
	const mottaker = user2.address;
	const antall = 333;

	const captable = await ConnectToCapTable_R(captableAddress);

	// Precondition: Verify that sender has enough shares to transfer
	const DEFAULT_PARTITION = ethers.utils.formatBytes32String(aksjeklasse);

	const senderBalance = await captable.balanceOfByPartition(DEFAULT_PARTITION, sender);
	// expect(senderBalance.toString()).toBeGreaterThanOrEqual(antall.toString());

	// Perform the transfer
	const res = await request.post(`${baseURL}/api/v1/company/${orgnr}/overdragelse`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({
			sender,
			mottaker,
			aksjeklasse,
			antall,
		}),
	});

	// Verify the response
	expect(res.status()).toBe(200);
	const responseBody = await res.json();
	expect(responseBody.message).toBe(
		`Successfully transferred ${antall} shares of class ${aksjeklasse} from ${sender} to ${mottaker}`,
	);

	// Postcondition: Verify the new balances of sender and recipient
	const senderNewBalance = await captable.balanceOfByPartition(DEFAULT_PARTITION, sender);
	expect(senderNewBalance.toString()).toBe((senderBalance - antall).toString());

	const recipientBalance = await captable.balanceOfByPartition(DEFAULT_PARTITION, mottaker);
	expect(recipientBalance.toString()).toBe(antall.toString());
});
