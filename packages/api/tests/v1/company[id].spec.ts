import { test, expect } from '@playwright/test';
import { faker } from '@faker-js/faker';
import { ethers } from 'ethers';
import { CapTable, CapTableRegistry__factory, CapTable__factory, ERC5564Messenger__factory, ERC5564Registry__factory } from '@brok/captable';
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from '../../src/contants';
import { getSharedSecret, getStealthAddress, signatureToStealthKeys } from '../../src/utils/stealth';
import { CreateNewCapTable, FindCapTableWithAddress, GenerateRandomCompanyName, GenerateRandomOrgnr } from '../utils';

test('should find captable by orgnr', async ({ request, baseURL }) => {
  const { orgnr } = await CreateNewCapTable()

  const res = await request.get(`${baseURL}/api/v1/company/${orgnr}`, {
    headers: {
      'Content-Type': 'application/json',
    },
  });

  expect(res).toBeOK();
  const json = await res.json();
  expect(json, 'json object should be defined').toBeDefined();
  expect(typeof json).toBe('object');
  expect(Object.keys(json).length, `json should have properties ${JSON.stringify(json)}`).toBeGreaterThan(0);
  expect('name' in json, 'json object should have property name').toBe(true);
  expect('orgnr' in json, 'json object should have property orgnr').toBe(true);
  expect('totalSupply' in json, 'json object should have property totalSupply').toBe(true);
  expect(json.orgnr, 'json property success should be true').toBe(orgnr);
});