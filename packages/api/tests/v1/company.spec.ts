import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { ethers } from 'ethers';
import { CapTable, CapTableRegistry__factory, CapTable__factory, ERC5564Messenger__factory, ERC5564Registry__factory } from '@brok/captable';
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from '../../src/contants';
import { getSharedSecret, getStealthAddress, signatureToStealthKeys } from '../../src/utils/stealth';

// Address 0xAbba3265E2dcdb5004CB87ca0F1280F5c6C9E33C
// const walletToGiveShares= new ethers.Wallet("0xa1828a210aae8fbd1f31b928d84d875bd583ef921773114944fc26f5ce113219")

const userWallet = ethers.Wallet.createRandom();
export const MESSAGE_FOR_SIGNATURE = 'ONLY FOR DEMO PURPOSES ==== BROK ====  ONLY FOR DEMO PURPOSES'; // sentence to recover stealth keys, known to everyone

// // Annotate entire file as serial.
// test.describe.configure({ mode: 'serial' });

test('should find all captables registered', async ({ request, baseURL }) => {

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

});