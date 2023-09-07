import debug from 'debug';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';
import { TASK_PRE_DEPLOY_CHECK } from './generate-deployments';
import { CapTable__factory, CapTableRegistry__factory } from '../dist';
export const TASK_DEMO_CAP_TABLE = 'DEMO';
const log = debug(`brok:task:${TASK_DEMO_CAP_TABLE}`);

task(TASK_DEMO_CAP_TABLE, 'Deploy a demo cap table for testing purposes')
  .addFlag('dev', 'Deploy development state.')
  .addFlag('log', 'Log execution')
  .addFlag('redeploy', 'Redeploy the contract instance on network if finds deployment deployment')
  .setAction(async (taskArgs: TaskArguments, hre: HardhatRuntimeEnvironment) => {
    try {
      const [deployer] = await hre.ethers.getSigners();
      log('Balance of deployer %s', hre.ethers.utils.formatEther(await deployer.getBalance()));
      if (hre.hardhatArguments.verbose || taskArgs.log) {
        log.enabled = true;
      }

      /* Get contract dependencies */
      const registryAddress = await hre.run(TASK_PRE_DEPLOY_CHECK, {
        contract: 'CAP_TABLE_REGISTRY',
        redeploy: taskArgs.redeploy,
        useLocal: true,
      });
      log('Using registry at %s', registryAddress);
      if (!registryAddress) {
        throw new Error('No registry address found');
      }
      // number lengt of digits
      const RyddigBobilNanv = "Ryddig Bobil AS";
      const RyddigBobilOrgnr = "815493000";     
      const DEFAULT_PARTITION = hre.ethers.utils.formatBytes32String('ordinære');

      // create a board director
      let balanceDeployer = await deployer.getBalance();
      // 0xBBB12c73703A8dC9ae2569E1C7AD699a5Ac8C782
      const boardDirectorWallet = new hre.ethers.Wallet('0x9abc1d0b34c15e5375ad2f195d0f54f01309b7bccfeff539771aedc25adcf39d').connect(hre.ethers.provider);
      const balanceBoardDirector = await boardDirectorWallet.getBalance();
      const minimumFundingAmount = hre.ethers.utils.parseEther('0.40');
      if (balanceBoardDirector.lt(minimumFundingAmount)) {
        if (balanceDeployer.lt(minimumFundingAmount)) {
          throw new Error('Not enough funds to deploy demo cap table');
        }
        await (
          await deployer.sendTransaction({
            to: boardDirectorWallet.address,
            value: minimumFundingAmount,
          })
        ).wait();
      }

      log('Board director wallet created at %s', boardDirectorWallet.address);
      log('Balance of board director wallet %s', hre.ethers.utils.formatEther(await boardDirectorWallet.getBalance()));

      const registry = await CapTableRegistry__factory.connect(registryAddress, deployer);
      // let capTable: CapTable | undefined;

      const capTable = await new CapTable__factory(boardDirectorWallet).deploy(
        RyddigBobilNanv,
        RyddigBobilOrgnr,
        hre.ethers.utils.parseEther('1'),
        [boardDirectorWallet.address, deployer.address],
        [DEFAULT_PARTITION],
      );

      if (!capTable) {
        throw new Error('Cap table not deployed');
      }

      // Add capTable to registry
      log('Adding cap table to registry...');
      await (await registry.addCapTable(capTable.address, RyddigBobilOrgnr.toString())).wait();

      log('status for capTable in registry', await registry.getStatus(capTable.address));
      // Comfirm added
      log('Cap table added to registry, executing cap table confirm');
      await (await capTable.confirmAddedToRegistry(registry.address)).wait();

      const isAdded = await capTable.isAddedToRegistry();
      if (!isAdded) {
        throw new Error('Cap table not added to registry');
      }

      // wallet address 0xcc6aa2c0D12716916e19012E954a0630fA25e097
      const shareholderWallet1 = new hre.ethers.Wallet('0xc4a527baf0eaf2270d7acd104d3a4ac15606e1550f344910af55dc30ea4703bc').connect(hre.ethers.provider);

      // wallet address 0x8be848CE9eBBA1e304E6dAA1D6b1B40f17e478fD
      const shareholderWallet2 = new hre.ethers.Wallet('0xda8e417a4c1b769e0022539ddd00c9697e89b414eb2320e06cc869d4fc5dabf8').connect(hre.ethers.provider);

      // wallet address 0xF04EB77C73c11D4b9eC610Cf8Ce6B51B7F78929B
      const shareholderWallet3 = new hre.ethers.Wallet('0xc3c3bdfb09d08eeaafc6d4bf6e54775805c4ea536f008312e221ce2d9928df2d').connect(hre.ethers.provider);

      // wallet address 0xeE879E18569a12687489a8CC48B53292Ea2907c6
      const shareholderWallet4 = new hre.ethers.Wallet('0xb3e1e7c20d7b6c3c75f4655a2dbb00c78ef619f36e7ca52dd71cc2fe3ac8e393').connect(hre.ethers.provider);

      // wallet address 0xc15451645ba50375580F673647C3Ac34aAD22e62
      const shareholderWallet5 = new hre.ethers.Wallet('0xf01e09e49991aba9a8940f60bfc4a80593355a2913c98fb6c722c86e17edf8e9').connect(hre.ethers.provider);

      log('Shareholder wallet created at %s', shareholderWallet1.address);
      log('Balance of shareholder wallet %s', await shareholderWallet1.getBalance());

      log('Shareholder wallet created at %s', shareholderWallet2.address);
      log('Balance of shareholder wallet %s', await shareholderWallet2.getBalance());

      log('Shareholder wallet created at %s', shareholderWallet3.address);
      log('Balance of shareholder wallet %s', await shareholderWallet3.getBalance());

      log('Shareholder wallet created at %s', shareholderWallet4.address);
      log('Balance of shareholder wallet %s', await shareholderWallet4.getBalance());

      log('Shareholder wallet created at %s', shareholderWallet5.address);
      log('Balance of shareholder wallet %s', await shareholderWallet5.getBalance());

      // Issue shares to board director
      const capTableAsBoardDirector = await CapTable__factory.connect(capTable.address, boardDirectorWallet);
      const capTableAsShareholder1 = await CapTable__factory.connect(capTable.address, shareholderWallet1);
      const capTableAsShareholder2 = await CapTable__factory.connect(capTable.address, shareholderWallet2);
      const capTableAsShareholder3 = await CapTable__factory.connect(capTable.address, shareholderWallet3);
      const capTableAsShareholder4 = await CapTable__factory.connect(capTable.address, shareholderWallet4);
      const capTableAsShareholder5 = await CapTable__factory.connect(capTable.address, shareholderWallet5);

      log('Issuing shares to board director...');
      await (await capTableAsBoardDirector.issue(boardDirectorWallet.address, hre.ethers.utils.parseEther('5000'), '0x11')).wait();
      log(`Board director has ${await capTableAsBoardDirector.balanceOf(boardDirectorWallet.address)} shares`);

      // issue shares to shareholder
      log('Issuing shares to shareholder...');
      await (await capTableAsBoardDirector.issue(shareholderWallet1.address, hre.ethers.utils.parseEther('3000'), '0x11')).wait();
      log(`Shareholder1 has ${await capTableAsShareholder1.balanceOf(shareholderWallet1.address)} shares`);

      await (await capTableAsBoardDirector.issue(shareholderWallet2.address, hre.ethers.utils.parseEther('200'), '0x11')).wait();
      log(`Shareholder2 has ${await capTableAsShareholder2.balanceOf(shareholderWallet2.address)} shares`);

      await (await capTableAsBoardDirector.issue(shareholderWallet3.address, hre.ethers.utils.parseEther('50'), '0x11')).wait();
      log(`Shareholder3 has ${await capTableAsShareholder3.balanceOf(shareholderWallet3.address)} shares`);

      await (await capTableAsBoardDirector.issue(shareholderWallet4.address, hre.ethers.utils.parseEther('999'), '0x11')).wait();
      log(`Shareholder4 has ${await capTableAsShareholder4.balanceOf(shareholderWallet4.address)} shares`);

      await (await capTableAsBoardDirector.issue(shareholderWallet5.address, hre.ethers.utils.parseEther('1000'), '0x11')).wait();
      log(`Shareholder5 has ${await capTableAsShareholder5.balanceOf(shareholderWallet5.address)} shares`);
    } catch (error) {
      console.error(error);
      throw error;
    }
  });
