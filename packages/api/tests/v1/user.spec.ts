import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker/locale/nb_NO';
import { ethers } from 'ethers';
import { CapTable, CapTableRegistry__factory, CapTable__factory, ERC5564Messenger__factory, ERC5564Registry__factory } from '@brok/captable';
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from '../../src/contants';
import { getSharedSecret, getStealthAddress, signatureToStealthKeys } from '../../src/utils/stealth';
import { CreateNewCapTable, GenerateOrgnr, IssueShares } from '../utils';

// Address 0xAbba3265E2dcdb5004CB87ca0F1280F5c6C9E33C
// const userWallet= new ethers.Wallet("0xa1828a210aae8fbd1f31b928d84d875bd583ef921773114944fc26f5ce113219")

const userWallet = ethers.Wallet.createRandom();
export const MESSAGE_FOR_SIGNATURE = 'ONLY FOR DEMO PURPOSES ==== BROK ====  ONLY FOR DEMO PURPOSES'; // sentence to recover stealth keys, known to everyone

// Annotate entire file as serial.
test.describe.configure({ mode: 'serial' });

test('should register wallet to hold stealth addresses', async ({ request, baseURL }) => {
  const stealthKeysSignature = await userWallet.signMessage(MESSAGE_FOR_SIGNATURE);
  const { spend, view } = signatureToStealthKeys(stealthKeysSignature);
  const registerKeysSignature = await userWallet.signMessage(`${spend.publicKey}`);

  const res = await request.post(`${baseURL}/api/v1/user/`, {
    headers: {
      'Content-Type': 'application/json',
    },
    data: JSON.stringify({
      signature: registerKeysSignature,
      spendPublicKey: spend.publicKey,
    }),
  });
  const json = await res.json();
  expect(res.status()).toBe(200);
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('success' in json, 'json object should have property stealthAddress').toBe(true);
  expect(json.success, 'json property success should be true').toBe(true);
});

test('should avoid duplicate registry for user wallet', async ({ request, baseURL }) => {
  const stealthKeysSignature = await userWallet.signMessage(MESSAGE_FOR_SIGNATURE);
  const { spend, view } = signatureToStealthKeys(stealthKeysSignature);
  const registerKeysSignature = await userWallet.signMessage(`${spend.publicKey}`);

  const res = await request.post(`${baseURL}/api/v1/user/`, {
    headers: {
      'Content-Type': 'application/json',
    },
    data: JSON.stringify({
      signature: registerKeysSignature,
      spendPublicKey: spend.publicKey,
    }),
  });

  const json = await res.json();
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('success' in json, 'json object should have property stealthAddress').toBe(true);
  expect(json.success, 'json property success should be true').toBe(true);
  expect(json.message, 'json property success should be true').toBe('Keys already registered');
});

test('should find no resources because nothing is registered yet ', async ({ request, baseURL }) => {
  const stealthKeysSignature = await userWallet.signMessage(MESSAGE_FOR_SIGNATURE);

  const res = await request.get(`${baseURL}/api/v1/user/`, {
    headers: {
      'Content-Type': 'application/json',
    },
    params: {
      signature: stealthKeysSignature, // TODO - After implement view keys, use viewKeys to do this operation
    },
  });

  const json = await res.json();
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  // expect('success' in json, 'json object should have property stealthAddress').toBe(true);
  expect(json.success, 'json property success should be true').toBe(true);
  expect(json.message, 'json property success should be true').toBe('Could not find any stealth addresses with this signature');
});

test('should find all resources belonging to wallet', async ({ request, baseURL }) => {
  // Setup
  const capTableAddress = await CreateNewCapTable()
  await IssueShares(capTableAddress, userWallet)

  // ----
  const stealthKeysSignature = await userWallet.signMessage(MESSAGE_FOR_SIGNATURE);

  const res = await request.get(`${baseURL}/api/v1/user/`, {
    headers: {
      'Content-Type': 'application/json',
    },
    params: {
      signature: stealthKeysSignature, // TODO - After implement view keys, use viewKeys to do this operation
    },
  });

  const json = await res.json();
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect(json.success, 'json property success should be true').toBe(true);
  expect(json.message, 'json property success should be true').toBe('Found some addresses');
  expect('stealthAddresses' in json, 'json object should have property stealthAddresses').toBe(true);

  console.log(json.stealthAddresses)
});
