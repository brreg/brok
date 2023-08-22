import { CapTable, ERC5564Registry__factory } from '@brok/captable';
import { ethers } from 'ethers';
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from '../../../../../contants';
import {
  formatPublicKeyForSolidityBytes,
  getStealthAddress,
  getAnnoncements,
  getSharedSecret,
  getRecoveryPrivateKey,
  signatureToStealthKeys,
} from '../../../../../utils/stealth';
import debug from 'debug';
import { ApiError } from 'next/dist/server/api-utils';
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R, ConnectToStealthAddressFactory_RW } from '../../../../../utils/blockchain';
import { ErrorResponse, ApiRequestLogger } from '../../../../../utils/api';
import { getCapTableWithOrgnr } from '../../../../../utils/the-graph';

const log = debug('brok:api:v1:company:[id]');
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
  try {
    ApiRequestLogger(req, log);
    switch (req.method) {
      case 'GET': {
        // Find info about company
        const { orgnr } = parseQuery(req.query)
        const captable = await getCapTableWithOrgnr(orgnr)
        log("captable:", captable)
        if (!captable) {
          throw new ApiError(404, `Could not find any company with orgnr ${orgnr} in BRØK` )
        }

        // const name = await captable.name()
        // const totalSupply = ethers.utils.formatEther(await captable.totalSupply())

        log("HTTP Response 200, return captable")
        return res.status(200).json({
          captable
        })

      }
      case 'PUT': {
        // Update captable info for company
      }
      case 'DELETE': {
        // Delete company
      }
      default:
        res.setHeader('Allow', ['GET', 'PUT', 'DELETE']);
        res.status(405).end(`Method ${req.method} Not Allowed`);
    }
  } catch (error) {
    ErrorResponse(error, log, res);
  }
}

// async function findCapTableWithOrgnr(orgnr: string): Promise<CapTable | undefined> {
//   const captable = await getCapTableWithOrgnr(orgnr)

//   const capTableRegistry = await ConnectToCapTableRegistry_R();
//   const allCapTablesAddresses = await capTableRegistry.getCapTableList();
//   for (const address of allCapTablesAddresses) {
//     const captable = await ConnectToCapTable_R(address);
//     const nr = await captable.getOrgnr()
//     if (orgnr === nr) {
//       return captable
//     }
//   }
// }

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

function parseQuery(
  query: Partial<{
    [key: string]: string | string[];
  }>,
) {
  if (!query.orgnr) {
    throw new ApiError(400, 'missing orgnr');
  }
  // if (typeof query.orgnr !== 'string') {
  //   throw new ApiError(400, 'orgnr must be provided as a string. e.g "112233445"');
  // }
  // if (query.orgnr.length !== 9) {
  //   throw new ApiError(400, 'orgnr must be nine digits');
  // }
  try {
    parseInt(query.orgnr.toString())
  } catch (error) {
    throw new ApiError(400, 'orgnr must be a valid number');
  }
  const orgnr: string = query.orgnr.toString();
  return { orgnr };
}