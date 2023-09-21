import { test, expect } from "@playwright/test";
import { WalletRecordInNavnetjener, createWalletRecord, getAmountOfSharesForOwner, getForetakByOrgnr, getForetakOwnedByFnrOrOrgnr, getWalletsForIdentifiers } from "../../src/utils/navnetjener";
import { ethers } from "ethers";
import { fail } from "assert";

/**

!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----!
!																																											 !
! 	Dette settet med tester fungerer KUN når navnetjeneren kjører lokalt på port 9292	 !
! 	Husk å kjøre deploy local tasken før du tester navnetjenren                        ! 
!   https://github.com/brreg/brok-navnetjener                                          !
!																																											 !
!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----OBS!-----!


* [GIN-debug] POST   /v1/wallet                         --> brok/navnetjener/api.CreateWallet (3 handlers)
* [GIN-debug] GET    /v1/wallet/:walletAddress          --> brok/navnetjener/api.GetWalletByWalletAddress (3 handlers)

* [GIN-debug] GET    /v1/aksjeeier/:id                  --> brok/navnetjener/api.GetAllForetakForAksjeeier (3 handlers)

* [GIN-debug] GET    /v1/aksjebok/                      --> brok/navnetjener/api.GetForetak (3 handlers)
* [GIN-debug] GET    /v1/aksjebok/:orgnr                --> brok/navnetjener/api.GetForetakByOrgnr (3 handlers)
* [GIN-debug] GET    /v1/aksjebok/:orgnr/balanse/:id    --> brok/navnetjener/api.GetNumberOfSharesForOwnerOfAForetak (3 handlers)
* [GIN-debug] POST   /v1/aksjebok/:orgnr/aksjeeier      --> brok/navnetjener/api.GetOwnersForForetak (3 handlers)
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

  const newWalletRecord: WalletRecordInNavnetjener[] = [
    {
      OwnerPersonFirstName: JONNY.FIRSTNAME,
      OwnerPersonLastName: JONNY.LASTNAME,
      OwnerPersonFnr: JONNY.IDENTIFIER,
      CapTableOrgnr: "000000001"
    }
  ]

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
  const res = await getForetakOwnedByFnrOrOrgnr(ELISE.IDENTIFIER);
  expect(res).toBeTruthy();
  expect(res.length).toBe(1);
  expect(res[0].id).toBe("0x462128b0a43a7b04e1c7f3e45039723fe70058ee");
});

test("Should find the amount of shares for a person", async () => {
  const res = await getAmountOfSharesForOwner("815493000", "310767859");
  expect(res).toBeTruthy();
  expect(res).toBe("1000");
});

test("BulkLookup API returns correct wallet addresses", async () => {
  // Prepare request data
  const identifiers = [NINA.IDENTIFIER, JONNY.IDENTIFIER]; // Nina (should be there) and Jonny (shoudn't be there)

  const res = await getWalletsForIdentifiers(identifiers, RYDDIG_BOBIL_AS.IDENTIFIER);
  console.dir(res, 5);

  // Validate the response
  expect(res).toHaveProperty('wallets');
  expect(res.wallets.length).toBe(2);

  // Create a helper function to find a wallet by identifier
  const findWalletByIdentifier = (identifier: string) => res.wallets.find(w => w.identifier === identifier);

  // Validate wallet addresses based on your test expectations
  // Check if Nina's wallet is a valid Ethereum address
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

