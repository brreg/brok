import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers } from "hardhat";
import { CapTableRegistry } from "../typechain-types";

describe("CapTableRegistry", function () {
	let owner: SignerWithAddress;
	let operator1: SignerWithAddress;
	let operator2: SignerWithAddress;
	let randomAddress: SignerWithAddress;
	let capTableRegistry: any;

	beforeEach(async () => {
		const accounts = await ethers.getSigners();
		owner = accounts[0];
		operator1 = accounts[1];
		operator2 = accounts[2];
		randomAddress = accounts[3];

		const CapTableRegistryFactory = await ethers.getContractFactory("CapTableRegistry");
		capTableRegistry = (await CapTableRegistryFactory.deploy()) as CapTableRegistry;
		await capTableRegistry.deployed();

		// Granting operator1 and operator2 the OPERATOR_ROLE
		await capTableRegistry.grantRole(
			ethers.utils.keccak256(ethers.utils.toUtf8Bytes("OPERATOR_ROLE")),
			operator1.address,
		);
		// await a.wait();
		await capTableRegistry.grantRole(
			ethers.utils.keccak256(ethers.utils.toUtf8Bytes("OPERATOR_ROLE")),
			operator2.address,
		);
	});

	describe("addCapTable", function () {
		it("should add a cap table by an operator", async function () {
			const orgID = "12345";
			await expect(capTableRegistry.connect(operator1).addCapTable(randomAddress.address, orgID))
				.to.emit(capTableRegistry, "CapTableAdded")
				.withArgs(randomAddress.address, orgID);

			expect(await capTableRegistry.getId(randomAddress.address)).to.equal(orgID);
			expect(await capTableRegistry.getAddress(orgID)).to.equal(randomAddress.address);
		});

		it("should not add a cap table by a non-operator", async function () {
			const orgID = "12345";
			await expect(capTableRegistry.connect(randomAddress).addCapTable(randomAddress.address, orgID)).to.be.reverted;
		});

		it("should not add a cap table with an already used id", async function () {
			const orgID = "12345";
			await capTableRegistry.connect(operator1).addCapTable(randomAddress.address, orgID);
			await expect(capTableRegistry.connect(operator2).addCapTable(operator2.address, orgID)).to.be.revertedWith(
				"id is already in use",
			);
		});
	});

	describe("removeCapTable", function () {
		it("should allow an operator to remove a cap table", async function () {
			const orgID = "67890";
			await capTableRegistry.connect(operator1).addCapTable(randomAddress.address, orgID);

			await expect(capTableRegistry.connect(operator1).removeCapTable(randomAddress.address))
				.to.emit(capTableRegistry, "CapTableRemoved")
				.withArgs(randomAddress.address, orgID);

			expect(await capTableRegistry.getId(randomAddress.address)).to.equal("");
			expect(await capTableRegistry.getAddress(orgID)).to.equal(ethers.constants.AddressZero);
		});

		it("should not allow a non-operator to remove a cap table", async function () {
			const orgID = "67890";
			await capTableRegistry.connect(operator1).addCapTable(randomAddress.address, orgID);

			await expect(capTableRegistry.connect(randomAddress).removeCapTable(randomAddress.address)).to.be.reverted;
		});
	});
});
