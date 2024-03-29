import debug from "debug";
import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment, TaskArguments } from "hardhat/types";
import { CapTableRegistry__factory } from "../typechain-types";
import { TASK_PRE_DEPLOY_CHECK } from "./generate-deployments";
export const TASK_DEPLOY_CAP_TABLE_REGISTRY = "deploy-cap-table-registry";
const log = debug(`brok:task:${TASK_DEPLOY_CAP_TABLE_REGISTRY}`);

task(TASK_DEPLOY_CAP_TABLE_REGISTRY, "Deploy contract")
	.addFlag("dev", "Deploy development state.")
	.addFlag("log", "Log execution")
	.addFlag("redeploy", "Redeploy the contract instance on network if finds deployment deployment")
	.setAction(async (taskArgs: TaskArguments, hre: HardhatRuntimeEnvironment) => {
		try {
			const [deployer] = await hre.ethers.getSigners();
			if (hre.hardhatArguments.verbose || taskArgs.log) {
				log.enabled = true;
			}

			/* Get contract dependencies */
			const contractAddress = await hre.run(TASK_PRE_DEPLOY_CHECK, {
				contract: "CAP_TABLE_REGISTRY",
				redeploy: taskArgs.redeploy,
			});

			const contract = await (async () => {
				log(`CAP_TABLE_REGISTRY address: ${contractAddress}, will ${contractAddress ? "attach" : "deploy"}}`);
				if (!contractAddress) {
					const capTableRegistry = await new CapTableRegistry__factory(deployer).deploy();
					await capTableRegistry.deployed();
					hre.deployed.CAP_TABLE_REGISTRY = capTableRegistry.address;
					return capTableRegistry;
				} else {
					const capTableRegistry = await new CapTableRegistry__factory(deployer).attach(contractAddress);
					return capTableRegistry;
				}
			})();
			log("CAP_TABLE_REGISTRY=", contract.address);
			if (!process.env.DEV_ENTERPRISE_SYSTEM_ADDRESS) {
				throw Error("Must set process.env.DEV_ENTERPRISE_SYSTEM_ADDRESS");
			}
			await (
				await contract.grantRole(hre.ethers.utils.id("OPERATOR_ROLE"), process.env.DEV_ENTERPRISE_SYSTEM_ADDRESS)
			).wait();
			const balanceDeployer = await deployer.getBalance();
			await (
				await deployer.sendTransaction({
					to: process.env.DEV_ENTERPRISE_SYSTEM_ADDRESS,
					value: balanceDeployer.div(hre.ethers.BigNumber.from(500)),
				})
			).wait();
		} catch (error) {
			console.error(error);
			throw error;
		}
	});
