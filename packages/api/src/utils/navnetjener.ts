import debug from 'debug';
import axios from 'axios';
import { ApiError } from 'next/dist/server/api-utils';
import { ethers } from 'ethers';
import { jsonHeader } from '../../tests/test-setup';
const log = debug('brok:utils:navnetjener');

const API_URL = process.env.NAVNETJENER_URL;
const API_VERSION = 'v1';
const API_BASE_URL = `${API_URL}/${API_VERSION}`;

type Wallet = {
  first_name: string;
  last_name: string;
  orgnr: number;
  birth_date: string;
  wallet_address: string;
};

type Person = {
  first_name: string;
  last_name: string;
  birth_date: string;
};

type Company = {
  name: string;
  orgnr: string;
};

type Owner = {
  person?: Person;
  company?: Company;
};

type Balance = {
  amount: string;
  partition: string;
};

type TokenHolder = {
  address: string;
  balances: Balance[];
  owner: Owner;
};

type Foretak = {
  id: string;
  name: string;
  orgnr: string;
  owner: string;
  partitions: string[];
  status: string;
  tokenHolders: TokenHolder[];
  totalSupply: string;
  minter: string;
};

export type WalletRecordInNavnetjener = {
  OwnerPersonFirstName?: string;
  OwnerPersonLastName?: string;
  OwnerPersonFnr?: string;
  OwnerCompanyName?: string;
  OwnerCompanyOrgnr?: string;
  CapTableOrgnr: string;
  WalletAddress?: string; // Wallets can be generated outside of this API e.g. self-custody
};

// TODO parentOrgnr evt. bare orgnr, burde vel være i URLen istedenfor å sendes som data
export type BulkLookupRequest = {
  identifiers: string[];
}

/*
walletAddress undefined: The field was not included in the response, perhaps because the query was just for balances.
walletAddress null: The query was specifically for a wallet, but none was found.
walletAddress string: A valid wallet address was found.
*/
export type WalletInfo = {
  identifier: string;
  walletAddress?: string | null;
  balance?: number | null;
};

export type BulkLookupResponse = {
  wallets: WalletInfo[];
};

/**
 * Create a shareholder record in navnetjener
 * TODO Consider making this operation atomic. If walletInfos gets a row with N/A as identifier, then the whole operation should be rolled back
 *
 * @param newWalletRecord
 * @returns WalletRecordInNavnetjener
 * @throws Error
 */
export async function createWalletRecord(newWalletRecords: WalletRecordInNavnetjener[]): Promise<BulkLookupResponse> {
  const jsonRecords: Array<Record<string, any>> = [];
  const walletInfos: WalletInfo[] = [];

  newWalletRecords.forEach((newWalletRecord) => {
    const walletAddress = ethers.Wallet.createRandom().address;

    jsonRecords.push({
      owner_person_first_name: newWalletRecord.OwnerPersonFirstName,
      owner_person_last_name: newWalletRecord.OwnerPersonLastName,
      owner_person_fnr: newWalletRecord.OwnerPersonFnr,
      owner_company_name: newWalletRecord.OwnerCompanyName,
      owner_company_orgnr: newWalletRecord.OwnerCompanyOrgnr,
      cap_table_orgnr: newWalletRecord.CapTableOrgnr,
      wallet_address: walletAddress
    });

    walletInfos.push({
      identifier: newWalletRecord.OwnerPersonFnr ?? newWalletRecord.OwnerCompanyOrgnr ?? "N/A",
      walletAddress: walletAddress
    });
  });


  try {
    await axios.post<WalletRecordInNavnetjener>(`${API_BASE_URL}/wallet`, jsonRecords, jsonHeader);
    return { wallets: walletInfos };
  } catch (error) {
    log(`Error creating new record for ${newWalletRecords}:`, error);
    throw error;
  }
}


/**
 * Get the owner data of a wallet address
 *
 * Throws ApiError if no wallet is found
 * @param walletAddress Wallet address
 * @returns Wallet
 * @throws ApiError
 */
export async function getWalletByAddress(walletAddress: string): Promise<Wallet> {
  try {
    const response = await axios.get<Wallet>(`${API_BASE_URL}/wallet/${walletAddress}`);
    return response.data;
  } catch (error) {
    log(`Error fetching wallet with address ${walletAddress}:`, error);
    throw new ApiError(404, `Could not find any wallet with address ${walletAddress} in BRØK`);
  }
}

/**
 * Get one company by orgnr
 *
 * Throws ApiError if no company is found
 * @param orgnr Organisasjonsnummer
 * @returns Foretak
 * @throws ApiError
 */
