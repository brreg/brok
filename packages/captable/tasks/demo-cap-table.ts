import debug from "debug";
import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment, TaskArguments } from "hardhat/types";
import { CapTable, CapTableRegistry__factory, CapTable__factory } from "../typechain-types";
import { TASK_PRE_DEPLOY_CHECK } from "./generate-deployments";
export const TASK_DEMO_CAP_TABLE = "DEMO";
const log = debug(`brok:task:${TASK_DEMO_CAP_TABLE}`);

task(TASK_DEMO_CAP_TABLE, "Deploy a demo cap table for testing purposes")
	.addFlag("dev", "Deploy development state.")
	.addFlag("log", "Log execution")
	.addFlag("redeploy", "Redeploy the contract instance on network if finds deployment deployment")
	.setAction(async (taskArgs: TaskArguments, hre: HardhatRuntimeEnvironment) => {
		try {
			const [deployer] = await hre.ethers.getSigners();
			log("Balance of deployer %s", hre.ethers.utils.formatEther(await deployer.getBalance()));
			if (hre.hardhatArguments.verbose || taskArgs.log) {
				log.enabled = true;
			}

			/* Get contract dependencies */
			const registryAddress = await hre.run(TASK_PRE_DEPLOY_CHECK, {
				contract: "CAP_TABLE_REGISTRY",
				redeploy: taskArgs.redeploy,
				useLocal: true,
			});
			log("Using registry at %s", registryAddress);
			if (!registryAddress) {
				throw new Error("No registry address found");
			}
			// number lengt of digits
			const randomOrgNr = Math.floor(Math.random() * 10000);
			const DEFAULT_PARTITION = hre.ethers.utils.formatBytes32String("ordinære");

			// create a board director
			let balanceDeployer = await deployer.getBalance();
			// 0xBBB12c73703A8dC9ae2569E1C7AD699a5Ac8C782
			const boardDirectorWallet = new hre.ethers.Wallet(
				"0x9abc1d0b34c15e5375ad2f195d0f54f01309b7bccfeff539771aedc25adcf39d",
			).connect(hre.ethers.provider);
			const balanceBoardDirector = await boardDirectorWallet.getBalance();
			const minimumFundingAmount = hre.ethers.utils.parseEther("0.3");
			if (balanceBoardDirector.lt(minimumFundingAmount)) {
				if (balanceDeployer.lt(minimumFundingAmount)) {
					throw new Error("Not enough funds to deploy demo cap table");
				}
				await (
					await deployer.sendTransaction({
						to: boardDirectorWallet.address,
						value: minimumFundingAmount,
					})
				).wait();
			}

			log("Board director wallet created at %s", boardDirectorWallet.address);
			log("Balance of board director wallet %s", hre.ethers.utils.formatEther(await boardDirectorWallet.getBalance()));

			const registry = await CapTableRegistry__factory.connect(registryAddress, deployer);
			let capTable: CapTable | undefined;

			try {
				capTable = await new CapTable__factory(boardDirectorWallet).deploy(
					`CapTable${randomOrgNr}`,
					randomOrgNr.toString(),
					hre.ethers.utils.parseEther("1"),
					[boardDirectorWallet.address, deployer.address],
					[DEFAULT_PARTITION],
				);
			} catch (error) {
				log("Error while deploying captable");
				if (error && error instanceof Error && "reason" in error) {
					log("Reason: %s", error.reason);
				}
			}
			if (!capTable) {
				throw new Error("Cap table not deployed");
			}
			log("Deploying demo cap table...");
			await capTable.deployed();
			log("Demo cap table deployed at %s", capTable.address);

			// Add capTable to registry
			log("Adding cap table to registry...");
			await (await registry.addCapTable(capTable.address, randomOrgNr.toString())).wait();

			log("status for capTable in registry", await registry.getStatus(capTable.address));
			// Comfirm added
			log("Cap table added to registry, executing cap table confirm");
			await (await capTable.confirmAddedToRegistry(registry.address)).wait();

			const isAdded = await capTable.isAddedToRegistry();
			if (!isAdded) {
				throw new Error("Cap table not added to registry");
			}
			// create shareholder 0xcc6aa2c0D12716916e19012E954a0630fA25e097
			const shareholderWallet = new hre.ethers.Wallet(
				"0xc4a527baf0eaf2270d7acd104d3a4ac15606e1550f344910af55dc30ea4703bc",
			).connect(hre.ethers.provider);
			const balanceShareholder = await shareholderWallet.getBalance();
			balanceDeployer = await deployer.getBalance();
			if (balanceShareholder.lt(minimumFundingAmount)) {
				if (balanceDeployer.lt(minimumFundingAmount)) {
					throw new Error("Not enough funds to deploy demo cap table");
				}
				await (
					await deployer.sendTransaction({
						to: shareholderWallet.address,
						value: minimumFundingAmount,
					})
				).wait();
			}
			log("Shareholder wallet created at %s", shareholderWallet.address);
			log("Balance of shareholder wallet %s", await shareholderWallet.getBalance());

			// Issue shares to board director
			const capTableAsBoardDirector = await CapTable__factory.connect(capTable.address, boardDirectorWallet);
			const capTableAsShareholder = await CapTable__factory.connect(capTable.address, shareholderWallet);

			log("Issuing shares to board director...");
			await (
				await capTableAsBoardDirector.issue(boardDirectorWallet.address, hre.ethers.utils.parseEther("1000"), "0x11")
			).wait();
			log(`Board director has ${await capTableAsBoardDirector.balanceOf(boardDirectorWallet.address)} shares`);

			// issue shares to shareholder
			log("Issuing shares to shareholder...");
			await (
				await capTableAsBoardDirector.issue(shareholderWallet.address, hre.ethers.utils.parseEther("1000"), "0x11")
			).wait();
			log(`Shareholder has ${await capTableAsShareholder.balanceOf(shareholderWallet.address)} shares`);
		} catch (error) {
			console.error(error);
			throw error;
		}
	});
