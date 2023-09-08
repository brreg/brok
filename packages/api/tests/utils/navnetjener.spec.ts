import { test, expect } from "@playwright/test";
import { WalletRecordInNavnetjener, createWalletRecord, getForetakByOrgnr, getForetakByFnr } from "../../src/utils/navnetjener";
import { ethers } from "ethers";

test("Should create a new wallet record in navnetjener",async () => {
  const wallet = ethers.Wallet.createRandom()
  const newWalletRecord: WalletRecordInNavnetjener = {
    OwnerPersonFirstName: "Jonny",
    OwnerPersonLastName: "Bravo",
    OwnerPersonFnr: "189912334",
    WalletAddress: wallet.address,
    CapTableOrgnr: "000000001"
  }
  const res = await createWalletRecord(newWalletRecord);
  expect(res).toBeTruthy();
});

test("Should receive captable from navnetjener", async () => {
  // Test data from ut.regsys
  const res = await getForetakByOrgnr("815493000");
  expect(res).toBeTruthy();
  console.log(res)
  expect(res.id).toBe("0x462128b0a43a7b04e1c7f3e45039723fe70058ee");
});

test("Should receive all companies for a person", async () => {
  const res = await getForetakByFnr("21058000000");
  expect(res).toBeTruthy();
  expect(res.length).toBe(1);
  expect(res[0].id).toBe("0x462128b0a43a7b04e1c7f3e45039723fe70058ee");
});