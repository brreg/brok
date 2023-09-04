/**
 * Testing the captable API endpoints for creating and fetching cap tables.
 *
 * Helper functions:
 * @function CreateNewCapTable
 * @function FindCapTableWithAddress
 * @function GenerateRandomCompanyName
 * @function GenerateRandomOrgnr
 */

import { CapTable__factory } from "@brok/captable";
import { test, expect } from "@playwright/test";
import { CreateNewCapTable, FindCapTableWithAddress, GenerateRandomCompanyName, GenerateRandomOrgnr } from "../utils";
import {
	ConnectToCapTableRegistry_R,
	ConnectToCapTableRegistry_RW,
	ConnectToCapTable_R,
	handleRPCError,
} from "../../src/utils/blockchain";
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from "../../src/contants";
import { ethers } from "ethers";

// Annotate entire file as serial.
test.describe.configure({ mode: "serial" });

const org1 = GenerateRandomOrgnr().toString();
const user1 = ethers.Wallet.createRandom();
let captableAddress: any;

// test("should find all captables registered", async ({ request, baseURL }) => {
// 	await CreateNewCapTable();

// 	const res = await request.get(`${baseURL}/api/v1/company/`, {
// 		// TODO Test som oppretter captable
// 		headers: {
// 			"Content-Type": "application/json",
// 		},
// 	});

// 	expect(res).toBeOK();
// 	const json = await res.json();
// 	expect(json, "json object should be defined").toBeDefined();
// 	expect(typeof json).toBe("object");
// 	expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
// 	expect("allCapTables" in json, "json object should have property allCapTables").toBe(true);
// 	expect(json.allCapTables.length, "json property success should be true").toBeGreaterThan(0);
// });

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

test("should populate captable with shareholders", async ({ request, baseURL }) => {
	const captable = await ConnectToCapTable_R(captableAddress);

	// Ikke mulig å hente liste over aksjonærer uten navnetjeneren (graph e.l.)
	// Så jeg må sjekke på aksjonærene direkte, men jeg har dem jo her, så det burde ikke være et problem

	const wallet = WALLET.connect(GET_PROVIDER());
	const captable_RW = captable.connect(wallet);

	const aksjeklasser = ["ordinære"];
	const mottakerAdresser = [user1.address];
	const antall = [1000];

	// assert wallet has captable_rw minter role
	expect(await captable_RW.isMinter(wallet.address)).toBe(true);

	// assert if value[0] is multiple of granularity
	expect((antall[0] % (await captable_RW.granularity())).toString()).toBe("0");

	// Issue
	// TODO call endpoint
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
	// TODO Kan hende at denne feiler, for endepunktet sier kanskje OK med en gang, mens det kan ta noen sekunder før den faktisk går gjennom
	const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");
	const partitions = [DEFAULT_PARTITION];

	const balance = await captable.balanceOfByPartition(partitions[0], user1.address);
	expect(balance.toString()).toBe(antall[0].toString());
});
