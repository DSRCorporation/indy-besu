use crate::{
    ledger::{BesuLedger, IndyLedger, Ledgers},
    wallet::{BesuWallet, IndyWallet},
};
use indy2_vdr::cl::types::{
    credential_definition_id::CredentialDefinitionId,
    credential_definition::migration::IndyCredentialDefinitionFormat,
    schema_id::SchemaId,
    schema::migration::IndySchemaFormat,
};
use serde_json::json;
use vdrtoolsrs::future::Future;

pub struct Holder {
    pub indy_wallet: IndyWallet,
    pub indy_ledger: IndyLedger,
    pub besu_ledger: BesuLedger,
    pub did: String,
    pub verkey: String,
    pub master_secret: String,
    pub used_ledger: Ledgers,
}

impl Holder {
    const NAME: &'static str = "holder";

    pub async fn setup() -> Holder {
        let indy_wallet = IndyWallet::new(Self::NAME);
        let indy_ledger = IndyLedger::new(Self::NAME);
        let besu_wallet = BesuWallet::new(None);
        let besu_ledger = BesuLedger::new(besu_wallet).await;
        let master_secret =
            vdrtoolsrs::anoncreds::prover_create_master_secret(indy_wallet.handle, None)
                .wait()
                .unwrap();
        let (did, verkey) = vdrtoolsrs::did::create_and_store_my_did(indy_wallet.handle, "{}")
            .wait()
            .unwrap();
        Holder {
            indy_wallet,
            indy_ledger,
            besu_ledger,
            did,
            verkey,
            master_secret,
            used_ledger: Ledgers::Indy,
        }
    }

    pub fn create_credential_request(&self, cred_offer: &str, cred_def: &str) -> (String, String) {
        vdrtoolsrs::anoncreds::prover_create_credential_req(
            self.indy_wallet.handle,
            &self.did,
            cred_offer,
            cred_def,
            &self.master_secret,
        )
        .wait()
        .unwrap()
    }

    pub fn store_credential(
        &self,
        cred_request_meta: &str,
        credential: &str,
        cred_def: &str,
    ) -> String {
        vdrtoolsrs::anoncreds::prover_store_credential(
            self.indy_wallet.handle,
            None,
            cred_request_meta,
            credential,
            cred_def,
            None,
        )
        .wait()
        .unwrap()
    }

    pub fn make_request_credentials(cred_id: &str) -> String {
        json!({
            "self_attested_attributes": {},
            "requested_attributes": {
                "attr1_referent": { "cred_id": cred_id, "revealed": true }
            },
            "requested_predicates": {}
        })
        .to_string()
    }

    pub async fn make_proof(&self, proof_request: &str, cred_id: &str) -> String {
        let requested_credentials = Holder::make_request_credentials(&cred_id);
        let credential =
            vdrtoolsrs::anoncreds::prover_get_credential(self.indy_wallet.handle, &cred_id)
                .wait()
                .unwrap();
        let credential = serde_json::from_str::<serde_json::Value>(&credential).unwrap();
        let schema_id = credential["schema_id"].as_str().unwrap();
        let cred_def_id = credential["cred_def_id"].as_str().unwrap();

        let (schema, cred_def) = match self.used_ledger {
            Ledgers::Indy => {
                let (_, schema) = self.indy_ledger.get_schema(&schema_id);
                let (_, cred_def) = self.indy_ledger.get_cred_def(&cred_def_id);
                (schema, cred_def)
            }
            Ledgers::Besu => {
                let schema_id = SchemaId::from_legacy_form(schema_id);
                let cred_def_id = CredentialDefinitionId::from_indy_form(cred_def_id);
                let schema = self.besu_ledger.get_schema(&schema_id).await;
                let cred_def = self.besu_ledger.get_cred_def(&cred_def_id).await;
                let schema: IndySchemaFormat = schema.into();
                let cred_def: IndyCredentialDefinitionFormat = cred_def.into();
                (json!(schema).to_string(), json!(cred_def).to_string())
            }
        };

        let schemas_json =
            json!({schema_id: serde_json::from_str::<serde_json::Value>(&schema).unwrap()})
                .to_string();
        let cred_defs_json =
            json!({cred_def_id: serde_json::from_str::<serde_json::Value>(&cred_def).unwrap()})
                .to_string();

        vdrtoolsrs::anoncreds::prover_create_proof(
            self.indy_wallet.handle,
            &proof_request,
            &requested_credentials,
            &self.master_secret,
            &schemas_json,
            &cred_defs_json,
            "{}",
        )
        .wait()
        .unwrap()
    }

    pub fn use_indy_ledger(&mut self) {
        self.used_ledger = Ledgers::Indy
    }

    pub fn use_besu_ledger(&mut self) {
        self.used_ledger = Ledgers::Besu
    }
}