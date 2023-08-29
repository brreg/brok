// SPDX-License-Identifier: ISC
pragma solidity ^0.8.0;

import './../ERC1400.sol';
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
        require(!isAddedToRegistry, 'CapTable is already added to registry');
        CapTableRegistry registry = CapTableRegistry(registryAddress);
        uint256 status = registry.getStatus(address(this));
        require(status == 2, 'CapTable is not approved');
        isAddedToRegistry = true;
    }

    function _issueByPartition(bytes32 toPartition, address operator, address to, uint256 value, bytes memory data) internal virtual override {
        require(isAddedToRegistry, 'CapTable is not added to registry');
        super._issueByPartition(toPartition, operator, to, value, data);
    }

    function getOrgnr() public view returns (string memory) {
        return _symbol;
    }

    /**
     * @notice Utfører en nyemisjon ved å gjennomføre en kapitalforhøyelse og utstede nye aksjer til gitte lommebøker.
     * @dev Kun godkjente utstedere (mintere) kan kalle denne funksjonen, og den vil fungere kun hvis tokenet er utstedbart. // TODO Er ikke det litt rart? Burde ikke en operator kunne gjøre dette?
     * @param partition En liste med aksjeklassene som utstedes. // TODO Er det naturlig å utstede flere aksjeklasser samtidig?
     * @param to En liste med adressene til mottakerne som skal motta de nye aksjene.
     * @param value En liste med mengdene av aksjer hver mottaker skal motta.
     * @param data Ekstra data som kan sendes til ERC1400 for tilpassede handlinger.
     * // TODO Litt uheldig om en barnefunksjonen av denne sender et event om at det skjedde en emisjon, hvis det egentlig er ved opprettelse av aksjeeierboken på BRØK eller opprelse av selskapet
     * // TODO Trengs denne funksjonen eller kan man bruke batchIssueByPartition direkte? Hvis man ønsker en funksjon med dette navnet kan man likevel bruke batchIssueByPartition() inni her
     * // TODO spytter ikke ut noe event om at det skjer en kapitalforhøyelse, så det er kanskje ikke noe problem. Burde vel helst være ulike funksjoner med ulike events, slik at det blir pent og pyntelig i block explorer
     */
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

    /**
     * @notice Utfører en aksjesplitt ved å allokere aksjer til eksisterende aksjonærer, uten å introdusere nye aksjonærer.
     * @dev Aksjonærene som skal motta de splittede aksjene må allerede eie aksjer i selskapet (ingen nye aksjonærer tillatt).
     * @param partition En liste med partition identifiers for de forskjellige kategoriene eller gruppene av tokens.
     * @param to Adressene til de eksisterende aksjonærene som skal motta de splittede aksjene.
     * @param value Mengdene av aksjer hver mottaker skal motta.
     * @param data Ekstra data som kan sendes til ERC1400 for tilpassede handlinger.
     * TODO Vurdere å skrive om til en splitForhold-parameter istedenfor value-parameteren. Da kan smartkontrakten selv regne ut hvor mange aksjer hver aksjonær skal motta, eks 1:10 -> 100 aksjer blir til 1000 aksjer.
     * TODO Vurdere å kun ta en partition/aksjeklasse om gangen, og heller kalle funksjonen flere ganger for å splitte flere aksjeklasser samtidig.
     * TODO Vurdere om det er mulig å hente to-listen fra kontraktene istedenfor å måtte sende den inn som parameter. Det beste var om man kunne si "split(klasseA, 1:10)"
     */
    function splitt(bytes32[] memory partition, address[] memory to, uint256[] memory value, bytes memory data) external {
        for (uint256 i = 0; i < to.length; i++) {
            // no new shareholders
            require(_balances[to[i]] != uint256(0), 'No new shareholders');
            _issueByPartition(partition[i], msg.sender, to[i], value[i], data);
        }
    }

    /**
     * @notice Utfører en kapitalnedsettelse ved å redusere antall aksjer for gitte aksjonærer.
     * @param partition En liste med partition identifiers for de forskjellige kategoriene eller gruppene av tokens.
     * @param from Adressene til aksjonærene hvis aksjer skal reduseres.
     * @param value Mengdene av aksjer som skal trekkes fra hver aksjonær.
     * @param data Ekstra data som kan sendes til ERC1400 for tilpassede handlinger.
     * @param operatorData Ytterligere data som kan behandles av en eventuell operator extension.
     */
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

    /**
     * @notice Utfører en aksjespleis ved å redusere antall aksjer for gitte aksjonærer, noe som fører til at hver aksje representerer en større eierandel.
     * @param partition En liste med partition identifiers for de forskjellige kategoriene eller gruppene av tokens.
     * @param from Adressene til aksjonærene hvis aksjer skal spleises.
     * @param value Mengdene av aksjer som skal trekkes fra hver aksjonær i forbindelse med spleisen.
     * @param data Ekstra data som kan sendes til ERC400 for tilpassede handlinger.
     * @param operatorData Ytterligere data som kan behandles av en eventuell operator extension.
     * TODO Vurder samme endringer som i splitt-funksjonen.
     */
    function spleis(bytes32[] memory partition, address[] memory from, uint256[] memory value, bytes memory data, bytes memory operatorData) external {
        for (uint256 i = 0; i < from.length; i++) {
            _redeemByPartition(partition[i], msg.sender, from[i], value[i], data, operatorData);
        }
    }
}
