// import { test, expect } from "@playwright/test";
// import { ethers } from "ethers";
// import { CapTable, CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
// import { GET_PROVIDER, WALLET } from "../src/contants";
// // import { loadEnvConfig } from "@next/env";
// // import debug from 'debug'
// // const log = debug("brok:test:capTable")
// // test.beforeAll(() => {
	
// // });

// test("/api/capTable should return deploy and registry transaction hash", async ({ request, baseURL }) => {

// 	// orgnr
// 	const min = Math.ceil(11111111);
//   const max = Math.floor(999999999);
//   const orgnr = Math.floor(Math.random() * (max - min + 1) + min);

// 	// log(orgnr)
// 	const res = await request.post(`${baseURL}/api/capTable`, {
// 		headers: {
// 			"Content-Type": "application/json",
// 		},
// 		data: JSON.stringify({ name: "Playwrigth", orgnr: orgnr}),
// 	});
// 	const json = await res.json();
// 	expect(res.ok(), json).toBe(true);


// 	expect(json, "json object should be defined").toBeDefined();
// 	expect(typeof json).toBe("object");
// 	expect(Object.keys(json).length, "json should have properties" + JSON.stringify(json)).toBeGreaterThan(0)
// 	expect("capTableAddress" in json, "json object should have property capTableAddress").toBe(true);
// 	expect("capTableDeployTransactionHash" in json, "json object should have property capTableDeployTransactionHash").toBe(true);
// 	expect("capTableRegistryTransactionHash" in json, "json object should have property capTableRegistryTransactionHash").toBe(true);


// 	// const wallet = WALLET.connect(GET_PROVIDER())
// 	// console.log("wallet:", wallet)
// 	// const createdCaptable = await new CapTable__factory(wallet).attach(json.capTableAddress)
// 	// console.log("createdCaptable:", createdCaptable)
// 	// const orgnrFromBlockchain = await createdCaptable.getOrgnr()
// 	// console.log("orgnrFromBlockchain:", orgnrFromBlockchain)
// 	// expect(orgnrFromBlockchain).toBe(orgnr)
// });