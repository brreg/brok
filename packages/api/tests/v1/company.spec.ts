import { test, expect } from '@playwright/test';
import { ethers } from 'ethers';
import { CreateNewCapTable, FindCapTableWithAddress, GenerateRandomCompanyName, GenerateRandomOrgnr } from '../utils';

// Address 0xAbba3265E2dcdb5004CB87ca0F1280F5c6C9E33C
// const walletToGiveShares= new ethers.Wallet("0xa1828a210aae8fbd1f31b928d84d875bd583ef921773114944fc26f5ce113219")

const userWallet = ethers.Wallet.createRandom();
export const MESSAGE_FOR_SIGNATURE = 'ONLY FOR DEMO PURPOSES ==== BROK ====  ONLY FOR DEMO PURPOSES'; // sentence to recover stealth keys, known to everyone

// // Annotate entire file as serial.
// test.describe.configure({ mode: 'serial' });

test('should find all captables registered', async ({ request, baseURL }) => {
  await CreateNewCapTable()

  const res = await request.get(`${baseURL}/api/v1/company/`, {
    headers: {
      'Content-Type': 'application/json',
    },
  });

  expect(res).toBeOK;
  const json = await res.json();
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('allCapTables' in json, 'json object should have property stealthAddress').toBe(true);
  expect(json.allCapTables.length, 'json property success should be true').toBeGreaterThan(0);

  console.log(json.allCapTables)
});

test('should create a new captable and find it', async ({ request, baseURL }) => {

  const res = await request.post(`${baseURL}/api/v1/company/`, {
    headers: {
      'Content-Type': 'application/json',
    },
    data: JSON.stringify({
      name: GenerateRandomCompanyName(),
      orgnr: GenerateRandomOrgnr().toString(),
    }),
  });

  expect(res).toBeOK;
  const json = await res.json();
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('capTableAddress' in json, 'json object should have property capTableAddress').toBe(true);
  expect(json.capTableAddress.length, 'json capTableAddress should be longer than zero').toBeGreaterThan(0);
  
  const capTable = await FindCapTableWithAddress(json.capTableAddress)

  console.log(json.capTable)
});