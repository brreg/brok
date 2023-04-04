import { CapTable, ERC5564Registry__factory } from '@brok/captable';
import { ethers } from 'ethers';
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from '../../../../contants';
import {
  formatPublicKeyForSolidityBytes,
  getStealthAddress,
  getAnnoncements,
  getSharedSecret,
  getRecoveryPrivateKey,
  signatureToStealthKeys,
} from '../../../../utils/stealth';
import debug from 'debug';
import { ApiError } from 'next/dist/server/api-utils';
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R, ConnectToStealthAddressFactory_RW } from '../../../../utils/blockchain';
import { ErrorResponse, ApiRequestLogger } from '../../../../utils/api';

const log = debug('brok:shareholder:register');
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
  try {
    ApiRequestLogger(req, log);
    switch (req.method) {
      case 'GET': {
        // Find all companies
        const capTableRegistry = await ConnectToCapTableRegistry_R();
        const allCapTablesAddresses = await capTableRegistry.getCapTableList();
        const allCapTables = await getDetailsFromCapTables(allCapTablesAddresses)

        log(`found ${allCapTables.length} CapTables`);
        log(`HTTP Response 200, return list with ${allCapTables.length} captables`);
        return res.status(200).json({ allCapTables });
      }
      case 'POST': {
        // Register a new captable for company
      }
      default:
        res.setHeader('Allow', ['GET', 'POST']);
        res.status(405).end(`Method ${req.method} Not Allowed`);
    }
  } catch (error) {
    ErrorResponse(error, log, res);
  }
}

async function getDetailsFromCapTables(captables: string[]): Promise<any[]> {
  const promise = captables.map(async (capTableAddress: string) => {
    const captable = await ConnectToCapTable_R(capTableAddress);
    return {
      orgnr: await captable.orgnr(),
      name: await captable.name(),
      ethAddress: capTableAddress,
    };
  });
  return Promise.all(promise)
}

function parseBody(body: any) {
  if (!('signature' in body)) {
    throw new ApiError(400, 'No signature provided in body');
  }
  if (!('spendPublicKey' in body)) {
    throw new ApiError(400, 'No spendPublicKey provided in body');
  }

  const signature: string = body.signature.toString();
  const spendPublicKey: string = body.spendPublicKey.toString();
  const isValidSignature = (sig: string) => ethers.utils.isHexString(sig) && sig.length === 132;
  if (!isValidSignature(signature)) {
    throw new ApiError(400, `Invalid signature: ${signature}`);
  }

  return { signature, spendPublicKey };
}
