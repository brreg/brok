const ethers = require('ethers');

function createWallet() {
    const wallet = ethers.Wallet.createRandom();
    console.log('Wallet Address:', wallet.address, 'Use this as DEV_ENTERPRISE_SYSTEM_ADDRESS in packages/captable/.env');
    console.log('Private Key:', wallet.privateKey, 'Use this as PRIVATE_KEY in packages/api/.env');
    console.log('Mnemonic Phrase:', wallet.mnemonic.phrase, 'Use this as SEED_DEV in packages/captable/.env');
}

createWallet();