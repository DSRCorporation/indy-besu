// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface UpgradeControlInterface {

    /**
     * @dev Event that is sent when a contract implementation upgrade is approved.
     * 
     * @param proxy Address of the proxy contract.
     * @param implementation Address of the proposed new implementation.
     * @param sender Address of the entity that approved the upgrade.
     */
    event UpgradeApproved(address indexed proxy, address indexed implementation, address indexed sender);

    /// @dev Error emitted when the number of approvals is insufficient.
    error InsufficientApprovals();

    /**
     * @dev Approves a specific contract implementation for an upgrade.
     * 
     * On successful approval, this function emits an `UpgradeApproved` event, 
     * with the proxy, implementation, and approver addresses.
     *
     * When approvals exceed 60 percent, the implementation will be upgraded.
     * 
     * Restrictions:
     * - Only accounts with the trustee role can call this method.
     * - The provided implementation must be a UUPS upgradable contract.
     *
     * @param proxy The address of the proxy contract.
     * @param implementation The address of the proposed new implementation.
     */
    function approve(address proxy, address implementation) external;

    /**
     * @dev Ensures that an implementation upgrade has received sufficient approvals
     * 
     *  At least 60% of users with the trustee role should approve before proceeding. 
     *  If approvals are insufficient, the function will be reverted with a `InsufficientApprovals` error.
     * 
     * @param proxy Address of the proxy contract.
     * @param implementation Address of the proposed new implementation.
     */
    function ensureSufficientApprovals(address proxy, address implementation) external view;
}