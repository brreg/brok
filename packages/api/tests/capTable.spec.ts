import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { ethers } from 'ethers';
import { CapTable, CapTableRegistry__factory, CapTable__factory } from '@brok/captable';
import { GET_PROVIDER, WALLET } from '../src/contants';
import { signatureToStealthKeys } from '../src/utils/stealth';

// Annotate entire file as serial.
test.describe.configure({ mode: 'serial' });

// orgnr
const min = Math.ceil(11111111);
const max = Math.floor(999999999);
const orgnr = Math.floor(Math.random() * (max - min + 1) + min);
let capTableAddress: string;
let stealthAddress : string 

// Address 0xAbba3265E2dcdb5004CB87ca0F1280F5c6C9E33C
const walletToGiveShares= new ethers.Wallet("0xa1828a210aae8fbd1f31b928d84d875bd583ef921773114944fc26f5ce113219")
// const walletToGiveShares= ethers.Wallet.createRandom()
export const MESSAGE_FOR_SIGNATURE = "ONLY FOR DEMO PURPOSES ==== BROK ====  ONLY FOR DEMO PURPOSES"; // sentence to recover stealth keys, known to everyone

test('/api/capTable should return deploy and registry transaction hash', async ({ request, baseURL }) => {
  const res = await request.post(`${baseURL}/api/capTable`, {
    headers: {
      'Content-Type': 'application/json',
    },
    data: JSON.stringify({ name: faker.company.name(), orgnr: orgnr }),
  });
  const json = await res.json();
  expect(res.ok(), json).toBe(true);

  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('capTableAddress' in json, 'json object should have property capTableAddress').toBe(true);
  expect('capTableDeployTransactionHash' in json, 'json object should have property capTableDeployTransactionHash').toBe(true);
  expect('capTableRegistryTransactionHash' in json, 'json object should have property capTableRegistryTransactionHash').toBe(true);

  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('capTableAddress' in json, 'json object should have property capTableAddress').toBe(true);
  expect('capTableDeployTransactionHash' in json, 'json object should have property capTableDeployTransactionHash').toBe(true);
  expect('capTableRegistryTransactionHash' in json, 'json object should have property capTableRegistryTransactionHash').toBe(true);

  capTableAddress = json.capTableAddress;
});

// Jon må onboardes ved å registrere stealth keys

test('should create stealth address', async ({ request, baseURL }) => {
  const stealthKeysSignature = await walletToGiveShares.signMessage(MESSAGE_FOR_SIGNATURE);
  const { spend, view } = signatureToStealthKeys(stealthKeysSignature);
  const registerKeysSignature = await walletToGiveShares.signMessage(`${spend.publicKey}`);
	
	const res = await request.post(`${baseURL}/api/shareholder/register`, {
    headers: {
      'Content-Type': 'application/json',
    },
    data: JSON.stringify({
      signature: registerKeysSignature,
      spendPublicKey: spend.publicKey,
    }),
  });
  const json = await res.json()
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('success' in json, 'json object should have property stealthAddress').toBe(true);
  expect(json.success, 'json property success should be true').toBe(true);
});

// test('should verify stealtAddress to hold brok tokens', async ({ request, baseURL }) => {
// 	const res = await request.post(`${baseURL}/api/shareholder/verify`, {
// 		headers: {
// 			"Content-Type": "application/json",
// 		},
// 		data: JSON.stringify({ address: stealthAddress }),
// 	});
// 	expect(res.ok(), await res.text()).toBe(true);
// 	const json = await res.json();
// 	expect(res.ok(), json).toBe(true);

// 	expect(json, "json object should be defined").toBeDefined();
// 	expect(typeof json).toBe("object");
// 	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);

// 	expect("status" in json, "json object should have property status").toBe(true);
// 	expect("message" in json, "json object should have property message").toBe(true);
// 	expect("isVerfied" in json, "json object should have property message").toBe(true);
// 	expect(json.status).toBe("success")
// 	expect(json.isVerfied).toBe(true)
// })


test('should issue shares to stealth address', async ({ request, baseURL }) => {	
	const res = await request.post(`${baseURL}/api/shares/issue`, {
    headers: {
      'Content-Type': 'application/json',
    },
    data: JSON.stringify({ addressToReceiveTokens: walletToGiveShares.address, capTableAddress: capTableAddress, amount: "1000" }),
  });
  const json = await res.json()
	console.log(json)
	expect(res.ok(), json).toBe(true);

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);

	expect("transaction" in json, "json object should have property transaction").toBe(true);
	expect("created" in json.transaction, "transaction object should have property created").toBe(true);
	expect("completed" in json.transaction, "transaction object should have property completed").toBe(true);
	expect("hash" in json.transaction, "transaction object should have property hash").toBe(true);
	expect("message" in json, "json object should have property message").toBe(true);
	expect(json.transaction.created).toBe(true)
});

test('should find all tokens hidden in stealthAddresses', async ({ request, baseURL }) => {	
  const signature = await walletToGiveShares.signMessage(MESSAGE_FOR_SIGNATURE);
  const { spend, view } = signatureToStealthKeys(signature);

  const res = await request.get(`${baseURL}/api/shareholder/stealth-address`, {
    headers: {
      'Content-Type': 'application/json',
    },
    params: {
      spendPrivateKey: spend.privateKey, // TODO - After implement view keys, use viewKeys to do this operation
    }
  });
  
  const json = await res.json()
	console.log(json)
	expect(res.ok(), json).toBe(true);

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect("stealthAddresses" in json, "json object should have property stealthAddresses").toBe(true);
  expect(json.stealthAddresses.length).toBeGreaterThanOrEqual(1)
  
})