// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  TypedMap,
  Entity,
  Value,
  ValueKind,
  store,
  Bytes,
  BigInt,
  BigDecimal
} from "@graphprotocol/graph-ts";

export class CapTableRegistry extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save CapTableRegistry entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type CapTableRegistry must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("CapTableRegistry", id.toString(), this);
    }
  }

  static load(id: string): CapTableRegistry | null {
    return changetype<CapTableRegistry | null>(
      store.get("CapTableRegistry", id)
    );
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get count(): BigInt {
    let value = this.get("count");
    return value!.toBigInt();
  }

  set count(value: BigInt) {
    this.set("count", Value.fromBigInt(value));
  }

  get capTables(): Array<string> {
    let value = this.get("capTables");
    return value!.toStringArray();
  }

  set capTables(value: Array<string>) {
    this.set("capTables", Value.fromStringArray(value));
  }

  get address(): Bytes {
    let value = this.get("address");
    return value!.toBytes();
  }

  set address(value: Bytes) {
    this.set("address", Value.fromBytes(value));
  }
}

export class CapTable extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save CapTable entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type CapTable must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("CapTable", id.toString(), this);
    }
  }

  static load(id: string): CapTable | null {
    return changetype<CapTable | null>(store.get("CapTable", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get name(): string {
    let value = this.get("name");
    return value!.toString();
  }

  set name(value: string) {
    this.set("name", Value.fromString(value));
  }

  get symbol(): string {
    let value = this.get("symbol");
    return value!.toString();
  }

  set symbol(value: string) {
    this.set("symbol", Value.fromString(value));
  }

  get partitions(): Array<string> | null {
    let value = this.get("partitions");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toStringArray();
    }
  }

  set partitions(value: Array<string> | null) {
    if (!value) {
      this.unset("partitions");
    } else {
      this.set("partitions", Value.fromStringArray(<Array<string>>value));
    }
  }

  get status(): string {
    let value = this.get("status");
    return value!.toString();
  }

  set status(value: string) {
    this.set("status", Value.fromString(value));
  }

  get registry(): string | null {
    let value = this.get("registry");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set registry(value: string | null) {
    if (!value) {
      this.unset("registry");
    } else {
      this.set("registry", Value.fromString(<string>value));
    }
  }

  get totalSupply(): BigInt {
    let value = this.get("totalSupply");
    return value!.toBigInt();
  }

  set totalSupply(value: BigInt) {
    this.set("totalSupply", Value.fromBigInt(value));
  }

  get owner(): Bytes {
    let value = this.get("owner");
    return value!.toBytes();
  }

  set owner(value: Bytes) {
    this.set("owner", Value.fromBytes(value));
  }

  get minter(): Bytes {
    let value = this.get("minter");
    return value!.toBytes();
  }

  set minter(value: Bytes) {
    this.set("minter", Value.fromBytes(value));
  }

  get controllers(): Array<Bytes> {
    let value = this.get("controllers");
    return value!.toBytesArray();
  }

  set controllers(value: Array<Bytes>) {
    this.set("controllers", Value.fromBytesArray(value));
  }

  get orgnr(): string {
    let value = this.get("orgnr");
    return value!.toString();
  }

  set orgnr(value: string) {
    this.set("orgnr", Value.fromString(value));
  }

  get tokenHolders(): Array<string> | null {
    let value = this.get("tokenHolders");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toStringArray();
    }
  }

  set tokenHolders(value: Array<string> | null) {
    if (!value) {
      this.unset("tokenHolders");
    } else {
      this.set("tokenHolders", Value.fromStringArray(<Array<string>>value));
    }
  }
}

export class TokenHolder extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save TokenHolder entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type TokenHolder must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("TokenHolder", id.toString(), this);
    }
  }

  static load(id: string): TokenHolder | null {
    return changetype<TokenHolder | null>(store.get("TokenHolder", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get capTable(): string | null {
    let value = this.get("capTable");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set capTable(value: string | null) {
    if (!value) {
      this.unset("capTable");
    } else {
      this.set("capTable", Value.fromString(<string>value));
    }
  }

  get address(): Bytes {
    let value = this.get("address");
    return value!.toBytes();
  }

  set address(value: Bytes) {
    this.set("address", Value.fromBytes(value));
  }

  get balances(): Array<string> | null {
    let value = this.get("balances");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toStringArray();
    }
  }

  set balances(value: Array<string> | null) {
    if (!value) {
      this.unset("balances");
    } else {
      this.set("balances", Value.fromStringArray(<Array<string>>value));
    }
  }
}

export class Balance extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Balance entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Balance must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Balance", id.toString(), this);
    }
  }

  static load(id: string): Balance | null {
    return changetype<Balance | null>(store.get("Balance", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get amount(): BigInt {
    let value = this.get("amount");
    return value!.toBigInt();
  }

  set amount(value: BigInt) {
    this.set("amount", Value.fromBigInt(value));
  }

  get partition(): string {
    let value = this.get("partition");
    return value!.toString();
  }

  set partition(value: string) {
    this.set("partition", Value.fromString(value));
  }

  get tokenHolder(): string | null {
    let value = this.get("tokenHolder");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set tokenHolder(value: string | null) {
    if (!value) {
      this.unset("tokenHolder");
    } else {
      this.set("tokenHolder", Value.fromString(<string>value));
    }
  }

  get capTable(): string | null {
    let value = this.get("capTable");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set capTable(value: string | null) {
    if (!value) {
      this.unset("capTable");
    } else {
      this.set("capTable", Value.fromString(<string>value));
    }
  }
}
