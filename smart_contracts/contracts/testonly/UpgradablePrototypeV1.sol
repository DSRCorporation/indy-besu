// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { Initializable } from "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";

import { UpgradeControlInterface } from "../upgrade/UpgradeControlInterface.sol";

contract UpgradablePrototypeV1 is UUPSUpgradeable, Initializable {

    UpgradeControlInterface _upgradeControl;

    function initialize(address upgradeControlAddress) public initializer {
         _upgradeControl = UpgradeControlInterface(upgradeControlAddress);
    }

    function _authorizeUpgrade(address newImplementation) internal view override {
        _upgradeControl.ensureSufficientApprovals(address(this), newImplementation);
    }

    function getVersion() public pure returns (string memory version) {
        return "1.0";
    }
}