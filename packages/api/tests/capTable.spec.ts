/**
 * KEPT IN REPO FOR REFERENCE ONLY FOR NOW
 *
 * Tests API for
 * - creating and approving a captable,
 * - and issuing shares to stealth addresses.
 *
 *
 * Key components:
 * - CapTable: Smart contract representing the cap table of a company.
 * - CapTableRegistry: Registry contract for storing deployed CapTables.
 * - orgnr: Unique organization number (randomly generated).
 * - capTableAddress: Address of the deployed CapTable.
 * - walletToGiveShares: Ethereum wallet address to which shares will be issued.
 */

import { faker } from "@faker-js/faker";
import { expect, test } from "@playwright/test";
import { ethers } from "ethers";

// Annotate entire file as serial.
test.describe.configure({ mode: "serial" });

// orgnr
const min = Math.ceil(11111111);
const max = Math.floor(999999999);
const orgnr = Math.floor(Math.random() * (max - min + 1) + min);
let capTableAddress: string;

// Address 0xAbba3265E2dcdb5004CB87ca0F1280F5c6C9E33C
const walletToGiveShares = new ethers.Wallet("0xa1828a210aae8fbd1f31b928d84d875bd583ef921773114944fc26f5ce113219");

test("/api/capTable should return deploy and registry transaction hash", async ({ request, baseURL }) => {
	const res = await request.post(`${baseURL}/api/capTable`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({ name: faker.company.name(), orgnr: orgnr }),
	});

	const json = await res.json();

	expect(res.ok(), json).toBe(true);
	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);
	expect("capTableAddress" in json, "json object should have property capTableAddress").toBe(true);

	expect(
		"capTableDeployTransactionHash" in json,
		"json object should have property capTableDeployTransactionHash",
	).toBe(true);

	expect(
		"capTableRegistryTransactionHash" in json,
		"json object should have property capTableRegistryTransactionHash",
	).toBe(true);

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
	expect("capTableAddress" in json, "json object should have property capTableAddress").toBe(true);

	expect(
		"capTableDeployTransactionHash" in json,
		"json object should have property capTableDeployTransactionHash",
	).toBe(true);

	expect(
		"capTableRegistryTransactionHash" in json,
		"json object should have property capTableRegistryTransactionHash",
	).toBe(true);

	capTableAddress = json.capTableAddress;
});

test("should issue shares", async ({ request, baseURL }) => {
	const res = await request.post(`${baseURL}/api/shares/issue`, {
		headers: {
			"Content-Type": "application/json",
		},

		data: JSON.stringify({
			addressToReceiveTokens: walletToGiveShares.address,
			capTableAddress: capTableAddress,
			amount: "1000",
		}),
	});

	console.log(res);
	const json = await res.json();

	expect(res.ok(), json).toBe(true);
	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);

	expect("transaction" in json, "json object should have property transaction").toBe(true);
	expect("created" in json.transaction, "transaction object should have property created").toBe(true);
	expect("completed" in json.transaction, "transaction object should have property completed").toBe(true);
	expect("hash" in json.transaction, "transaction object should have property hash").toBe(true);
	expect("message" in json, "json object should have property message").toBe(true);
	expect(json.transaction.created).toBe(true);
});
