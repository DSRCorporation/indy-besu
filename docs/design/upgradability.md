# Upgrading Smart Contract

Upgrading smart contracts is a challenging task because they are immutable once deployed on a blockchain. This makes fixing bugs or adding new features difficult. However, using proxy smart contracts provides a way to overcome this limitation

## Proxy Smart Contracts

A proxy contract is a smart contract that forwards function calls to another contract, known as the implementation contract. The proxy contract holds the state, while the implementation contract contains the logic. When you want to upgrade the contract, you deploy a new implementation and update the proxy to point to this new address. This way, you can change the contract’s behavior without affecting its address. There are several common patterns of proxy implementation.

### Proxy Smart Contract Patterns

#### Transparent proxy pattern ([EIP-1538](https://eips.ethereum.org/EIPS/eip-1538))
In the transparent proxy pattern used for upgrade smart contracts, the implementation of updates resides on the proxy side. The proxy utilizes its own memory to store both the state of the contract and the address of the current implementation. The proxy implemets a fallback function that uses [delegatecall](https://solidity-by-example.org/delegatecall/) to invoke the appropriate function in the linked implementation. Usually, only the admin or owner of this contract has the authority to update the implementation. A notable challenge with this pattern is the potential for function clashes. This occurs when the 4-byte hash of a function signature in the implementation matches that of a function in the proxy, leading to unintentional behavior and potential vulnerabilities. One common solution to mitigate this problem is to ensure that only the contract administrator has the rights to call proxy-specific methods, while other accounts are restricted to calling methods from the implementation.

**Pros**: Relatively straidforward to implement
**Cons**: Deployment can be gas inefficent

#### UUPS proxy pattern ([EIP-1822](https://eips.ethereum.org/EIPS/eip-1822))
The UUPS proxy pattern is similar to the transparent proxy pattern. However, a distinct difference lies in where the update logic is implemented. In the UUPS pattern, the update logic is placed within the implementation contract, allowing for the possibility to modify this logic or even remove it entirely in the future. This introduces potential risks. If a bug is present in a new implementation,  the contract update logic can be broken. While it's advisable to implement safeguards against unintentional breakdown of the update functionality, it's essential to note that these can't guard against intentional breakdowns.

**Pros**: Offers the options to update or eliminate upgrade logic.
**Cons**: Upgrade should be executed carefally as it can break upgrage logic.

#### Beacon
Beacon proxy is a proxy pattern which is a separate contract that holds the logic address for one or more proxy contracts. In this setup, the proxy contract doesn’t store the address of the logic contract directly. Instead, it points to a beacon contract, which in turn points to the logic contract. This allows multiple proxy contracts to share a single logic contract through a common beacon, making it easier to manage upgrades for a group of proxy contracts.

**Pros**: Highly efficient for deploying contracts with similar logic and reduces costs by using one implementation and beacon proxy.
**Cons**: Complex to implement and maintain

#### Diamond proxy pattern ([EIP-2535](https://eips.ethereum.org/EIPS/eip-2535))
Diamond proxy pattern introduces the concept of "Diamonds" which are a more modular approach to smart contract upgrades. A Diamond is a contract that delegates calls to multiple function implementations, known as “facets.” Facets can be added, replaced, or removed, allowing for more flexible and modular upgrades. This standard also includes a way to query which facets are currently active.

**Pros**: Helps in addressing smart contract size limitations and functionality can be upgraded incrementally.
**Cons**: Complex to implement and maintain

## Upgrading Smart Contracts using UUPS proxy pattern
Among the listed templates, the UUPS proxy template is the most suitable for our needs. It is not only straightforward to implement but also it provides flexability to update the upgrade logic in the future. The [OpenZeppelin](https://docs.openzeppelin.com/contracts/4.x/api/proxy) library can be used for this implementation. Conveniently, it offers seamless integration with the Hardhat library, simplifying our testing process. To implement this proxy using OpenZeppelin, one only needs to inherit from the [UUPSUpgradeable](https://docs.openzeppelin.com/contracts/4.x/api/proxy#UUPSUpgradeable) contract. This contract provides an abstract method, [_authorizeUpgrade](https://docs.openzeppelin.com/contracts/4.x/api/proxy#UUPSUpgradeable-_authorizeUpgrade-address-), where we can define our own upgrade permission mechanism. Additionally, it has a built-in security feature to prevent any upgrades to non-UUPS compliant implementations. This is essential as it avoids unintentional upgrades to an implementation contract lacking the requisite upgrade mechanisms, which would otherwise permanently lock the proxy's upgradeability.

### Upgrade Control Smart Contract's methods

* Method: **ensureSufficientApprovals**
* Description: This transaction ensures that an implementation upgrade has received sufficient approvals. At least 60% of users with the trustee role should approve before proceeding. If approvals are insufficient, the transaction will be reverted with a `InsufficientApprovals` error. It can be invoked within the [_authorizeUpgrade](https://docs.openzeppelin.com/contracts/4.x/api/proxy#UUPSUpgradeable-_authorizeUpgrade-address-) method of the [UUPSUpgradeable](https://docs.openzeppelin.com/contracts/4.x/api/proxy#UUPSUpgradeable) contract.
* Restrictions: None.
* Format:
  ```
  UpgradeController.ensureSufficientApprovals(address proxy, address implementation)
  ```
* Method: **approve**
* Description: Transaction to approve specefic contract implementation. Once over 60 percent of approvals are received, this function upgrades implementation.
* Restrictions:
  * Sender must have TRUSTEE role assigned
  * Implementation must be [UUPSUpgradeable](https://docs.openzeppelin.com/contracts/4.x/api/proxy#UUPSUpgradeable)
* Format
  ```
  UpgradeController.approve(address proxy, address implementation)
  ```
#### Usage Example

```
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy";

contract UpgradableContract is UUPSUpgradable {

    UpgradeControlInterface _upgradeControl;

    constructor(address upgradeControlAddress) {
        _upgradeControl = UpgradeControlInterface(upgradeControlAddress);
    }

    function _authorizeUpgrade(address newImplementation) internal override {
      _upgradeControl.ensureSufficientApprovals(address(this), newImplementation);
    }
}
```


