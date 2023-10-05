import { faker } from "@faker-js/faker";
import { ethers } from "ethers";
import {
	CapTableRegistry__factory,
	CapTable__factory,
	ERC5564Messenger__factory,
	ERC5564Registry__factory,
} from "@brok/captable";
import { CONTRACT_ADDRESSES, CONTROLLERS, DEFAULT_PARTITION, GET_PROVIDER, WALLET } from "../src/contants";
import { ConnectToCapTableRegistry_R, ConnectToCapTable_R } from "../src/utils/blockchain";
import { APIRequestContext } from "@playwright/test";

/**
 * Create a random orgnr
 *
 * @returns number between 111 111 111 and 999 999 999
 */
export function GenerateRandomOrgnr(): number {
	const min = Math.ceil(111111111);
	const max = Math.floor(999999999);
	const orgnr = Math.floor(Math.random() * (max - min + 1) + min);
	return orgnr;
}

export function GenerateRandomCompanyName() {
	const options = [
		// () => {
		//   return faker.hacker.adjective() + " " + faker.hacker.noun() + " AS"
		// },
		// () => {
		//   return faker.company.catchPhraseAdjective() + " " + faker.hacker.noun() + " AS"
		// },
		// () => {
		//   return faker.company.catchPhraseAdjective() + " " + faker.address.city() + " AS"
		// },
		() => {
			return faker.company.catchPhraseAdjective() + " " + faker.animal.type() + " AS";
		},
	];
	const random = Math.floor(Math.random() * options.length);
	console.log(random);

	return options[random]();
}

export async function CreateNewCapTable(): Promise<{ capTableAddress: string; orgnr: string }> {
	const orgnr = GenerateRandomOrgnr().toString();
	const wallet = WALLET.connect(GET_PROVIDER());
	const deployTx = await new CapTable__factory().getDeployTransaction(
		GenerateRandomCompanyName(),
		orgnr,
		ethers.utils.parseEther("1"),
		CONTROLLERS,
		[DEFAULT_PARTITION],
	);
	const signedTx = await wallet.sendTransaction(deployTx);
	const capTableAddress = ethers.utils.getContractAddress(signedTx);
	await (
		await new CapTableRegistry__factory(wallet)
			.attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY)
			.addCapTable(capTableAddress, orgnr.toString())
	).wait();
	await new CapTable__factory(wallet)
		.attach(capTableAddress)
		.confirmAddedToRegistry(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY);

	return { capTableAddress, orgnr };
}

// TODO Her er jo tanken god, men getAddres som den bruker er jo noe helt annet: function getAddress(string calldata id) external view returns (address capTableAddress)
// Den tar en ID inn og RETUNERER en adresse. Så det er jo ikke det vi vil ha. Vi vil ha en funksjon som tar en adresse inn og returnerer et aksjeeierbok-objekt — Er det mulig?
export async function FindCapTableWithAddress(ethAddress: string) {
	// Find all companies
	const capTableRegistry = await ConnectToCapTableRegistry_R();
	const captable = await capTableRegistry.getAddress(ethAddress);
	return captable;
}

export async function FindCapTableWithOrgnr(orgnr: string) {
	const capTableRegistry = await ConnectToCapTableRegistry_R();
	const allCapTablesAddresses = await capTableRegistry.getCapTableList();
	const promise = allCapTablesAddresses.map(async (capTableAddress: string) => {
		const captable = await ConnectToCapTable_R(capTableAddress);
		const nr = await captable.getOrgnr();
		if (orgnr === nr) {
			return {
				orgnr: nr,
				name: await captable.name(),
				ethAddress: capTableAddress,
			};
		}
	});
	Promise.all(promise);
}

export function sjekkMottakere(request: APIRequestContext, baseURL: (string | undefined), orgnr: string, identifiers: string[]) {
	return request.post(`${baseURL}/api/v1/company/${orgnr}/sjekk-mottakere`, {
		headers: {
			"Content-Type": "application/json",
		},
		data: JSON.stringify({
			mottkerIDer: identifiers,
		}),
	});
}