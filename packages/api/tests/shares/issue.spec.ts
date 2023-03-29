import { test, expect } from "@playwright/test";
import { faker } from '@faker-js/faker';
import { ethers } from "ethers";
import { CapTable, CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from "../../src/contants";
import debug from "debug";

const log = debug("brok:api:test:shares:issue")

test("/api/shareholder/verify should return true", async ({ request, baseURL }) => {
	
  const { capTableAddress, addressToReceiveTokens } = await setup()
  
  const res = await request.post(`${baseURL}/api/shares/issue`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({ capTableAddress, addressToReceiveTokens, amount: "1000" }),
	});
	expect(res.ok(), await res.text()).toBe(true);
	const json = await res.json();
	expect(res.ok(), json).toBe(true);

	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);

	expect("transaction" in json, "json object should have property status").toBe(true);
	expect("created" in json.transaction, "transaction object should have property created").toBe(true);
	expect("completed" in json.transaction, "transaction object should have property completed").toBe(true);
	expect("hash" in json.transaction, "transaction object should have property hash").toBe(true);
	expect("message" in json, "json object should have property message").toBe(true);
	expect(json.transaction.created).toBe(true)
})

async function setup() {
  // setup
  const wallet = WALLET.connect(GET_PROVIDER());
  const orgnr = randomOrgnr().toString()
  const orgName = faker.company.name()
  const addressToReceiveTokens = ethers.Wallet.createRandom().address
  const capTable_Registry = new CapTableRegistry__factory(wallet).attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);
  
  // add address to whitelist, so address can receive tokens from CapTable
  const addAddress_result = await capTable_Registry.setAuthenticatedPerson(addressToReceiveTokens)
  log("add address to whitelist",addAddress_result)

  // create CapTable
  const createCapTable_transaction = await new CapTable__factory().getDeployTransaction(
    orgName,
    orgnr,
    ethers.utils.parseEther("1"),
    CONTROLLERS,
    [DEFAULT_PARTITION],
    CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY
  );
  const createCapTable_transaction_signed = await wallet.sendTransaction(createCapTable_transaction)
  log("create CapTable:", createCapTable_transaction_signed)

  const capTableAddress = ethers.utils.getContractAddress(createCapTable_transaction_signed)
  log("new CapTable address", capTableAddress)

  // add CapTable to CapTable Registry
  const addCapTableToRegistry = await capTable_Registry.addCapTable(capTableAddress, orgnr)
  log("add new CapTable to Registry", addCapTableToRegistry)

  return {capTableAddress, addressToReceiveTokens}
}

function randomOrgnr() {
  const min = Math.ceil(11111111);
  const max = Math.floor(999999999);
  return Math.floor(Math.random() * (max - min + 1) + min);
}