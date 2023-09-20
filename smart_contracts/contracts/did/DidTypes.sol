// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @dev DidDocumentStorage holds the DID Document and its associated metadata
 */
struct DidDocumentStorage {
    DidDocument document;
    DidMetadata metadata;
}

/**
 * @dev VerificationRelationship links a DID to a verification method
 */
struct DidMetadata {
    uint256 created;
    uint256 updated;
    bool deactivated;
}

/**
 * @dev DidDocument represent the main DID Document structure.
 */
struct DidDocument {
    string[] context;
    string id;
    string[] controller;
    VerificationMethod[] verificationMethod;
    VerificationRelationship[] authentication;
    VerificationRelationship[] assertionMethod;
    VerificationRelationship[] capabilityInvocation;
    VerificationRelationship[] capabilityDelegation;
    VerificationRelationship[] keyAgreement;
    Service[] service;
    string[] alsoKnownAs;
}

/**
 * @dev VerificationMethod are used to define how to authenticate/authorise interactions with a DID subject or delegates.
 */
struct VerificationMethod {
    string id;
    string verificationMethodType;
    string controller;
    string publicKeyJwk;
    string publicKeyMultibase;
}

/**
 * @dev VerificationRelationship links a DID to a verification method.
 */
struct VerificationRelationship {
    string id;
    VerificationMethod verificationMethod;
}

/**
 * @dev Service describes a service endpoint related to the DID.
 */
struct Service {
    string id;
    string serviceType;
    string[] serviceEndpoint;
    string[] accept;
    string[] routingKeys;
}

/**
 * @dev Signature describes DID Document signature
 */
struct Signature {
    string id;
    string value;
}