import debug from "debug";
import axios from 'axios';
import { ApiError } from "next/dist/server/api-utils";
const log = debug("brok:utils:navnetjener");

const API_URL = process.env.NAVNETJENER_URL;
const API_VERSION = "v1";
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

type Balance = {
  amount: string;
  partition: string;
};

type TokenHolder = {
  address: string;
  balances: Balance[];
  person: Person;
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
    const response = await axios.get<Wallet>(API_BASE_URL + "/wallets/" + walletAddress);
    return response.data;
  } catch (error) {
    log(`Error fetching wallet with address ${walletAddress}:`, error);
    throw error;
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
    const response = await axios.get<Foretak>(API_BASE_URL + "/foretak/" + orgnr);
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
    const response = await axios.get<Foretak[]>(API_BASE_URL + "/person/" + pnr);
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