/**
 * Cap Table Find By Orgnr Test
 *
 * Path: /api/v1/company/:orgnr
 * Method: GET
 *
 * Helper functions:
 * @function CreateNewCapTable
 *
 * Test:
 * - should find captable by orgnr: Ensures that a cap table can be fetched by its orgnr. Validates multiple
 *   properties of the returned JSON object such as 'name', 'orgnr', and 'totalSupply'.
 *
 * Key points checked in the test:
 * - HTTP response should be OK.
 * - The returned JSON should be defined and of type 'object'.
 * - The JSON should have key properties such as 'name', 'orgnr', and 'totalSupply'.
 * - The orgnr in the returned JSON should match the orgnr used for querying.
 */

import { expect, test } from "@playwright/test";
import { CreateNewCapTable } from "../utils";

test("should find captable by orgnr", async ({ request, baseURL }) => {
	const { orgnr } = await CreateNewCapTable();

	// TODO Find a way to wait for graphql to be ready
	await new Promise(resolve => setTimeout(resolve, 2000));

	const res = await request.get(`${baseURL}/api/v1/company/${orgnr}`, {
		headers: {
			"Content-Type": "application/json",
		},
	});

	expect(res).toBeOK();
	const json = await res.json();
	const foretak = json.foretak;

	expect(foretak, "json object should be defined").toBeDefined();
	expect(typeof foretak).toBe("object");
	expect(Object.keys(foretak).length, `json should have properties ${JSON.stringify(foretak)}`).toBeGreaterThan(0);
	expect("name" in foretak, "json object should have property name").toBe(true);
	expect("orgnr" in foretak, "json object should have property orgnr").toBe(true);
	expect("totalSupply" in foretak, "json object should have property totalSupply").toBe(true);
	expect(foretak.orgnr, "json property success should be true").toBe(orgnr);
});
