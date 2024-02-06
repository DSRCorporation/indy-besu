use crate::{
    client::client::test::client,
    contracts::{
        auth::Role,
        cl::types::{credential_definition::test::credential_definition, schema::test::schema},
        did::{DID, ETHR_DID_METHOD},
    },
    error::VdrResult,
    signer::basic_signer::{
        test::{basic_signer, TRUSTEE_ACC},
        BasicSigner,
    },
    types::{Address, SignatureData, Transaction},
    LedgerClient, TransactionEndorsingData,
};

fn did(address: &Address) -> DID {
    DID::build(ETHR_DID_METHOD, None, address.as_ref())
}

async fn sign_and_submit_transaction(
    client: &LedgerClient,
    transaction: Transaction,
    signer: &BasicSigner,
) -> String {
    let sign_bytes = transaction.get_signing_bytes().unwrap();
    let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.as_ref()).unwrap();
    transaction.set_signature(signature);
    let block_hash = client.submit_transaction(&transaction).await.unwrap();
    client.get_receipt(&block_hash).await.unwrap()
}

fn sign_endorsing_data(data: &TransactionEndorsingData, signer: &BasicSigner) -> SignatureData {
    signer
        .sign(&data.get_signing_bytes().unwrap(), data.from.as_ref())
        .unwrap()
}

mod did {
    use super::*;
    use crate::{
        contracts::{
            did::{
                did_ethr_registry,
                did_ethr_registry::test::{public_key, service, validity},
                types::{
                    did_doc::test::{default_ethr_did_document, TEST_DID_ETHR},
                    did_doc_attribute::DidDocAttribute,
                },
                DID,
            },
            types::did::ParsedDid,
        },
        did_ethr_registry::test::{public_key_2, public_key_3},
        Address, LedgerClient, Validity, VdrResult,
    };

    async fn endorse_set_did_attribute(
        client: &LedgerClient,
        did: &DID,
        attribute: &DidDocAttribute,
        validity: &Validity,
        signer: &BasicSigner,
    ) {
        let transaction_endorsing_data = did_ethr_registry::build_did_set_attribute_endorsing_data(
            client, did, attribute, validity,
        )
        .await
        .unwrap();

        let signature = sign_endorsing_data(&transaction_endorsing_data, signer);

        let transaction = did_ethr_registry::build_did_set_attribute_signed_transaction(
            client,
            &TRUSTEE_ACC,
            did,
            attribute,
            validity,
            &signature,
        )
        .await
        .unwrap();

        sign_and_submit_transaction(&client, transaction, &signer).await;
    }

    async fn endorse_revoke_did_attribute(
        client: &LedgerClient,
        did: &DID,
        attribute: &DidDocAttribute,
        signer: &BasicSigner,
    ) -> String {
        let transaction_endorsing_data =
            did_ethr_registry::build_did_revoke_attribute_endorsing_data(client, did, attribute)
                .await
                .unwrap();

        let signature = sign_endorsing_data(&transaction_endorsing_data, signer);

        let transaction = did_ethr_registry::build_did_revoke_attribute_signed_transaction(
            client,
            &TRUSTEE_ACC,
            did,
            attribute,
            &signature,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await
    }

    #[async_std::test]
    async fn demo_create_did_ethr() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        let did = super::did(&TRUSTEE_ACC.clone());

        // read DID changed block -> it must be none
        let transaction = did_ethr_registry::build_get_did_changed_transaction(&client, &did)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        let changed = did_ethr_registry::parse_did_changed_result(&client, &result).unwrap();
        assert!(changed.is_none());

        // add service attribute to DID
        let transaction = did_ethr_registry::build_did_set_attribute_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &service(),
            &validity(),
        )
        .await
        .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await;

        // Read DID events
        let transaction = did_ethr_registry::build_get_did_events_query(&client, &did, None, None)
            .await
            .unwrap();
        let events = client.query_events(&transaction).await.unwrap();
        assert_eq!(1, events.len());
        let event = did_ethr_registry::parse_did_event_response(&client, &events[0]).unwrap();
        let _attribute: DidDocAttribute = event.try_into().unwrap();

        // read DID changed block -> it must be NOT none
        let transaction = did_ethr_registry::build_get_did_changed_transaction(&client, &did)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        let changed = did_ethr_registry::parse_did_changed_result(&client, &result).unwrap();
        assert!(!changed.is_none());

