/**
 * This file defines the "deploy-all" task using Hardhat, a development environment for Ethereum.
 *
 * The task performs several operations related to the deployment of contracts, including:
 *   - Retrieving and logging the deployer's address and balance.
 *   - Running tasks for deploying the cap table registry.
 *   - Generating deployments and NPM packages.
 *
 * Flags:
 *   - redeploy: Forces redeployment of contracts, setting new contract instances.
 *   - dev: Specifies whether to deploy in a development state.
 *   - log: Enables logging of execution.
 *
 * The script logs information about the network, deployer, and balance, and executes related tasks.
 * Errors are caught and logged for further troubleshooting.
 *
 * Usage:
 *   - Run the task using Hardhat's command line interface or programmatically within your Hardhat project.
 */

import debug from "debug";
import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment, TaskArguments } from "hardhat/types";
import { TASK_DEPLOY_CAP_TABLE_REGISTRY } from "./deploy-cap-table-registry";
import { TASK_GENERATE_DEPLOYMENTS, TASK_GENERATE_NPM_PACKAGE } from "./generate-deployments";

const log = debug("brok:task:deploy-all");

task("deploy-all", "Create a deployments folder")
	.addFlag("redeploy", "Force redeploy contracts, will set new contract instances.")
	.addFlag("dev", "Deploy development state.")
	.addFlag("log", "Log execution")
	.setAction(async (taskArgs: TaskArguments, hre: HardhatRuntimeEnvironment) => {
		try {
			const [deployer] = await hre.ethers.getSigners();

			if (hre.hardhatArguments.verbose || taskArgs.log) {
				log.enabled = true;
			}

			log("deploy-all network:", hre.network.name);
			log("Deploying from account:", deployer.address);
			log("Balance of deployer:", hre.ethers.utils.formatEther(await deployer.getBalance()));

			await hre.run(TASK_DEPLOY_CAP_TABLE_REGISTRY, {
				dev: taskArgs.dev,
				log: hre.hardhatArguments.verbose || taskArgs.log,
				redeploy: taskArgs.redeploy,
			});

			await hre.run(TASK_GENERATE_DEPLOYMENTS, {
				log: hre.hardhatArguments.verbose || taskArgs.log,
			});

			await hre.run(TASK_GENERATE_NPM_PACKAGE, {
				log: false,
			});

			log("deploy-all network:", hre.network.name, " done");
		} catch (error) {
			console.error(error);
			throw error;
		}
	});
