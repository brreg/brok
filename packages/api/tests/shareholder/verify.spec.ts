/**
 * @file Test suite for `/api/shareholder/verify` endpoint.
 *
 * This test suite aims to validate the behavior of the endpoint that handles
 * Ethereum address verification for a cap table's whitelist.
 *
 * @summary
 * The test suite includes two main scenarios:
 *
 * 1. `/api/shareholder/verify` should return status success
 *    - Generates a random Ethereum address using the ethers library.
 *    - Makes a POST request to add the address to the whitelist.
 *    - Expects the operation to be successful and the address to be marked as verified.
 *    - Makes a GET request to verify if the address is indeed in the whitelist.
 *
 * 2. `/api/shareholder/verify` should return false when checking an address not added to the whitelist
 *    - Generates a random Ethereum address that is not in the whitelist.
 *    - Makes a GET request to check the address.
 *    - Expects the API to return that the address is not verified.
 */

import { expect, test } from "@playwright/test";
import { ethers } from "ethers";
// import { loadEnvConfig } from "@next/env";
// import debug from 'debug'
// const log = debug("brok:test:capTable")
// test.beforeAll(() => {

// });

test("/api/shareholder/verify should return status success", async ({ request, baseURL }) => {
	const { address } = ethers.Wallet.createRandom();

	// try to add a new address in whitelist for approved addresses to hold captable tokens
	const res = await request.post(`${baseURL}/api/shareholder/verify`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({ address }),
	});

	expect(res.ok(), await res.text()).toBe(true);
	const json = await res.json();
	expect(res.ok(), json).toBe(true);

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);

	expect("status" in json, "json object should have property status").toBe(true);
	expect("message" in json, "json object should have property message").toBe(true);
	expect("isVerfied" in json, "json object should have property message").toBe(true);
	expect(json.status).toBe("success");
	expect(json.isVerfied).toBe(true);

	// check if wallets exsist in whitelist
	const res2 = await request.get(`${baseURL}/api/shareholder/verify`, {
		params: {
			address,
		},
		headers: {
			"Content-Type": "application/json",
		},
	});

	expect(res2.ok(), await res2.text()).toBe(true);
	const json2 = await res2.json();
	expect(res2.ok(), json2).toBe(true);

	expect(json2, "json object should be defined").toBeDefined();
	expect(typeof json2).toBe("object");
	expect(Object.keys(json2).length, `json should have properties${JSON.stringify(json2)}`).toBeGreaterThan(0);
	console.log(json2);

	expect("isVerfied" in json, "json object should have property message").toBe(true);
	expect(json2.isVerfied).toBe(true);
});

test("/api/shareholder/verify should return false when checking a address not added to the whitelist", async ({
	request,
	baseURL,
}) => {
	const { address } = ethers.Wallet.createRandom();

	const res = await request.get(`${baseURL}/api/shareholder/verify`, {
		params: {
			address,
		},
		headers: {
			"Content-Type": "application/json",
		},
	});

	expect(res.ok(), await res.text()).toBe(true);
	const json = await res.json();
	expect(res.ok(), json).toBe(true);

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);
	console.log(json);

	expect("isVerfied" in json, "json object should have property message").toBe(true);
	expect(json.isVerfied).toBe(false);
});
