import { test, expect } from "@playwright/test";
import { ethers } from "ethers";
import { CapTable, CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
import { CONTRACT_ADDRESSES, DEFAULT_NETWORK, GET_PROVIDER, WALLET } from "../src/contants";
import { loadEnvConfig } from "@next/env";
import debug from "debug";
import { handleRPCError } from "../src/utils/blockchain";
const log = debug("brok:test:checkTransaction");
// import { loadEnvConfig } from "@next/env";
// import debug from 'debug'
// const log = debug("brok:test:capTable")
test.beforeAll(() => {
	const projectDir = process.cwd();
	loadEnvConfig(projectDir);
});

test("/api/checkTransaction should return transaction status", async ({ request, baseURL }) => {
	const provider = GET_PROVIDER();
	const wallet = WALLET.connect(provider);
	const transactions = await provider.getBlockWithTransactions(1);
	const transaction = transactions.transactions[0];
	// console.log(transaction);

	// log(orgnr)
	const res = await request.get(`${baseURL}/api/checkTransaction`, {
		params: {
			transactionHash: transaction.hash,
		},
		headers: {
			"Content-Type": "application/json",
		},
	});
	const json = await res.json();
	expect(res.ok(), json).toBe(true);

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
	expect("completed" in json, "json object should have property status").toBe(true);
	expect("succeeded" in json, "json object should have property status").toBe(true);
	expect(json.completed).toBe(true);
	expect(json.succeeded).toBe(true);
});

// TODO
// Transaction fails and trows error, error is as expected so the test succeeds when its not run properly
// Try with a task manager in sted of holding on the consept of stateless api 
test("/api/checkTransaction should return transaction fail status on a failed transaction", async ({
	request,
	baseURL,
}) => {
	console.log("DEFAULT_NETWORK", DEFAULT_NETWORK);
	const provider = GET_PROVIDER();
	const wallet = WALLET.connect(provider);
	const registry = CapTableRegistry__factory.connect(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY, wallet);

	try {
		const tx = await registry.addCapTable(ethers.constants.AddressZero, "123"); // Fails here, rethink
		console.log("tx", tx);

		const transactionHash = tx.hash;
		// const transaction = await tx.wait(); // fails
		console.log("transaction", transactionHash);

		// log("transaction", transaction)
		if (!transactionHash) throw new Error("transaction should be defined");
		console.log("transactions to fail", transactionHash);

		const res = await request.get(`${baseURL}/api/checkTransaction`, {
			params: {
				transactionHash: transactionHash,
			},
			headers: {
				"Content-Type": "application/json",
			},
		});

		// log(orgnr)
		const json = await res.json();
		console.log("json", json);
		expect(res.ok(), json).toBe(true);

		expect(json, "json object should be defined").toBeDefined();
		expect(typeof json).toBe("object");
		expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
		expect("confirmations" in json, "json object should have property confirmations").toBe(true);
	} catch (error) {
		const message = handleRPCError(error);
		expect(message).toBe("address cannot be zero address");
	}
});
