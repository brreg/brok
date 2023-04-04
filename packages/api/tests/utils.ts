import { faker } from '@faker-js/faker/locale/nb_NO';
import { ethers } from 'ethers';
import { CapTable, CapTableRegistry__factory, CapTable__factory, ERC5564Messenger__factory, ERC5564Registry__factory } from '@brok/captable';
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from '../src/contants';
import { formatPublicKeyForSolidityBytes, getSharedSecret, getStealthAddress, signatureToStealthKeys } from '../src/utils/stealth';
import { ConnectToStealthAddressFactory_RW } from '../src/utils/blockchain';


/**
 * Create a random orgnr
 * 
 * @returns number between 111 111 111 and 999 999 999
 */
export function GenerateOrgnr(): number {
  const min = Math.ceil(111111111);
  const max = Math.floor(999999999);
  const orgnr = Math.floor(Math.random() * (max - min + 1) + min);
  return orgnr
}

export async function CreateNewCapTable(): Promise<string> {

  const orgnr = GenerateOrgnr().toString()
  const wallet = WALLET.connect(GET_PROVIDER());
  const deployTx = await new CapTable__factory().getDeployTransaction(
    faker.company.name(),
    orgnr,
    ethers.utils.parseEther('1'),
    CONTROLLERS,
    [DEFAULT_PARTITION],
    CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY,
  );
  const signedTx = await wallet.sendTransaction(deployTx);
  const capTableAddress = ethers.utils.getContractAddress(signedTx);
  await new CapTableRegistry__factory(wallet).attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY).addCapTable(capTableAddress, orgnr.toString());
  
  return capTableAddress
}

export async function IssueShares(capTableAddress: string, userWallet?: ethers.Wallet) {
  
  const wallet = WALLET.connect(GET_PROVIDER());
  const capTable = await new CapTable__factory(wallet).attach(capTableAddress)
  const messenger = await new ERC5564Messenger__factory(wallet).attach(CONTRACT_ADDRESSES.ERC5564_MESSENGER)
  
  const randomEthereumWallet = ethers.Wallet.createRandom()
  const registry = new ERC5564Registry__factory(wallet).attach(CONTRACT_ADDRESSES.ERC5564_REGISTRY);
  
  let stealthKeys

  // stealth config
  if (!userWallet) {
    userWallet = ethers.Wallet.createRandom();
  }
  stealthKeys = await registry.stealthKeys(userWallet.address, CONTRACT_ADDRESSES.SECP256K1_GENERATOR )
  const sharedSecret = getSharedSecret(randomEthereumWallet.privateKey.slice(2), `04${stealthKeys?.spendingPubKey.slice(2)}`)
  const stealthAddress = getStealthAddress(`04${stealthKeys?.spendingPubKey.slice(2)}`, sharedSecret)

  // add shares to stealth address
  const result = await capTable.issue(userWallet.address, ethers.utils.parseEther("1000"), ethers.constants.HashZero);
  const announcement = await messenger.announce(
    `0x${randomEthereumWallet.publicKey.slice(4)}`,
    ethers.utils.hexZeroPad(stealthAddress, 32),
    ethers.constants.HashZero,
  );
}