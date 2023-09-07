import debug from 'debug';
import axios from 'axios';
import { ApiError } from 'next/dist/server/api-utils';
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
  person: Person | undefined;
  company: Company | undefined;
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
  OwnerPersonPnr?: string;
  OwnerCompanyName?: string;
  OwnerCompanyOrgnr?: string;
  CapTableOrgnr: string;
  WalletAddress: string;
};

/**
 * Create a shareholder record in navnetjener
 *
 * @param newWalletRecord
 * @returns WalletRecordInNavnetjener
 * @throws Error
 */
export async function createWalletRecord(newWalletRecord: WalletRecordInNavnetjener) {
  const jsonRecord = {
    "owner_person_first_name": newWalletRecord.OwnerPersonFirstName,
    "owner_person_last_name": newWalletRecord.OwnerPersonLastName,
    "owner_person_pnr": newWalletRecord.OwnerPersonPnr,
    "owner_company_name": newWalletRecord.OwnerCompanyName,
    "owner_company_orgnr": newWalletRecord.OwnerCompanyOrgnr,
    "cap_table_orgnr": newWalletRecord.CapTableOrgnr,
    "wallet_address": newWalletRecord.WalletAddress
  };
  const customHeader = {
    headers: {
      'Content-Type': 'application/json',
    },
  };
  try {
    const response = await axios.post<WalletRecordInNavnetjener>(API_BASE_URL + '/wallet', jsonRecord, customHeader);
    return response.data;
  } catch (error) {
    log(`Error creating new record for ${newWalletRecord}:`, error);
    throw error;
  }
}

/**
 * Get one wallet by address
 *
 * Throws ApiError if no wallet is found
 * @param walletAddress Wallet address
 * @returns Wallet
 * @throws ApiError
 */
export async function getWalletByAddress(walletAddress: string): Promise<Wallet> {
  try {
    const response = await axios.get<Wallet>(API_BASE_URL + '/wallets/' + walletAddress);
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
    const response = await axios.get<Foretak>(API_BASE_URL + '/foretak/' + orgnr);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      log(`Error fetching foretak with orgnr ${orgnr}. Response from navnetjener:`, error.response!.data!);
    } else {
      log(`Error fetching foretak with orgnr ${orgnr}. NOT a Axios error:`, error);
    }
    throw new ApiError(404, `Could not find any company with orgnr ${orgnr} in BRØK`);
  }
}

/**
 * Get all companies for a person
 *
 * Throws ApiError if no companies are found
 * @param pnr Personnummer
 * @returns Array of Foretak
 * @throws ApiError
 */
export async function getForetakByPnr(pnr: string): Promise<Foretak[]> {
  try {
    const response = await axios.get<Foretak[]>(API_BASE_URL + '/person/' + pnr);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      log(`Error fetching foretak with pnr ${pnr}. Response from navnetjener:`, error.response!.data!);
    } else {
      log(`Error fetching foretak with pnr ${pnr}. NOT a Axios error:`, error);
    }
    throw new ApiError(404, `Could not find any foretak for person with pnr ${pnr} in BRØK`);
  }
}

/**
 * Get all companies
 *
 * Page is starting at 0
 *
 * Throws ApiError navnetjener returns error
 * @param page Page number
 * @returns Array of Foretak
 * @throws ApiError
 */
export async function getAllForetak(page: number): Promise<Foretak[]> {
  try {
    const response = await axios.get<Foretak[]>(API_BASE_URL + '/foretak?page=' + page);
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      log(`Error fetching foretak. Response from navnetjener:`, error.response!.data!);
    } else {
      log(`Error fetching foretak. NOT a Axios error:`, error);
    }
    throw new ApiError(404, `Could not find any foretak in BRØK`);
  }
}
