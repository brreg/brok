const ethers = require('ethers');

function createWallet() {
  const wallet = ethers.Wallet.createRandom();
  console.log('Wallet Address:', wallet.address);
  console.log('Private Key:', wallet.privateKey);
}

createWallet();
