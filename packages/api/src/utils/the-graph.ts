import debug from "debug";
import { request, gql } from 'graphql-request';
import { THE_GRAPH_URL } from "../contants";
import { ethers } from "ethers";
const log = debug("brok:utils:the-graph");

export async function getCapTableWithShareholderAddresses(address: string) : Promise<TokenHolder> {
	const variables = {
		address: address.toLowerCase(),
	};
	const data = await request<{ tokenHolder: TokenHolder}>(THE_GRAPH_URL, GET_CAP_TABLES_FOR_ADDRESSES, variables);
	const tokenHolder = data.tokenHolder;
	log("Data from the graph: ", data);
	return tokenHolder;
}

export async function getCapTableWithOrgnr(orgnr: string) : Promise<CapTable[]> {
	const variables = {
		orgnr: orgnr,
	};
	const data = await request<{ capTables: CapTable[]}>(THE_GRAPH_URL, GET_CAP_TABLE_WITH_ORGNR, variables);
	const capTable = data.capTables;
	log("Data from the graph: ", data);
	return capTable;
}


const GET_CAP_TABLE_WITH_ORGNR = gql`
	query getCapTableForOrgnr($orgnr: String!) {
		capTables(where: { orgnr: $orgnr }) {
			id
			name
			symbol
			partitions
			status
			registry {
				id
			}
			tokenHolders {
				address
				balances {
					amount
					partition
				}
			}
			totalSupply
			owner
			minter
			controllers
			orgnr
		}
	}
`;

const GET_CAP_TABLES_FOR_ADDRESSES = gql`
	query getCapTablesForAddresses($addresses: [String!]!) {
		tokenHolders(where: { address_in: $addresses }) {
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
				tokenHolders {
					address
					balances {
						amount
						partition
					}
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
	amount: BigInt;
	partition: string;
}

export interface CapTable {
	id: string;
	name: string;
	symbol: string;
	partitions: string[];
	status: string;
	registry: Registry;
	tokenHolders: TokenHolder[];
	totalSupply: BigInt;
	owner: string;
	minter: string;
	controllers: string[];
	orgnr: string;
}

export interface Registry {
	id: string;
}
