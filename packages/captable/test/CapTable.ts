import { takeSnapshot } from "@nomicfoundation/hardhat-network-helpers";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers } from "hardhat";
import { CapTable, CapTableRegistry } from "../typechain-types/index";

let snapshot: any;

let operator1: SignerWithAddress;
let user1: SignerWithAddress;
let user2: SignerWithAddress;
let boardDirectorWallet: SignerWithAddress;
let companyWallet: SignerWithAddress;

let approvedCapTable: CapTable;
let pendingCapTable: CapTable;
let capTableRegistry: CapTableRegistry;

const DEFAULT_PARTITION = ethers.utils.formatBytes32String("ordinære");

before(async () => {
	const allAccounts = await ethers.getSigners();
	operator1 = allAccounts[0];
	user1 = allAccounts[1];
	user2 = allAccounts[2];
	boardDirectorWallet = allAccounts[3];
	companyWallet = allAccounts[4];

	const CapTableRegistryFactory = await ethers.getContractFactory("CapTableRegistry");
	capTableRegistry = (await CapTableRegistryFactory.deploy()) as CapTableRegistry;
	await capTableRegistry.deployed();

	const CapTableFactory = await ethers.getContractFactory("CapTable");
	approvedCapTable = (await CapTableFactory.deploy(
		"ApprovedCapTable AS",
		"12345678",
		1,
		[boardDirectorWallet.address, operator1.address],
		[DEFAULT_PARTITION],
	)) as CapTable;
	await approvedCapTable.deployed();

	pendingCapTable = (await CapTableFactory.deploy(
		"NotApprovedCapTable AS",
		"234234234",
		ethers.utils.parseEther("1"), // TODO Gjør lik den over
		[],
		[DEFAULT_PARTITION],
	)) as CapTable;
	await pendingCapTable.deployed();

	// Now, let's add the capTable to the registry.
	await capTableRegistry.addCapTable(approvedCapTable.address, "12345678");

	snapshot = await takeSnapshot();
});

afterEach(async () => {
	await snapshot.restore();
});

describe("MyCapTable", function () {
	it("Should emit NewCapTable on creation", async function () {
		expect(await approvedCapTable.deployed())
			.to.emit(approvedCapTable, "NewCapTable")
			.withArgs("12345678", operator1.address);
	});

	it("Should not allow operations if not added to registry", async function () {
		await expect(
			pendingCapTable.kapitalforhoyselse_nye_aksjer([DEFAULT_PARTITION], [user1.address], [1000], []),
		).to.be.revertedWith("CapTable is not added to registry");
	});

	it("should issue and TRANSFER shares", async function () {
		await approvedCapTable.confirmAddedToRegistry(capTableRegistry.address);

		// Define partitions, recipients and values.
		const partitions = [DEFAULT_PARTITION, DEFAULT_PARTITION, DEFAULT_PARTITION];
		const recipients = [boardDirectorWallet.address, user1.address, user2.address];
		const values = [30000, 11539, 4615];

		// Issue new shares.
		await approvedCapTable.kapitalforhoyselse_nye_aksjer(partitions, recipients, values, "0x11");

		// Send shares to boardDirectorWallet
		await approvedCapTable.operatorTransferByPartition(
			DEFAULT_PARTITION,
			user1.address,
			companyWallet.address,
			11539,
			"0x11",
			"0x11",
		);
		await approvedCapTable.operatorTransferByPartition(
			DEFAULT_PARTITION,
			user2.address,
			companyWallet.address,
			4615,
			"0x11",
			"0x11",
		);

		// Reduce shares.
		// Define partitions, recipients and values.
		const reducePartitions = [DEFAULT_PARTITION];
		const reduceRecipients = [companyWallet.address];
		const reduceValues = [11539 + 4615];
		await approvedCapTable.kapitalnedsettelse_reduksjon_aksjer(
			reducePartitions,
			reduceRecipients,
			reduceValues,
			"0x11",
			"0x11",
		);

		// Verify reduced balance.
		const remainingSupply = await approvedCapTable.totalSupplyByPartition(DEFAULT_PARTITION);
		expect(remainingSupply.toString()).to.equal("30000", "Shares should have been reduced to 30000.");
	});

	it("should issue and REDUCE shares", async function () {
		await approvedCapTable.confirmAddedToRegistry(capTableRegistry.address);

		// Define partitions, recipients and values.
		const partitions = [DEFAULT_PARTITION];
		const recipients = [user1.address];
		const values = [1000];

		// Issue new shares.
		await approvedCapTable.kapitalforhoyselse_nye_aksjer(partitions, recipients, values, "0x11");

		// Verify that the recipient received the tokens.
		const balance = await approvedCapTable.balanceOfByPartition(partitions[0], user1.address);
		expect(balance.toString()).to.equal(values[0].toString(), "User should have received the new shares.");

		// Reduce shares.
		const reduceValues = [700];
		await approvedCapTable.kapitalnedsettelse_reduksjon_aksjer(partitions, recipients, reduceValues, "0x11", "0x11");

		// Verify reduced balance.
		const remainingBalance = await approvedCapTable.balanceOfByPartition(partitions[0], user1.address);
		expect(remainingBalance.toString()).to.equal("300", "Shares should have been reduced for the user.");
	});

	it("should split shares for existing shareholders", async function () {
		await approvedCapTable.confirmAddedToRegistry(capTableRegistry.address);

		// Issue initial shares to a user.
		const initialPartitions = [DEFAULT_PARTITION];
		const initialRecipients = [user1.address];
		const initialValues = [1000];
		await approvedCapTable.kapitalforhoyselse_nye_aksjer(initialPartitions, initialRecipients, initialValues, "0x11");

		// Split shares.
		const splitValues = [500];
		await approvedCapTable.splitt(initialPartitions, initialRecipients, splitValues, "0x11");

		// Verify balance after split.
		const newBalance = await approvedCapTable.balanceOfByPartition(initialPartitions[0], user1.address);
		expect(newBalance.toString()).to.equal("1500", "Shares should have been split for the user.");
	});

	it("should merge shares for existing shareholders", async function () {
		await approvedCapTable.confirmAddedToRegistry(capTableRegistry.address);

		// Issue initial shares to a user.
		const initialPartitions = [DEFAULT_PARTITION];
		const initialRecipients = [user1.address];
		const initialValues = [1500];
		await approvedCapTable.kapitalforhoyselse_nye_aksjer(initialPartitions, initialRecipients, initialValues, "0x11");

		// Merge shares.
		const mergeValues = [500];
		await approvedCapTable.spleis(initialPartitions, initialRecipients, mergeValues, "0x11", "0x11");

		// Verify balance after merge.
		const newBalance = await approvedCapTable.balanceOfByPartition(initialPartitions[0], user1.address);
		expect(newBalance.toString()).to.equal("1000", "Shares should have been merged for the user.");
	});

	// TODO Tester for at man forsøker å gjøre ting som ikke er mulig, f.eks. å splitte aksjer som ikke finnes, eller å ikke ha riktige rettigheter.
});
