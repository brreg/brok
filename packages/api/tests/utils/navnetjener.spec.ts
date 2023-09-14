import { test, expect } from "@playwright/test";
import { WalletRecordInNavnetjener, createWalletRecord, getForetakByOrgnr, getForetakByFnr, getWalletsForIdentifiers } from "../../src/utils/navnetjener";
import { ethers } from "ethers";

/**

!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----!
|																																											 |
| 	Dette settet med tester fungerer KUN når navnetjeneren kjører lokalt på port 9292	 |
| 	Husk å kjøre deploy local tasken før du tester navnetjenren                        | 
|   https://github.com/brreg/brok-navnetjener                                          |
|																																											 |
|-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----!


[GIN-debug] GET    /v1/wallet/:walletAddress --> brok/navnetjener/api.GetWalletByWalletAddress (4 handlers)
[GIN-debug] POST   /v1/wallet                --> brok/navnetjener/api.CreateWallet (4 handlers)
[GIN-debug] POST   /v1/wallets/bulk          --> brok/navnetjener/api.GetWalletsForIdentifiers (4 handlers)
[GIN-debug] GET    /v1/person/:fnr           --> brok/navnetjener/api.GetAllForetakForPerson (4 handlers)
[GIN-debug] GET    /v1/foretak/:orgnr        --> brok/navnetjener/api.GetForetakByOrgnr (4 handlers)
[GIN-debug] GET    /v1/foretak               --> brok/navnetjener/api.GetForetak (4 handlers)
*/

const JONNY = {
  IDENTIFIER: "18998612345",
  FIRSTNAME: "Jonny",
  LASTNAME: "Bravo",
};
const NINA = {
  IDENTIFIER: "15097600002",
  FIRSTNAME: "Nina",
  LASTNAME: "Pedersen",
};

const ELISE = {
  IDENTIFIER: "21058000000",
  FIRSTNAME: "Elise",
  LASTNAME: "Berg",
};

const RYDDIG_BOBIL_AS = {
  IDENTIFIER: "815493000",
  NAME: "RYDDIG BOBIL AS",
};

test("Should create a new wallet record in navnetjener", async () => {
  const wallet = ethers.Wallet.createRandom()

  const newWalletRecord: WalletRecordInNavnetjener = {
    OwnerPersonFirstName: JONNY.FIRSTNAME,
    OwnerPersonLastName: JONNY.LASTNAME,
    OwnerPersonFnr: JONNY.IDENTIFIER,
    WalletAddress: wallet.address,
    CapTableOrgnr: "000000001" //What captable is this?
  }

  const res = await createWalletRecord(newWalletRecord);
  expect(res).toBeTruthy();
});

test("Should receive captable from navnetjener", async () => {
  // Test data from ut.regsys
  const res = await getForetakByOrgnr(RYDDIG_BOBIL_AS.IDENTIFIER);
  expect(res).toBeTruthy();
  expect(res.id).toBe("0x462128b0a43a7b04e1c7f3e45039723fe70058ee");
});

test("Should receive all companies for a person", async () => {
  const res = await getForetakByFnr(ELISE.IDENTIFIER);
  expect(res).toBeTruthy();
  expect(res.length).toBe(1);
  expect(res[0].id).toBe("0x462128b0a43a7b04e1c7f3e45039723fe70058ee");
});

test("BulkLookup API returns correct wallet addresses", async () => {
  // Prepare request data
  const identifiers = [NINA.IDENTIFIER, JONNY.IDENTIFIER]; // Nina (should be there) and Jonny (shoudn't be there)

  const res = await getWalletsForIdentifiers(identifiers, RYDDIG_BOBIL_AS.IDENTIFIER);

  // Validate the response
  expect(res).toHaveProperty('wallets');
  expect(Object.keys(res.wallets).length).toBe(2);

  // Validate wallet addresses based on your test expectations
  // Check if Nina's wallet is a valid Ethereum address
  const ethereumAddressPattern = /^0x[a-fA-F0-9]{40}$/;
  expect(res.wallets[NINA.IDENTIFIER]).toMatch(ethereumAddressPattern);
  expect(res.wallets[JONNY.IDENTIFIER]).toBeDefined();
});
