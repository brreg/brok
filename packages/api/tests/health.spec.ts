/**
 * @fileoverview This test suite is designed to test the health of the API by sending a GET request to the `/api/health` endpoint.
 *
 * If needed, you can load environment variables before running tests using `loadEnvConfig`.
 * Uncomment the `test.beforeAll()` block for that.
 */

import { test, expect } from "@playwright/test";

// Uncomment below if you need to load environment variables
// test.beforeAll(() => {
// 	const projectDir = process.cwd();
// 	loadEnvConfig(projectDir);
// });

/**
 * Test the `/api/health` endpoint.
 *
 * This test sends a GET request to the `/api/health` endpoint and expects:
 * 1. The status to be OK.
 * 2. The response to be a JSON object.
 * 3. The JSON object to contain a `status` key with the value `ok`.
 */
test("/api/health should return status ok", async ({ request, baseURL }) => {
	const res = await request.get(`${baseURL}/api/health`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({}),
	});
	expect(res.ok(), JSON.stringify(res.statusText())).toBe(true);

	const json = await res.json();
	expect(json, "json object should be defined").toBeDefined();
	expect(typeof json).toBe("object");
	expect(Object.keys(json).length, `json should have properties${JSON.stringify(json)}`).toBeGreaterThan(0);
	expect("status" in json, "json object should have property status").toBe(true);
	expect(json.status, "Status should be ok").toBe("ok");
});