export async function getForetakByOrgnr(orgnr: string): Promise<Foretak> {
  try {
    const response = await axios.get<Foretak>(`${API_BASE_URL}/aksjebok/${orgnr}`);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error) && error.response && error.response.data) {
      log(`Error fetching foretak with orgnr ${orgnr}. Response from navnetjener:`, error.response.data);
    } else {
      log(`Error fetching foretak with orgnr ${orgnr}. NOT a Axios error:`, error);
    }
    throw new ApiError(404, `Could not find any company with orgnr ${orgnr} in BRØK`);
  }
}

/**
 * Get all companies owned by a person or organization
 *
 * Throws ApiError if no companies are found
 * @param fnrOrOrgnr Personnummer or Orgnr
 * @returns Array of Foretak
 * @throws ApiError
 */
export async function getForetakOwnedByFnrOrOrgnr(fnrOrOrgnr: string): Promise<Foretak[]> {
  try {
    const response = await axios.get<Foretak[]>(`${API_BASE_URL}/aksjeeier/${fnrOrOrgnr}`);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error) && error.response && error.response.data) {
      log(`Error fetching foretak with fnr ${fnrOrOrgnr}. Response from navnetjener:`, error.response.data);
    } else {
      log(`Error fetching foretak with fnr ${fnrOrOrgnr}. NOT a Axios error:`, error);
    }
    throw new ApiError(404, `Could not find any foretak for person with fnr ${fnrOrOrgnr} in BRØK`);
  }
}

/**
 * Get amount of shares owned by a person or organization in a captable
 *
 * Throws ApiError if no companies are found
 * @param capTableOrgnr Orgnr of the captable
 * @param fnrOrOrgnr Personnummer or Orgnr or the share owner
 * @returns Number of shares
 * @throws ApiError
 */
export async function getAmountOfSharesForOwner(capTableOrgnr: string, fnrOrOrgnr: string): Promise<string> {
  try {
    const response = await axios.get<string>(`${API_BASE_URL}/aksjebok/${capTableOrgnr}/balanse/${fnrOrOrgnr}`);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error) && error.response && error.response.data) {
      log(`Error fetching foretak with orgnr ${capTableOrgnr}.. Response from navnetjener:`, error.response.data);
    } else {
      log(`Error fetching foretak with orgnr ${capTableOrgnr}.. NOT a Axios error:`, error);
    }
    throw new ApiError(404, `Could not find any shares for ${fnrOrOrgnr} in CapTable ${capTableOrgnr} in BRØK`);
  }
}

/**
 * Get all companies
 *
 * Page is starting at 0
 *
 * Throws ApiError if navnetjener returns error
 * @param page Page number
 * @returns Array of Foretak
 * @throws ApiError
 */
export async function getAllForetak(page: number): Promise<Foretak[]> {
  try {
    const response = await axios.get<Foretak[]>(`${API_BASE_URL}/aksjebok?page=${page}`);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error) && error.response && error.response.data) {
      log("Error fetching foretak. Response from navnetjener:", error.response.data);
    } else {
      log("Error fetching foretak. NOT a Axios error:", error);
    }
    throw new ApiError(404, "Could not find any foretak in BRØK");
  }
}

/**
 * Get all wallets for given identifiers
 *
 * Throws ApiError if the server returns an error
 *
 * @param identifiers Array of Identifiers (fødselsnummer or orgnr)
 * @param parentOrgnr Parent organization number
 * @returns BulkLookupResponse containing mapping of identifier to wallet addresses
 * @throws ApiError
 */
export async function getWalletsForIdentifiers(identifiers: string[], parentOrgnr: string): Promise<BulkLookupResponse> {
  const requestData: BulkLookupRequest = {
    identifiers
  };

  try {
    const response = await axios.post<BulkLookupResponse>(`${API_BASE_URL}/aksjebok/${parentOrgnr}/aksjeeier`, requestData);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      console.log("Error fetching wallets. Response from server:", error.response?.data);
    } else {
      console.log("Error fetching wallets. NOT an Axios error:", error);
    }
    throw new Error("Could not fetch wallets");
  }
}

/**
 * Get balances for given identifiers
 *
 * Throws ApiError if the server returns an error
 *
 * @param identifiers Array of Identifiers (wallet addresses or other)
 * @returns BulkBalanceResponse containing mapping of identifier to balances
 * @throws ApiError
 */
export async function balanceOfIdentifiers(requestData: BulkLookupRequest): Promise<BulkLookupResponse> {
  try {
    const response = await axios.post<BulkLookupResponse>(`${API_BASE_URL}/balances`, requestData);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      console.log("Error fetching balances. Response from server:", error.response?.data);
    } else {
      console.log("Error fetching balances. NOT an Axios error:", error);
    }
    throw new Error("Could not fetch balances");
  }
}