        // add service key to DID
        let transaction = did_ethr_registry::build_did_set_attribute_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &public_key(),
            &validity(),
        )
        .await
        .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await;

        // resolve DID document
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document = did_doc_with_meta.did_document.unwrap();
        assert_eq!(1, did_document.service.len());
        assert_eq!(2, did_document.verification_method.len());
        assert_eq!(
            false,
            did_doc_with_meta.did_document_metadata.deactivated.unwrap()
        );

        Ok(())
    }

    #[async_std::test]
    async fn demo_endorse_did_ethr() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        let did = super::did(&identity);

        // endorse service attribute
        endorse_set_did_attribute(&client, &did, &service(), &validity(), &signer).await;

        // endorse key attribute
        endorse_set_did_attribute(&client, &did, &public_key(), &validity(), &signer).await;

        // resolve DID document
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document = did_doc_with_meta.did_document.unwrap();
        assert_eq!(1, did_document.service.len());
        assert_eq!(2, did_document.verification_method.len());

        Ok(())
    }

    #[async_std::test]
    async fn demo_did_ethr_deactivate() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        let did = super::did(&identity);

        // add service attribute
        let service = service();
        let validity = validity();
        endorse_set_did_attribute(&client, &did, &service, &validity, &signer).await;

        // deactivate DID
        let new_owner = Address::null();
        let transaction_endorsing_data =
            did_ethr_registry::build_did_change_owner_endorsing_data(&client, &did, &new_owner)
                .await
                .unwrap();

        let signature = sign_endorsing_data(&transaction_endorsing_data, &signer);

        let transaction = did_ethr_registry::build_did_change_owner_signed_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &new_owner,
            &signature,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await;

        // Resole DID document
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document = did_doc_with_meta.did_document.unwrap();

        // DID is deactivated
        assert!(did_doc_with_meta.did_document_metadata.deactivated.unwrap());

        // DID Document is empty
        let parse_did = ParsedDid::try_from(&did).unwrap();
        assert_eq!(parse_did.as_short_did(), did_document.id);
        assert_eq!(0, did_document.service.len());
        assert_eq!(0, did_document.verification_method.len());
        assert_eq!(0, did_document.authentication.len());
        assert_eq!(0, did_document.assertion_method.len());

        Ok(())
    }

    #[async_std::test]
    async fn demo_did_add_remove_attribute() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        let did = super::did(&identity);

        // set service attribute
        let service = service();
        let validity = validity();
        endorse_set_did_attribute(&client, &did, &service, &validity, &signer).await;

        // set first key attribute
        let public_key = public_key();
        endorse_set_did_attribute(&client, &did, &public_key, &validity, &signer).await;

        // set second key attribute
        let public_key_2 = public_key_2();
        endorse_set_did_attribute(&client, &did, &public_key_2, &validity, &signer).await;

        // resolve DID document
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document_before_remove = did_doc_with_meta.did_document.unwrap();
        assert_eq!(1, did_document_before_remove.service.len());
        assert_eq!(3, did_document_before_remove.verification_method.len());
        assert_eq!(2, did_document_before_remove.key_agreement.len());
        assert_eq!(1, did_document_before_remove.authentication.len());
        assert_eq!(1, did_document_before_remove.assertion_method.len());

        // remove service and second key
        endorse_revoke_did_attribute(&client, &did, &public_key, &signer).await;
        endorse_revoke_did_attribute(&client, &did, &service, &signer).await;

        // resolve DID document
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document_after_remove = did_doc_with_meta.did_document.unwrap();
        assert_eq!(0, did_document_after_remove.service.len());
        assert_eq!(2, did_document_after_remove.verification_method.len());
        assert_eq!(1, did_document_after_remove.key_agreement.len());
        assert_eq!(1, did_document_after_remove.authentication.len());
        assert_eq!(1, did_document_after_remove.assertion_method.len());

        // add third key
        let public_key_3 = public_key_3();
        endorse_set_did_attribute(&client, &did, &public_key_3, &validity, &signer).await;

        // resolve DID document
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document_after_add = did_doc_with_meta.did_document.unwrap();
        assert_eq!(0, did_document_after_add.service.len());
        assert_eq!(3, did_document_after_add.verification_method.len());
        assert_eq!(1, did_document_after_add.key_agreement.len());
        assert_eq!(1, did_document_after_add.authentication.len());
        assert_eq!(2, did_document_after_add.assertion_method.len());

        Ok(())
    }

    #[async_std::test]
    async fn demo_resolve_offchain_did() -> VdrResult<()> {
        let client = client();

        let did = DID::from(TEST_DID_ETHR);

        // Resole DID document
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document = did_doc_with_meta.did_document.unwrap();

        // DID Document is empty
        assert_eq!(
            default_ethr_did_document(Some(client.chain_id())),
            did_document
        );

        Ok(())
    }
}

