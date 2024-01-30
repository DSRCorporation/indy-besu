use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
    Address,
};

use crate::contracts::did::types::did::DID;
use log::{trace, warn};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

pub const BASE_CONTEXT: &str = "https://www.w3.org/ns/did/v1";
pub const SECPK_CONTEXT: &str = "https://w3id.org/security/suites/secp256k1recovery-2020/v2";
pub const KEYS_CONTEXT: &str = "https://w3id.org/security/v3-unstable";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocumentWithMeta {
    pub did_document: DidDocument,
    pub did_document_metadata: DidMetadata,
    pub did_resolution_metadata: DidResolutionMetadata,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DidResolutionOptions {
    pub accept: String,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DidResolutionMetadata {
    pub content_type: String,
    pub error: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: StringOrVector,
    pub id: DID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<StringOrVector>,
    pub verification_method: Vec<VerificationMethod>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub authentication: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub assertion_method: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub capability_invocation: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub capability_delegation: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub key_agreement: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub service: Vec<Service>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidRecord {
    pub document: DidDocument,
    pub metadata: DidMetadata,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DidMetadata {
    pub owner: Address,
    pub sender: Address,
    pub created: u64,
    pub updated: u64,
    pub deactivated: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: VerificationKeyType,
    pub controller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_multibase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base58: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base64: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub enum VerificationKeyType {
    #[default]
    Ed25519VerificationKey2018,
    X25519KeyAgreementKey2019,
    Ed25519VerificationKey2020,
    X25519KeyAgreementKey2020,
    JsonWebKey2020,
    EcdsaSecp256k1VerificationKey2019,
    EcdsaSecp256k1VerificationKey2020,
    EcdsaSecp256k1RecoveryMethod2020,
}

impl ToString for VerificationKeyType {
    fn to_string(&self) -> String {
        match self {
            VerificationKeyType::Ed25519VerificationKey2018 => {
                "Ed25519VerificationKey2018".to_string()
            }
            VerificationKeyType::X25519KeyAgreementKey2019 => {
                "X25519KeyAgreementKey2019".to_string()
            }
            VerificationKeyType::Ed25519VerificationKey2020 => {
                "Ed25519VerificationKey2020".to_string()
            }
            VerificationKeyType::X25519KeyAgreementKey2020 => {
                "X25519KeyAgreementKey2020".to_string()
            }
            VerificationKeyType::JsonWebKey2020 => "JsonWebKey2020".to_string(),
            VerificationKeyType::EcdsaSecp256k1VerificationKey2019
            | VerificationKeyType::EcdsaSecp256k1VerificationKey2020 => {
                "EcdsaSecp256k1VerificationKey2019".to_string()
            }
            VerificationKeyType::EcdsaSecp256k1RecoveryMethod2020 => {
                "EcdsaSecp256k1RecoveryMethod2020".to_string()
            }
        }
    }
}

impl TryFrom<&str> for VerificationKeyType {
    type Error = VdrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Ed25519VerificationKey2018" => Ok(VerificationKeyType::Ed25519VerificationKey2018),
            "X25519KeyAgreementKey2019" => Ok(VerificationKeyType::X25519KeyAgreementKey2019),
            "Ed25519VerificationKey2020" => Ok(VerificationKeyType::Ed25519VerificationKey2020),
            "X25519KeyAgreementKey2020" => Ok(VerificationKeyType::X25519KeyAgreementKey2020),
            "JsonWebKey2020" => Ok(VerificationKeyType::JsonWebKey2020),
            "EcdsaSecp256k1RecoveryMethod2020" => {
                Ok(VerificationKeyType::EcdsaSecp256k1RecoveryMethod2020)
            }
            "EcdsaSecp256k1VerificationKey2019" => {
                Ok(VerificationKeyType::EcdsaSecp256k1VerificationKey2020)
            }
            _type => Err({
                let vdr_error = VdrError::CommonInvalidData(format!(
                    "Unexpected verification key type {}",
                    _type
                ));

                warn!(
                    "Error: {} during converting VerificationKeyType from String: {} ",
                    vdr_error, value
                );

                vdr_error
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum VerificationMethodOrReference {
    String(String),
    VerificationMethod(VerificationMethod),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub service_endpoint: ServiceEndpoint,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
    String(String),
    Object(ServiceEndpointObject),
    Set(Vec<ServiceEndpoint>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceEndpointObject {
    pub uri: String,
    pub accept: Vec<String>,
    pub routing_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOrVector {
    String(String),
    Vector(Vec<String>),
}

impl Default for StringOrVector {
    fn default() -> Self {
        let vector = StringOrVector::Vector(Vec::new());

        trace!("Created new StringOrVector::Vector: {:?}", vector);

        vector
    }
}

impl TryFrom<&DidDocument> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &DidDocument) -> Result<Self, Self::Error> {
        Ok(ContractParam::String(json!(value).to_string()))
    }
}

impl TryFrom<&ContractOutput> for DidDocument {
    type Error = VdrError;

    fn try_from(value: &ContractOutput) -> Result<Self, Self::Error> {
        serde_json::from_str(&value.get_string(0)?).map_err(|err| {
            VdrError::ContractInvalidResponseData(format!(
                "Unable to parse DID Document from the response. Err: {:?}",
                err
            ))
        })
    }
}

impl TryFrom<ContractOutput> for DidMetadata {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let owner = value.get_address(0)?;
        let sender = value.get_address(1)?;
        let created = value.get_u128(2)? as u64;
        let updated = value.get_u128(3)? as u64;
        let deactivated = value.get_bool(4)?;
        let did_metadata = DidMetadata {
            owner,
            sender,
            created,
            updated,
            deactivated,
        };
        Ok(did_metadata)
    }
}

impl TryFrom<ContractOutput> for DidRecord {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let output_tuple = value.get_tuple(0)?;
        let did_document = DidDocument::try_from(&output_tuple)?;
        let metadata = output_tuple.get_tuple(1)?;
        let did_doc_with_metadata = DidRecord {
            document: did_document,
            metadata: DidMetadata::try_from(metadata)?,
        };
        Ok(did_doc_with_metadata)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::utils::rand_bytes;

    pub const ISSUER_ID: &str = "did:ethr:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5";
    pub const CONTEXT: &str = "https://www.w3.org/ns/did/v1";
    pub const MULTIBASE_KEY: &str = "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf";
    pub const SERVICE_ENDPOINT: &str = "http://example.com";
    pub const SERVICE_TYPE: &str = "Service";
    pub const KEY_1: &str = "KEY-1";

    pub fn verification_method(id: &str) -> VerificationMethod {
        VerificationMethod {
            id: format!("{}#{}", id, KEY_1),
            type_: VerificationKeyType::Ed25519VerificationKey2018,
            controller: id.to_string(),
            public_key_multibase: Some(MULTIBASE_KEY.to_string()),
            public_key_hex: None,
            public_key_base58: None,
            blockchain_account_id: None,
            public_key_base64: None,
        }
    }

    pub fn verification_relationship(id: &str) -> VerificationMethodOrReference {
        VerificationMethodOrReference::String(format!("{}#{}", id, KEY_1))
    }

    pub fn _service(id: &str) -> Service {
        Service {
            id: id.to_string(),
            type_: SERVICE_TYPE.to_string(),
            service_endpoint: ServiceEndpoint::String(SERVICE_ENDPOINT.to_string()),
        }
    }

    pub fn new_id() -> String {
        format!(
            "did:ethr:testnet:{}",
            &bs58::encode(rand_bytes()).into_string()
        )
    }

    pub fn did_doc(id: Option<&str>) -> DidDocument {
        let id = id.map(String::from).unwrap_or_else(new_id);
        DidDocument {
            context: StringOrVector::Vector(vec![CONTEXT.to_string()]),
            id: DID::from(id.as_str()),
            controller: None,
            verification_method: vec![verification_method(&id)],
            authentication: vec![verification_relationship(&id)],
            assertion_method: vec![],
            capability_invocation: vec![],
            capability_delegation: vec![],
            key_agreement: vec![],
            service: vec![],
            also_known_as: Some(vec![]),
        }
    }

    fn did_doc_param() -> ContractParam {
        ContractParam::String(json!(did_doc(Some(ISSUER_ID))).to_string())
    }

    mod convert_into_contract_param {
        use super::*;

        #[test]
        fn convert_did_doc_into_contract_param_test() {
            let param: ContractParam = (&did_doc(Some(ISSUER_ID))).try_into().unwrap();
            assert_eq!(did_doc_param(), param);
        }
    }

    mod convert_into_object {
        use super::*;

        #[test]
        fn convert_contract_output_into_did_doc() {
            let data = ContractOutput::new(vec![did_doc_param()]);
            let converted = DidDocument::try_from(&data).unwrap();
            assert_eq!(did_doc(Some(ISSUER_ID)), converted);
        }
    }
}
