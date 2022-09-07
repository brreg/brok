// Operative types

export type CreateCapTableInput = Organisation & {
  shareholders: (NewShareholder & PartitionAmount & Partial<EthereumIdentifier>)[];
};

export type CapTable = Organisation &
  EthereumIdentifier &
  CeramicIdentifier & {
    totalShares: string;
    shareholders: (ShareholderIndetifier &
      EthereumIdentifier &
      Partial<EncumbranceIdentifier> &
      CeramicIdentifier &
      (ShareholderOrganization | ShareholderPerson) &
      ShareholderBalances)[];
  };

export type TransferInput = FromShareholder & PartitionAmount & (ExsistingShareholder | NewShareholder);
export type TransferRequest = FromShareholder & PartitionAmount & ExsistingShareholder;
export type IssueInput = PartitionAmount & (ExsistingShareholder | NewShareholder);
export type IssueRequest = PartitionAmount & ExsistingShareholder;
export type RedeemRequest = PartitionAmount & FromShareholder;

export type OperationResult = { success: boolean; message: string };

// == Models ==

export type Organisation = {
  orgnr: string;
  name: string;
};

export type FromShareholder = {
  from: EthereumAddress;
};
export type ExsistingShareholder = {
  to: EthereumAddress;
};
export type NewShareholder = ShareholderIndetifier & (ShareholderOrganization | ShareholderPerson);
export type ShareholderWithBalances = (ShareholderIndetifier &
  EthereumIdentifier &
  (ShareholderOrganization | ShareholderPerson) &
  ShareholderBalances)[];

export type Shareholder = ShareholderIndetifier & EthereumIdentifier & (ShareholderOrganization | ShareholderPerson) & Partial<EncumbranceIdentifier>;

export type ShareholderIndetifier = {
  name: string; // Personens fulle navn eller selskapsnavn
  postalcode: string;
  countryCode: string;
};

export type ShareholderOrganization = {
  organizationIdentifier: string; // Selskapets unike identifikator
  organizationIdentifierType: 'OrganizationNumber' | 'EUID' | 'LEI' | string;
};

export type ShareholderPerson = {
  birthDate: string; // Fylles kun ut for personer
};

export type ShareholderBalances = {
  balances: PartitionAmount[];
};

// CapTable types
export type Balance = EthereumIdentifier & PartitionAmount;
export type PartitionAmount = {
  amount: string;
  partition: string;
};

// Ethereum types
export type EthereumIdentifier = {
  ethAddress: EthereumAddress;
};

export type CapTableEthereumId = EthereumAddress;
export type ShareholderEthereumId = EthereumAddress;
export type EthereumAddress = string;

// Ceramic types
export type CeramicIdentifier = {
  ceramicID: CeramicID;
};

export type CeramicID = string;
export type CapTableCeramic = Organisation & {
  shareholderEthToCeramic: Record<EthereumAddress, CeramicID>;
};

// GraphQL types
export * from './utils/CapTableGraphQL.utils.js';

// encumbrances
export type CreatedDate = {
  dateCreated: string;
};
export type EncumbranceHolder = ShareholderIndetifier & (ShareholderOrganization | ShareholderPerson);
export type EncumbranceIdentifier = {
  encumbrance: Encumbrance;
};
export type Encumbrance = CreatedDate & EncumbranceHolder & PartitionAmount;
