import { APIRequestContext, expect, test } from "@playwright/test";
import { fail } from "assert";
import { getWalletsForIdentifiers } from "../../src/utils/navnetjener";
import { JONNY, NINA, commonTestSetup } from "../test-setup";

let captableAddress: string;
let orgnr: string;

test.beforeAll(async ({ request, baseURL }) => {
	if (!baseURL)
		throw new Error("No baseURL");

	const result = await commonTestSetup(1, baseURL);
	captableAddress = result.captableAddress;
	orgnr = result.orgnr;
});


test("BulkLookup API returns correct wallet addresses", async () => {
	// Prepare request data
	const identifiers = [NINA.IDENTIFIER, JONNY.IDENTIFIER]; // Nina (should be there) and Jonny (shoudn't be there)

	const res = await getWalletsForIdentifiers(identifiers, orgnr);

	// Validate the response
	expect(res).toHaveProperty('wallets');
	expect(res.wallets.length).toBe(2);


	// Validate wallet addresses based on your test expectations
	// Check if Nina's wallet is a valid Ethereum address
	const findWalletByIdentifier = (identifier: string) => res.wallets.find(w => w.identifier === identifier);
	const ethereumAddressPattern = /^0x[a-fA-F0-9]{40}$/;

	const ninaWallet = findWalletByIdentifier(NINA.IDENTIFIER);
	const jonnyWallet = findWalletByIdentifier(JONNY.IDENTIFIER);

	if (ninaWallet) {
		expect(ninaWallet.walletAddress).toMatch(ethereumAddressPattern);
	} else {
		fail("Nina's wallet should be defined");
	}

	if (jonnyWallet) {
		expect(jonnyWallet.walletAddress).toBeNull();  // Assuming that a not-found wallet has its address set to null
	} else {
		fail("Jonny's wallet should be defined");
	}
});