mod schema {
    use super::*;
    use crate::{schema_registry, LedgerClient, Schema, SchemaId, DID};

    pub(crate) async fn endorse_schema(
        client: &LedgerClient,
        did: &DID,
        signer: &BasicSigner,
    ) -> (SchemaId, Schema) {
        let (schema_id, schema) = schema(did, None);
        let transaction_endorsing_data =
            schema_registry::build_create_schema_endorsing_data(client, &schema_id, &schema)
                .await
                .unwrap();

        let signature = sign_endorsing_data(&transaction_endorsing_data, signer);

        let transaction = schema_registry::build_create_schema_signed_transaction(
            client,
            &TRUSTEE_ACC.clone(),
            &schema_id,
            &schema,
            &signature,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await;
        (schema_id, schema)
    }

    #[async_std::test]
    async fn demo_create_schema() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // create DID
        let did = super::did(&TRUSTEE_ACC.clone());

        // write
        let (schema_id, schema) = schema(&did, None);
        let transaction = schema_registry::build_create_schema_transaction(
            &client,
            &TRUSTEE_ACC.clone(),
            &schema_id,
            &schema,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await;

        // read
        let resolved_schema = schema_registry::resolve_schema(&client, &schema_id)
            .await
            .unwrap();
        assert_eq!(schema, resolved_schema);

        Ok(())
    }

    #[async_std::test]
    async fn demo_endorse_schema() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        // create DID
        let did = super::did(&identity);

        // endorse schema
        let (schema_id, schema) = endorse_schema(&client, &did, &signer).await;

        // read
        let resolved_schema = schema_registry::resolve_schema(&client, &schema_id)
            .await
            .unwrap();
        assert_eq!(schema, resolved_schema);

        Ok(())
    }
}

mod credential_definition {
    use super::*;
    use crate::{credential_definition_registry, schema_registry};

    #[async_std::test]
    async fn demo_create_credential_definition() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // create DID
        let did = super::did(&TRUSTEE_ACC.clone());

        // create Schema
        let (schema_id, schema) = schema(&did, None);
        let transaction = schema_registry::build_create_schema_transaction(
            &client,
            &TRUSTEE_ACC,
            &schema_id,
            &schema,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await;

        // write
        let (credential_definition_id, credential_definition) =
            credential_definition(&did, &schema_id, None);
        let transaction =
            credential_definition_registry::build_create_credential_definition_transaction(
                &client,
                &TRUSTEE_ACC,
                &credential_definition_id,
                &credential_definition,
            )
            .await
            .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await;

        // read
        let resolved_credential_definition =
            credential_definition_registry::resolve_credential_definition(
                &client,
                &credential_definition_id,
            )
            .await
            .unwrap();
        assert_eq!(credential_definition, resolved_credential_definition);

        Ok(())
    }

    #[async_std::test]
    async fn demo_endorse_credential_definition() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        // create DID Document
        let did = super::did(&identity);

        // create Schema
        let (schema_id, _) = super::schema::endorse_schema(&client, &did, &signer).await;

        // write
        let (credential_definition_id, credential_definition) =
            credential_definition(&did, &schema_id, None);
        let transaction_endorsing_data =
            credential_definition_registry::build_create_credential_definition_endorsing_data(
                &client,
                &credential_definition_id,
                &credential_definition,
            )
            .await
            .unwrap();

        let signature = sign_endorsing_data(&transaction_endorsing_data, &signer);

        let transaction =
            credential_definition_registry::build_create_credential_definition_signed_transaction(
                &client,
                &TRUSTEE_ACC.clone(),
                &credential_definition_id,
                &credential_definition,
                &signature,
            )
            .await
            .unwrap();
        sign_and_submit_transaction(&client, transaction, &signer).await;

        // read
        let resolved_credential_definition =
            credential_definition_registry::resolve_credential_definition(
                &client,
                &credential_definition_id,
            )
            .await
            .unwrap();
        assert_eq!(credential_definition, resolved_credential_definition);

        Ok(())
    }
}

mod role {
    use super::*;
    use crate::role_control;

