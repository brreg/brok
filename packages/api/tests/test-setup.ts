import { ethers } from 'ethers';
import { GenerateRandomCompanyName, GenerateRandomOrgnr } from './utils';
import axios from 'axios';
import { BulkLookupResponse, WalletRecordInNavnetjener, createWalletRecord } from '../src/utils/navnetjener';
import { ForetakResponse } from '../src/pages/api/v1/company';

export const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");

export const JONNY = {
    IDENTIFIER: "18998612345",
    FIRSTNAME: "Jonny",
    LASTNAME: "Bravo",
};

export const NINA = {
    IDENTIFIER: "15097600002",
    FIRSTNAME: "Nina",
    LASTNAME: "Pedersen",
};

export const jsonHeader = {
    headers: {
        'Content-Type': 'application/json',
    },
};

async function createCapTable(baseURL: string, orgnr: string): Promise<string> {
    const data = { name: GenerateRandomCompanyName(), orgnr };
    const createRes = await axios.post<ForetakResponse>(`${baseURL}/api/v1/company/`, data);
    return createRes.data.capTableAddress;
}

async function createWalletRecords(baseURL: string, numOfWalletRecords: number, orgnr: string): Promise<string> {
    const walletRecords: WalletRecordInNavnetjener[] = [];

    const ninaWalletRecord: WalletRecordInNavnetjener = {
        OwnerPersonFirstName: NINA.FIRSTNAME,
        OwnerPersonLastName: NINA.LASTNAME,
        OwnerPersonFnr: NINA.IDENTIFIER,
        CapTableOrgnr: orgnr,
    };
    walletRecords.push(ninaWalletRecord);

    if (numOfWalletRecords === 2) {
        const jonnyWalletRecord: WalletRecordInNavnetjener = {
            OwnerPersonFirstName: JONNY.FIRSTNAME,
            OwnerPersonLastName: JONNY.LASTNAME,
            OwnerPersonFnr: JONNY.IDENTIFIER,
            CapTableOrgnr: orgnr,
        };
        walletRecords.push(jonnyWalletRecord);
    } if (numOfWalletRecords > 2) {
        throw new Error("createWalletRecords() only supports 1 or 2 wallet records");
    }

    // const res = await createWalletRecord(walletRecords);

    const res = await axios.post<BulkLookupResponse>(`${baseURL}/api/v1/company/${orgnr}/opprett-lommeboker`, JSON.stringify(walletRecords), jsonHeader);
    const ninaWalletAddress = res.data.wallets[0]?.walletAddress ?? "ERROR";

    return ninaWalletAddress;
}

async function populateCapTableWithShareholders(numOfWalletRecords: number, baseURL: string, orgnr: string): Promise<void> {
    const mottakere = [NINA.IDENTIFIER];
    const antall = [30000];

    if (numOfWalletRecords === 2) {
        mottakere.push(JONNY.IDENTIFIER);
        antall.push(3333);
    }

    await axios.post(`${baseURL}/api/v1/company/${orgnr}/kapitalforhoyelse`, JSON.stringify({ mottakere, antall }), jsonHeader);
}


export async function commonTestSetup(numOfWalletRecords: number, baseURL: string): Promise<{ captableAddress: string, orgnr: string, ninaWalletAddress: string }> {
    try {
        const orgnr = GenerateRandomOrgnr().toString();
        const captableAddress = await createCapTable(baseURL, orgnr);
        const ninaWalletAddress = await createWalletRecords(baseURL, numOfWalletRecords, orgnr);
        await populateCapTableWithShareholders(numOfWalletRecords, baseURL, orgnr);
        return { orgnr, captableAddress, ninaWalletAddress };
    } catch (error) {
        console.error("An error occurred in commonTestSetup:", error);
        throw error;
    }
}
