import { test, expect } from "@playwright/test";
import { getForetakByOrgnr } from "../../src/utils/navnetjener";

test("Should receive captable from navnetjener", async () => {
  // Test data from ut.regsys
  const res = await getForetakByOrgnr("310780472");
  expect(res).toBeTruthy();
  expect(res.id).toBe("0x03c581e3f2c9532b4cfba794eddc4ec5f4b30fd6");
});