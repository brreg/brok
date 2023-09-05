import { test, expect } from "@playwright/test";
import { getForetakByOrgnr, getForetakByPnr } from "../../src/utils/navnetjener";

test("Should receive captable from navnetjener", async () => {
  // Test data from ut.regsys
  const res = await getForetakByOrgnr("310780472");
  expect(res).toBeTruthy();
  expect(res.id).toBe("0x03c581e3f2c9532b4cfba794eddc4ec5f4b30fd6");
});

test("Should receive all companies for a person", async () => {
  const res = await getForetakByPnr("2105800000");
  expect(res).toBeTruthy();
  expect(res.length).toBe(2);
  expect(res[0].id).toBe("0x03c581e3f2c9532b4cfba794eddc4ec5f4b30fd6");
});