/**
# API Documentation for Share Issuance Service

## Overview

This API is responsible for issuing new shares in a company's capital table. It allows you to specify the organizational number (orgnr), share classes (aksjeklasser), recipient addresses (mottakerAdresser), and the number of shares (antall) to be issued for each recipient.

## API Endpoint

**Method**: POST  
**Endpoint**: The specific URL for the API will be where this code is deployed. The `orgnr` should be passed as a query parameter.

### Request Parameters

1. **orgnr** (Query Parameter)
    - Type: String
    - Description: The organizational number of the company.

2. **Body**

    - **aksjeklasser**
        - Type: Array of Strings
        - Description: Specifies the classes/types of shares being issued.
        - Example: `["Class A", "Class B"]`
        
    - **mottakerAdresser**
        - Type: Array of Strings
        - Description: Ethereum addresses of the recipients to whom the new shares are to be issued.
        - Example: `["0xabc...", "0xdef..."]`
        
    - **antall**
        - Type: Array of Strings
        - Description: Specifies the number of shares to issue for each recipient address. The number of elements must match the `mottakerAdresser` array.
        - Example: `["10", "20"]`

### Request Example

```json
POST /v1/company/123456789/kapitalforhoyelse
{
    "aksjeklasser": ["Class A", "Class B"],
    "mottakerAdresser": ["0xabc...", "0xdef..."],
    "antall": ["10", "20"]
}
```

### Response Format

- HTTP Status Code: 200 OK on success, error codes on failure.
- Response Body: JSON object containing the message and details about the transaction.

#### Response Example (Success)

```json
{
    "message": "Successfully issued 30 new shares to 2 addresses"
}
```

## Errors

- **404 Not Found**: Could not find any company with the provided `orgnr`.
- **400 Bad Request**: If required fields are missing or invalid.

### Example (Error)

```json
{
    "message": "Could not find any company with orgnr 123456789 in BRØK",
    "statusCode": 404
}
```
 */

import { CapTable } from "@brok/captable";
import debug from "debug";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";
import { ApiRequestLogger, ErrorResponse } from "../../../../../utils/api";
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R } from "../../../../../utils/blockchain";
import { GET_PROVIDER, WALLET } from "../../../../../contants";

const log = debug("brok:api:v1:company:[id]");
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	try {
		ApiRequestLogger(req, log);

		// TODO Hvis aksjeklassen ikke finnes: fail. Eget ende-punkt for å opprette aksjeklasse
		switch (req.method) {
			case "POST": {
				// Find info about company
				const { orgnr } = parseQuery(req.query);
				const { aksjeklasser, mottakerAdresser, antall } = req.body;

				const captable = await findCapTableWithOrgnr(orgnr);

				if (!captable) {
					throw new ApiError(404, `Could not find any company with orgnr ${orgnr} in BRØK`);
				}

				const wallet = WALLET.connect(GET_PROVIDER());
				const captable_RW = captable.connect(wallet);

				// Convert each element in aksjeklasser array to bytes32 format
				const partitions = aksjeklasser.map((klass: string) => ethers.utils.formatBytes32String(klass));

				// Issue
				await captable_RW.kapitalforhoyselse_nye_aksjer(partitions, mottakerAdresser, antall, "0x11");

				const sum: number = antall.reduce(
					(accumulator: number, currentValue: string) => accumulator + parseInt(currentValue, 10),
					0,
				);

				return res.status(200).json({
					message: `Successfully issued ${sum} new shares to ${mottakerAdresser.length} addresses`,
				});
			}

			default:
				res.setHeader("Allow", ["GET", "PUT", "DELETE"]);
				res.status(405).end(`Method ${req.method} Not Allowed`);
		}
	} catch (error) {
		ErrorResponse(error, log, res);
	}
}

async function findCapTableWithOrgnr(orgnr: string): Promise<CapTable | undefined> {
	const capTableRegistry = await ConnectToCapTableRegistry_R();
	const allCapTablesAddresses = await capTableRegistry.getCapTableList();

	for (const address of allCapTablesAddresses) {
		const captable = await ConnectToCapTable_R(address);
		const nr = await captable.getOrgnr();
		if (orgnr === nr) {
			return captable;
		}
	}
}

function parseBody(body: any) {
	if (!("signature" in body)) {
		throw new ApiError(400, "No signature provided in body");
	}

	if (!("spendPublicKey" in body)) {
		throw new ApiError(400, "No spendPublicKey provided in body");
	}

	const signature: string = body.signature.toString();
	const spendPublicKey: string = body.spendPublicKey.toString();
	const isValidSignature = (sig: string) => ethers.utils.isHexString(sig) && sig.length === 132;

	if (!isValidSignature(signature)) {
		throw new ApiError(400, `Invalid signature: ${signature}`);
	}

	return { signature, spendPublicKey };
}

function parseQuery(
	query: Partial<{
		[key: string]: string | string[];
	}>,
) {
	if (!query.orgnr) {
		throw new ApiError(400, "missing orgnr");
	}
	if (typeof query.orgnr !== "string") {
		throw new ApiError(400, 'orgnr must be provided as a string. e.g "112233445"');
	}
	if (query.orgnr.length !== 9) {
		throw new ApiError(400, "orgnr must be nine digits");
	}
	try {
		parseInt(query.orgnr.toString());
	} catch (error) {
		throw new ApiError(400, "orgnr must be a valid number");
	}

	const orgnr: string = query.orgnr.toString();
	return { orgnr };
}
