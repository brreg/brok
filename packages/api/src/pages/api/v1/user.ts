import { ERC5564Registry__factory } from '@brok/captable';
import { ethers } from 'ethers';
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from '../../../contants';
import { formatPublicKeyForSolidityBytes, getStealthAddress } from '../../../utils/stealth';
import debug from 'debug';
import ApiRequestLogger from '../../../utils/apiRequestLogger';
import { ApiError } from 'next/dist/server/api-utils';
import { connectToStealthAddressFactory_RW } from '../../../utils/blockchain';
import { errorResponse } from '../../../utils/api';

const log = debug('brok:shareholder:register');
type Data = {};

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
  try {
    ApiRequestLogger(req, log);
    switch (req.method) {
      case 'GET':
        // Find all captables and shares belonging to the user
        const { wallet } = parseQuery(req.query);


      case 'POST':
        // Register new user's wallet, so stealth addresses can be created from it

        const { signature, spendPublicKey } = parseBody(req.body);

        // a digested spendPublicKey + signature should return user wallet address
        const digest = ethers.utils.arrayify(ethers.utils.hashMessage(spendPublicKey));
        const recoveredAddress = ethers.utils.recoverAddress(digest, signature);

        const isRegistered = await checkIfWalletIsRegisteredForStealth(recoveredAddress)

        if (isRegistered) {
          log('HTTP Response 200, keys already registered');
          return res.status(200).json({
            success: true,
            message: 'Keys already registered',
          });
        } 

        const receipt = await registerWalletForStealth(recoveredAddress, spendPublicKey)
        log(`HTTP Response 200, Registered keys for ${recoveredAddress} with receipt ${receipt.transactionHash}`);
        return res.status(200).json({
          success: true,
          message: `Registered keys for ${recoveredAddress} with receipt ${receipt.transactionHash}`,
        });
        
      default:
        res.setHeader('Allow', ['GET', 'POST']);
        res.status(405).end(`Method ${req.method} Not Allowed`);
    }
  } catch (error) {
    errorResponse(error, log, res);
  }
}

async function checkIfWalletIsRegisteredForStealth(wallet: string): Promise<boolean> {
  const registry = connectToStealthAddressFactory_RW();

  log(`Checking address: ${wallet} for stealth keys`);
  const currentKeys = await registry.stealthKeys(wallet, CONTRACT_ADDRESSES.SECP256K1_GENERATOR);
  log('current keys', currentKeys);
  if ('spendingPubKey' in currentKeys && currentKeys.spendingPubKey !== '0x') {
    log('wallet already registered');
    return true
  }
  log('wallet is not registered');
  return false
}

async function registerWalletForStealth(wallet: string, spendPublicKey: string) {
  const registry = connectToStealthAddressFactory_RW();

  const spendPublicKeyParsed = formatPublicKeyForSolidityBytes(spendPublicKey);
  const viewPublicKeyParsed = '0x11'; // TODO - Start using view keys

  const tx = await registry.registerKeysOnBehalf(
    wallet,
    CONTRACT_ADDRESSES.SECP256K1_GENERATOR,
    '0x11',
    spendPublicKeyParsed,
    viewPublicKeyParsed,
  );
  const receipt = await tx.wait();

  return receipt
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

function parseQuery(
  query: Partial<{
    [key: string]: string | string[];
  }>,
) {
  if (!('wallet' in query)) {
    throw new ApiError(400, 'wallet missing');
  }

  let wallet: string;
  try {
    wallet = ethers.utils.getAddress(query.wallet!.toString());
  } catch (error) {
    throw new ApiError(400, 'Invalid wallet in query');
  }

  return { wallet };
}
