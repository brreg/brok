import debug from "debug";
import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment, TaskArguments } from "hardhat/types";
import { TASK_PRE_DEPLOY_CHECK } from "./generate-deployments";
import { ethers } from "ethers";
import { CapTableRegistry__factory, CapTable__factory } from "../typechain-types";
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
			const RyddigBobilNanv = "Ryddig Bobil AS";
			const RyddigBobilOrgnr = "815493000";
			const DEFAULT_PARTITION = hre.ethers.utils.formatBytes32String("ordinære");

			const registry = await CapTableRegistry__factory.connect(registryAddress, deployer);
			// let capTable: CapTable | undefined;

			const capTable = await new CapTable__factory(deployer).deploy(
				RyddigBobilNanv,
				RyddigBobilOrgnr,
				1,
				[],
				[DEFAULT_PARTITION],
			);

			if (!capTable) {
				throw new Error("Cap table not deployed");
			}

			// Add capTable to registry
			log("Adding cap table to registry...");
			await (await registry.addCapTable(capTable.address, RyddigBobilOrgnr.toString())).wait();

			log("status for capTable in registry", await registry.getStatus(capTable.address));
			// Comfirm added
			log("Cap table added to registry, executing cap table confirm");
			await (await capTable.confirmAddedToRegistry(registry.address)).wait();

			const isAdded = await capTable.isAddedToRegistry();
			if (!isAdded) {
				throw new Error("Cap table not added to registry");
			}

			const aksjeklasseArray = new Array(6).fill(DEFAULT_PARTITION);

			const shareholder1 = "0xBBB12c73703A8dC9ae2569E1C7AD699a5Ac8C782";
			const shareholder2 = "0xcc6aa2c0D12716916e19012E954a0630fA25e097";
			const shareholder3 = "0x8be848CE9eBBA1e304E6dAA1D6b1B40f17e478fD";
			const shareholder4 = "0xF04EB77C73c11D4b9eC610Cf8Ce6B51B7F78929B";
			const shareholder5 = "0xeE879E18569a12687489a8CC48B53292Ea2907c6";
			const shareholder6 = "0xc15451645ba50375580F673647C3Ac34aAD22e62";

			await capTable.kapitalforhoyselse_nye_aksjer(
				aksjeklasseArray,
				[shareholder1, shareholder2, shareholder3, shareholder4, shareholder5, shareholder6],
				[5000, 3000, 200, 50, 999, 1000],
				"0x11",
			);

			const balanseShareholder1 = await capTable.balanceOfByPartition(DEFAULT_PARTITION, shareholder1);
			const balanseShareholder2 = await capTable.balanceOfByPartition(DEFAULT_PARTITION, shareholder2);
			const balanseShareholder3 = await capTable.balanceOfByPartition(DEFAULT_PARTITION, shareholder3);
			const balanseShareholder4 = await capTable.balanceOfByPartition(DEFAULT_PARTITION, shareholder4);
			const balanseShareholder5 = await capTable.balanceOfByPartition(DEFAULT_PARTITION, shareholder5);
			const balanseShareholder6 = await capTable.balanceOfByPartition(DEFAULT_PARTITION, shareholder6);

			log("Balance of shareholder1 wallet %s", balanseShareholder1);
			log("Balance of shareholder2 wallet %s", balanseShareholder2);
			log("Balance of shareholder3 wallet %s", balanseShareholder3);
			log("Balance of shareholder4 wallet %s", balanseShareholder4);
			log("Balance of shareholder5 wallet %s", balanseShareholder5);
			log("Balance of shareholder6 wallet %s", balanseShareholder6);
		} catch (error) {
			console.error(error);
			throw error;
		}
	});
