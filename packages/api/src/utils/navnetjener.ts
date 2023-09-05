import debug from "debug";
import axios from 'axios';
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

async function getWalletByAddress(walletAddress: string): Promise<Wallet> {
  try {
    const response = await axios.get<Wallet>(API_BASE_URL + "/wallets/" + walletAddress);
    return response.data;
  } catch (error) {
    log(`Error fetching wallet with address ${walletAddress}:`, error);
    throw error;
  }
}

async function getForetakByOrgnr(orgnr: string): Promise<Foretak> {
  try {
    const response = await axios.get<Foretak>(API_BASE_URL + "/foretak/" + orgnr);
    return response.data;
  } catch (error) {
    log(`Error fetching foretak with orgnr ${orgnr}:`, error);
    throw error;
  }
}

export { getWalletByAddress, getForetakByOrgnr };