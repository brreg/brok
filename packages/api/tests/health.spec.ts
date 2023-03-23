import { test, expect } from "@playwright/test";
import { ethers } from "ethers";

// test.beforeAll(() => {
// 	const projectDir = process.cwd();
// 	loadEnvConfig(projectDir);
// });

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
	expect(Object.keys(json).length, "json should have properties" + JSON.stringify(json)).toBeGreaterThan(0)
	expect("status" in json, "json object should have property status").toBe(true);
	expect(json.status, "Status should be ok").toBe("ok");
});
