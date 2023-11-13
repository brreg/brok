// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import '@openzeppelin/contracts/access/AccessControl.sol';

contract CapTableRegistry is AccessControl {
    address[] internal _capTables; // list of all capTables added AND removed
    uint256 internal _activeCapTables;
    mapping(address => string) internal _addressToId; // address => orgnr
    mapping(string => address) internal _idToAddress; // id = orgnr
    mapping(address => uint256) internal _addressToStatus; // // 0:notCreated 1:notUsed 2:approved 3:notUsed 4:removed 5:notUsed
    mapping(address => address) private _operatorOf; // capTable => operator

    bytes32 public constant OPERATOR_ROLE = keccak256('OPERATOR_ROLE');

    event CapTableAdded(address indexed capTableAddress, string indexed id);
    event CapTableRemoved(address indexed capTableAddress, string indexed id);

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(OPERATOR_ROLE, msg.sender);
    }

    // TODO When a captable is added, then removed, and added again, it's pushed to _capTables twice. Consider if the push should be omitted if the captable is already in the array
    function addCapTable(address adr, string calldata id) external onlyRole(OPERATOR_ROLE) {
        // id is orgnr, must be a string!
        // Make sure the id is not already registered
        require(_idToAddress[id] == address(0), 'id is already in use');
        require(adr != address(0), 'address cannot be zero address');
        bool emptyIdOnAddress = bytes(_addressToId[adr]).length == 0;
        require(emptyIdOnAddress, 'address is already in use');
        unchecked {
            _capTables.push(adr);
            _idToAddress[id] = adr;
            _addressToId[adr] = id;
            _activeCapTables++;
            _operatorOf[adr] = msg.sender;
            _addressToStatus[adr] = 2;
        }
        emit CapTableAdded(adr, id);
    }

    // Note that the captable will still be present in the capTables array
    function removeCapTable(address adr) external onlyRole(OPERATOR_ROLE) {
        string memory id = _addressToId[adr];
        require(_idToAddress[id] != address(0), 'no address registered on id');
        bool emptyIdOnAddress = bytes(id).length == 0;
        require(!emptyIdOnAddress, 'no id registered on address');

        unchecked {
            _idToAddress[id] = address(0);
            _addressToId[adr] = string('');
            _activeCapTables--;
            _operatorOf[adr] = address(0);
            _addressToStatus[adr] = 4;
        }
        emit CapTableRemoved(adr, id);
    }

    function getOperatorForCapTable(address adr) external view returns (address) {
        address operator = _operatorOf[adr];
        return hasRole(OPERATOR_ROLE, operator) ? operator : address(0);
    }

    function getActiveCapTablesCount() external view returns (uint256 activeCapTables) {
        return _activeCapTables;
    }

    function getCapTableList() external view returns (address[] memory capTableList) {
        return _capTables;
    }

    function getId(address adr) external view returns (string memory id) {
        return _addressToId[adr];
    }

    function getStatus(address adr) external view returns (uint256 status) {
        return _addressToStatus[adr];
    }

    function getAddress(string calldata id) external view returns (address capTableAddress) {
        return _idToAddress[id];
    }
}
