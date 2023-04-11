// SPDX-License-Identifier: ISC

pragma solidity ^0.8.0;
import './../ERC1400.sol';
import 'hardhat/console.sol';
import './CapTableRegistry.sol';

contract CapTable is ERC1400 {
    event NewCapTable(string indexed orgnr, address indexed fagsystem);
    bool public isAddedToRegistry = false;

    constructor(
        string memory name,
        string memory orgnr,
        uint256 granularity,
        address[] memory controllers,
        bytes32[] memory defaultPartitions
    ) ERC1400(name, orgnr, granularity, controllers, defaultPartitions) {
        emit NewCapTable(orgnr, msg.sender);
    }

    function confirmAddedToRegistry(address registryAddress) external {
        console.log("confirmAddedToRegistry %s", registryAddress);
        require(!isAddedToRegistry, 'CapTable is already added to registry');
        CapTableRegistry registry = CapTableRegistry(registryAddress);
        uint256 status = registry.getStatus(address(this));
        console.log('status %s', status);
        require(status == 2, 'CapTable is not approved');
        isAddedToRegistry = true;
    }

    function _issueByPartition(bytes32 toPartition, address operator, address to, uint256 value, bytes memory data) internal virtual override  {
        require(isAddedToRegistry, 'CapTable is not added to registry');
        super._issueByPartition(toPartition, operator, to, value, data);
    }

    function getOrgnr() public view returns (string memory) {
        return _symbol;
    }

    function kapitalforhoyselse_nye_aksjer(
        bytes32[] memory partition,
        address[] memory to,
        uint256[] memory value,
        bytes memory data
    ) external onlyMinter isIssuableToken {
        for (uint256 i = 0; i < to.length; i++) {
            _issueByPartition(partition[i], msg.sender, to[i], value[i], data);
        }
    }

    function splitt(bytes32[] memory partition, address[] memory to, uint256[] memory value, bytes memory data) external {
        for (uint256 i = 0; i < to.length; i++) {
            // no new shareholders
            require(_balances[to[i]] != uint256(0), 'No new shareholders');
            _issueByPartition(partition[i], msg.sender, to[i], value[i], data);
        }
    }

    function kapitalnedsettelse_reduksjon_aksjer(
        bytes32[] memory partition,
        address[] memory from,
        uint256[] memory value,
        bytes memory data,
        bytes memory operatorData
    ) external {
        for (uint256 i = 0; i < from.length; i++) {
            _redeemByPartition(partition[i], msg.sender, from[i], value[i], data, operatorData);
        }
    }

    function spleis(bytes32[] memory partition, address[] memory from, uint256[] memory value, bytes memory data, bytes memory operatorData) external {
        for (uint256 i = 0; i < from.length; i++) {
            _redeemByPartition(partition[i], msg.sender, from[i], value[i], data, operatorData);
        }
    }
}
