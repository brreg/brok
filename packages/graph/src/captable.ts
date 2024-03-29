import { dataSource, log } from "@graphprotocol/graph-ts";
import { CapTable as CapTableSchema, TokenHolder, Balance } from "../generated/schema";
import {
	CapTable,
	IssuedByPartition,
	RedeemedByPartition,
	TransferByPartition,
} from "../generated/templates/CapTable/CapTable";

let context = dataSource.context();
let capTableRegistryId = context.getString("capTableRegistryId");
let capTableId = context.getString("capTableId");

// export function handleNewCapTable(event: NewCapTable): void {}

export function handleIssuedByPartition(event: IssuedByPartition): void {
	let capTable = CapTableSchema.load(event.address.toHexString());
	if (capTable == null) {
		log.critical("LOGICAL SMART CONTRACT ERROR {}", ["issueByParition called but no capTable exists."]);
		return;
	} else {
		let contract = CapTable.bind(event.address);
		capTable.totalSupply = contract.totalSupply();
		capTable.save();
	}

	// Token holder

	let tokenHolder = TokenHolder.load(event.address.toHexString() + "-" + event.params.to.toHexString());
	if (tokenHolder == null) {
		tokenHolder = new TokenHolder(event.address.toHexString() + "-" + event.params.to.toHexString());
		tokenHolder.address = event.params.to;
		tokenHolder.capTable = capTable.id;
		tokenHolder.save();
	}

	//Balance
	let balance = Balance.load(
		event.address.toHexString() + "-" + event.params.to.toHexString() + "-" + event.params.partition.toString(),
	);
	if (balance == null) {
		balance = new Balance(
			event.address.toHexString() + "-" + event.params.to.toHexString() + "-" + event.params.partition.toString(),
		);
		balance.capTable = capTable.id;
		balance.partition = event.params.partition.toString();
		balance.tokenHolder = tokenHolder.id;
		balance.amount = event.params.value;
	} else {
		balance.amount = event.params.value.plus(balance.amount);
	}

	balance.save();
}

export function handleTransferByPartition(event: TransferByPartition): void {
	// fromTokenHolder balance adjustment
	let fromBalance = Balance.load(
		event.address.toHexString() + "-" + event.params.from.toHexString() + "-" + event.params.fromPartition.toString(),
	);
	if (fromBalance == null) {
		log.critical("LOGICAL SMART CONTRACT ERROR {}", ["fromBalance in handleTransferByPartition should always exist. "]);
		return;
	}
	fromBalance.amount = fromBalance.amount.minus(event.params.value);
	fromBalance.save();

	// toTokenHolder balance adjustment
	let toBalance = Balance.load(
		event.address.toHexString() + "-" + event.params.to.toHexString() + "-" + event.params.fromPartition.toString(),
	);
	if (toBalance) {
		toBalance.amount = toBalance.amount.plus(event.params.value);
		toBalance.save();
	} else {
		// if toBalance was not adjusted, it does not exist and we need to create it.
		let toTokenHolder = TokenHolder.load(event.address.toHexString() + "-" + event.params.to.toHexString());
		if (toTokenHolder == null) {
			toTokenHolder = new TokenHolder(event.address.toHexString() + "-" + event.params.to.toHexString());
			toTokenHolder.address = event.params.to;
			toTokenHolder.capTable = event.address.toHexString();
			toTokenHolder.save();
		}
		let toBalance = new Balance(
			event.address.toHexString() + "-" + event.params.to.toHexString() + "-" + event.params.fromPartition.toString(),
		);
		toBalance.partition = event.params.fromPartition.toString();
		toBalance.amount = event.params.value;
		toBalance.tokenHolder = toTokenHolder.id;
		toBalance.capTable = event.address.toHexString();
		toBalance.save();
	}
	// Update total supply
	let capTable = CapTableSchema.load(event.address.toHexString());
	if (capTable == null) {
		log.critical("LOGICAL SMART CONTRACT ERROR {}", ["fromBalance in handleRedeemByPartition should always exist. "]);
		return;
	}
	let contract = CapTable.bind(event.address);
	capTable.totalSupply = contract.totalSupply();
	capTable.save();
}

export function handleRedeemByPartition(event: RedeemedByPartition): void {
	// fromTokenHolder balance adjustment
	let fromBalance = Balance.load(
		event.address.toHexString() + "-" + event.params.from.toHexString() + "-" + event.params.partition.toString(),
	);
	if (fromBalance == null) {
		log.critical("LOGICAL SMART CONTRACT ERROR {}", ["fromBalance in handleRedeemByPartition should always exist. "]);
		return;
	}
	fromBalance.amount = fromBalance.amount.minus(event.params.value);
	fromBalance.save();

	// Update total supply
	let capTable = CapTableSchema.load(event.address.toHexString());
	if (capTable == null) {
		log.critical("LOGICAL SMART CONTRACT ERROR {}", ["fromBalance in handleRedeemByPartition should always exist. "]);
		return;
	}
	let contract = CapTable.bind(event.address);
	capTable.totalSupply = contract.totalSupply();
	capTable.save();
}