    pub(crate) async fn build_and_submit_assign_role_transaction(
        client: &LedgerClient,
        assignee_account: &Address,
        role_to_assign: &Role,
        signer: &BasicSigner,
    ) -> String {
        let transaction = role_control::build_assign_role_transaction(
            client,
            &TRUSTEE_ACC,
            role_to_assign,
            assignee_account,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    async fn build_and_submit_revoke_role_transaction(
        client: &LedgerClient,
        revokee_account: &Address,
        role_to_revoke: &Role,
        signer: &BasicSigner,
    ) -> String {
        let transaction = role_control::build_revoke_role_transaction(
            client,
            &TRUSTEE_ACC,
            role_to_revoke,
            revokee_account,
        )
        .await
        .unwrap();

        let sign_bytes = transaction.get_signing_bytes().unwrap();
        let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.as_ref()).unwrap();
        transaction.set_signature(signature);

        let block_hash = client.submit_transaction(&transaction).await.unwrap();

        client.get_receipt(&block_hash).await.unwrap()
    }

    async fn build_and_submit_get_role_transaction(
        client: &LedgerClient,
        assignee_account: &Address,
    ) -> Role {
        let transaction = role_control::build_get_role_transaction(client, assignee_account)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        role_control::parse_get_role_result(&client, &result).unwrap()
    }

    async fn build_and_submit_has_role_transaction(
        client: &LedgerClient,
        role: &Role,
        assignee_account: &Address,
    ) -> bool {
        let transaction = role_control::build_has_role_transaction(client, role, assignee_account)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        role_control::parse_has_role_result(&client, &result).unwrap()
    }

    #[async_std::test]
    async fn demo_build_and_submit_assign_and_remove_role_transactions_test() -> VdrResult<()> {
        let signer = basic_signer();
        let (assignee_account, _) = signer.create_account(None).unwrap();
        let client = client();
        let role_to_assign = Role::Endorser;

        build_and_submit_assign_role_transaction(
            &client,
            &assignee_account,
            &role_to_assign,
            &signer,
        )
        .await;

        let assigned_role = build_and_submit_get_role_transaction(&client, &assignee_account).await;
        assert_eq!(role_to_assign, assigned_role);

        build_and_submit_revoke_role_transaction(
            &client,
            &assignee_account,
            &role_to_assign,
            &signer,
        )
        .await;

        let has_role =
            build_and_submit_has_role_transaction(&client, &role_to_assign, &assignee_account)
                .await;
        assert!(!has_role);

        Ok(())
    }
}

mod validator {
    use crate::{
        contracts::network::ValidatorAddresses, signer::basic_signer::test::basic_signer,
        validator_control,
    };

    use super::*;

    async fn build_and_submit_get_validators_transaction(
        client: &LedgerClient,
    ) -> ValidatorAddresses {
        let transaction = validator_control::build_get_validators_transaction(&client)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();

        validator_control::parse_get_validators_result(&client, &result).unwrap()
    }

    async fn build_and_submit_add_validator_transaction(
        client: &LedgerClient,
        new_validator_address: &Address,
        signer: &BasicSigner,
    ) -> String {
        let transaction = validator_control::build_add_validator_transaction(
            &client,
            &TRUSTEE_ACC,
            new_validator_address,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    async fn build_and_submit_remove_validator_transaction(
        client: &LedgerClient,
        validator_address: &Address,
        signer: &BasicSigner,
    ) -> String {
        // write
        let transaction = validator_control::build_remove_validator_transaction(
            &client,
            &TRUSTEE_ACC,
            validator_address,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    #[async_std::test]
    async fn demo_build_and_submit_transaction_test() -> VdrResult<()> {
        let signer = basic_signer();
        let (new_validator_address, _) = signer.create_account(None).unwrap();
        let client = client();
        role::build_and_submit_assign_role_transaction(
            &client,
            &TRUSTEE_ACC,
            &Role::Steward,
            &signer,
        )
        .await;

        build_and_submit_add_validator_transaction(&client, &new_validator_address, &signer).await;

        let validator_list = build_and_submit_get_validators_transaction(&client).await;
        assert_eq!(validator_list.len(), 5);
        assert!(validator_list.contains(&new_validator_address));

        build_and_submit_remove_validator_transaction(&client, &new_validator_address, &signer)
            .await;

        let validator_list = build_and_submit_get_validators_transaction(&client).await;
        assert_eq!(validator_list.len(), 4);
        assert!(!validator_list.contains(&new_validator_address));

        Ok(())
    }
}
