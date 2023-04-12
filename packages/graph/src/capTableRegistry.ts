import { Bytes, BigInt, DataSourceContext, log } from "@graphprotocol/graph-ts";
import { CapTableAdded, CapTableRemoved } from "../generated/CapTableRegistry/CapTableRegistry";
import { CapTableRegistry as CapTableRegistrySchema, CapTable as CapTableSchema } from "../generated/schema";
import { CapTable as CapTableTemplate } from "../generated/templates";
import { CapTable as CapTableTemplateDetail } from "../generated/templates/CapTable/CapTable";

export function handleCapTableAdded(event: CapTableAdded): void {
	log.info("######## START handleCapTableAdded", []);
	// Entities can be loaded from the store using a string ID; this ID
	// needs to be unique across all entities of the same type
	let capTableRegistry = CapTableRegistrySchema.load(event.address.toHex());
	log.info("######## handleCapTableAdded: capTableRegistry {}", [event.address.toHex()]);
	// Entities only exist after they have been saved to the store;
	// `null` checks allow to create entities on demand
	if (capTableRegistry == null) {
		capTableRegistry = new CapTableRegistrySchema(event.address.toHex());

		// Entity fields can be set using simple assignments
		capTableRegistry.count = BigInt.fromI32(0);
		capTableRegistry.address = event.address;
	}

	// let capTableQueContract = CapTableRegistry.bind(event.address);
	// let uuid = capTableQueContract.getUuid(event.address);

	// Start indexing the new capTable
	let context = new DataSourceContext();
	context.setString("capTableRegistryId", capTableRegistry.id);
	log.info("######## handleCapTableAdded: capTableRegistryId {}", [capTableRegistry.id.toString()]);
	context.setString("capTableId", event.params.id.toString());
	log.info("######## handleCapTableAdded: capTableId {}", [event.params.id.toString()]);
	CapTableTemplate.createWithContext(event.params.capTableAddress, context);

	let capTable = CapTableSchema.load(event.params.capTableAddress.toHexString());
	if (capTable == null) {
		capTable = new CapTableSchema(event.params.capTableAddress.toHexString());
		let contract = CapTableTemplateDetail.bind(event.params.capTableAddress);
		let owner = contract.try_owner();
		if (owner.reverted) {
			log.info("FOUND BAD CAP TABLE CONTRACT WITH ADDRESS: {}", [event.params.capTableAddress.toHexString()]);
			return;
		}
		let partitionsBytes = contract.totalPartitions();
		let partitions: Array<String> = [];
		for (let i = 0; i < partitionsBytes.length; i++) {
			partitions.push(partitionsBytes[i].toString());
		}

		capTable.name = contract.name().toString();
		capTable.partitions = partitions;
		capTable.symbol = contract.symbol().toString();
		capTable.orgnr = contract.symbol().toString(); // TODO - Should use capTableId which we get from the context of CapTableRegistry. THis allows a captable to claim whatever orgnumber the want.
		capTable.minter = owner.value;
		capTable.status = "APPROVED";
		capTable.registry = capTableRegistry.id;
		capTable.owner = owner.value;
		capTable.totalSupply = contract.totalSupply();

		let _controllers = contract.controllers().map<Bytes>((a) => a as Bytes);
		capTable.controllers = _controllers;
		capTable.save();
	}

	capTableRegistry.count = capTableRegistry.count + BigInt.fromI32(1);
	log.info("######## handleCapTableAdded: count {}", [capTableRegistry.count.toString()]);
	capTableRegistry.save();
	log.info("######## END handleCapTableAdded", []);
}

export function handleCapTableRemoved(event: CapTableRemoved): void {
	let capTable = CapTableSchema.load(event.params.capTableAddress.toHexString());
	if (capTable == null) {
		log.critical("LOGICAL SMART CONTRACT ERROR {}", ["capTable in handleCapTableApproved should always exist."]);
		return;
	}
	capTable.orgnr = "REMOVED_" + capTable.orgnr;
	capTable.status = "REMOVED";
	capTable.save();
}
// export function handleCapTableDeclined(event: capTableDeclined): void {
// 	let capTable = CapTableSchema.load(event.params.capTableAddress.toHexString());
// 	if (capTable == null) {
// 		log.critical("LOGICAL SMART CONTRACT ERROR {}", ["capTable in handleCapTableApproved should always exist."]);
// 		return;
// 	}
// 	let seperatorIndex = capTable.orgnr.indexOf("_");
// 	capTable.orgnr = capTable.orgnr.slice(seperatorIndex);
// 	capTable.orgnr = "DECLINED_" + capTable.orgnr;
// 	capTable.status = "DECLINED";
// 	capTable.save();
// }
