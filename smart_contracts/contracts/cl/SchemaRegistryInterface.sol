// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { SchemaRecord } from "./SchemaTypes.sol";

interface SchemaRegistryInterface {
    /**
     * @dev Event that is sent when a Schema is created
     *
     * @param schemaId Created Schema ID
     */
    event SchemaCreated(string schemaId);

    /**
     * @dev Creates a new Schema.
     *
     * Once the Schema is created, this function emits a `SchemaCreated` event
     * with the new Schema ID.
     *
     * This function can revert with following errors:
     * - `SchemaAlreadyExist`: Raised if Schema with provided ID already exist.
     * - `IssuerNotFound`: Raised if the associated issuer doesn't exist.
     * - `IssuerHasBeenDeactivated`: Raised if the associated issuer is not active.
     * - `UnauthorizedIssuer`: Raised when an issuer DID specified in Schema is not owned by sender
     *
     * @param id        Id of schema to be created.
     * @param issuerId  Id of schema issuer.
     * @param schema    AnonCreds schema as JSON string.
     */
    function createSchema(string calldata id, string calldata issuerId, string calldata schema) external;

    /**
     * @dev Resolve the Schema associated with the given ID.
     *
     * If no matching Schema is found, the function revert with `SchemaNotFound` error
     *
     * @param id The ID of the Schema to be resolved.
     *
     * @return schemaRecord Returns the Schema with Metadata.
     */
    function resolveSchema(string calldata id) external returns (SchemaRecord memory schemaRecord);
}
