import debug from "debug";
import { THE_GRAPH_URL } from "../contants";
import { ethers } from "ethers";
const log = debug("brok:utils:the-graph");

export async function getCapTablesForAddresses(addresses: string[]) {
	const query = GraphQLQueries.CAP_TABLES_FOR_ADDRESSES(addresses);
	let validResponse: undefined | TokenHolders = undefined;

	// create a timer that fires one requst each second for 5 seconds, then stops
	const fetchData = async () => {
		const res = await fetch(THE_GRAPH_URL, {
			method: "post",
			body: JSON.stringify({
				query,
			}),
			headers: { "Content-Type": "application/json" },
		});
		if (validResponse) {
			return;
		}
		if (!res.ok) {
			log("Error from the graph: ", res);
		} else {
			const data = (await res.json()) as CAP_TABLES_FOR_ADDRESSES_RESPONSE;
			log("Data from the graph: ", data.data);
			if (data.data.tokenHolders.length > 0) {
				validResponse = data.data;
			} else {
				log("No data from the graph");
			}
		}
	};
	const interval = setInterval(fetchData, 1000);
	setTimeout(() => clearInterval(interval), 5000);
	while (validResponse === undefined) {
		await new Promise((resolve) => setTimeout(resolve, 500));
	}
	if (!validResponse) {
		throw new Error("Error fetching data from the graph");
	} else {
		return formatGraphResponse(validResponse) as unknown as TokenHolders;
	}
}

function formatGraphResponse(data: Record<string, string | string[] | Record<string, any>>): Record<string, any> {
	const maybeFormatValue = (value: string) => {
		if (typeof value === "string") {
			try {
				const maybeBN = ethers.BigNumber.from(value);
				if (maybeBN) {
					return ethers.utils.formatEther(maybeBN);
				}
			} catch (error) {}
			try {
				const maybeAddress = ethers.utils.getAddress(value);
				if (maybeAddress) {
					return maybeAddress;
				}
			} catch (error) {}
		}
		return value;
	};

	const formattedData: Record<string, any> = {};

	for (const [key, value] of Object.entries(data)) {
		if (Array.isArray(value)) {
			formattedData[key] = value.map((v) => maybeFormatValue(v));
		} else if (typeof value === "object" && value !== null) {
			formattedData[key] = formatGraphResponse(value as Record<string, string | string[] | Record<string, any>>);
		} else {
			formattedData[key] = maybeFormatValue(value);
		}
	}
	return formattedData;
}

// biome-ignore lint/complexity/noStaticOnlyClass: <explanation>
export class GraphQLQueries {
	static CAP_TABLES_FOR_ADDRESSES(addresses: string[]) {
		return `
      {
        tokenHolders(where: {address_in: [${addresses.map((a) => `"${a.toLowerCase()}"`).join(", ")}]}) {
            address
            balances {
                amount
                partition
            }
            capTable {
                id
                name
                symbol
                partitions
                status
                registry {
                id
                }
                totalSupply
                owner
                minter
                controllers
                orgnr
            }
          }
        }
        `;
	}
}
export interface CAP_TABLES_FOR_ADDRESSES_RESPONSE {
	data: TokenHolders;
}
export interface TokenHolders {
	tokenHolders: TokenHolder[];
}

export interface TokenHolder {
	address: string;
	balances: Balance[];
	capTable: CapTable;
}

export interface Balance {
	amount: string;
	partition: string;
}

export interface CapTable {
	id: string;
	name: string;
	symbol: string;
	partitions: string[];
	status: string;
	registry: Registry;
	totalSupply: string;
	owner: string;
	minter: string;
	controllers: string[];
	orgnr: string;
}

export interface Registry {
	id: string;
}
