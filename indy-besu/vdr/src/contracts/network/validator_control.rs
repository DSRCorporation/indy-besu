use log::{debug, info};

use crate::{
    error::VdrResult,
    types::{Transaction, TransactionBuilder, TransactionParser, TransactionType},
    Address, LedgerClient,
};

use super::validator_info::ValidatorAddresses;

/// ValidatorControl contract methods
pub struct ValidatorControl;

impl ValidatorControl {
    const CONTRACT_NAME: &'static str = "ValidatorControl";
    const METHOD_ADD_VALIDATOR: &'static str = "addValidator";
    const METHOD_REMOVE_VALIDATOR: &'static str = "removeValidator";
    const METHOD_GET_VALIDATORS: &'static str = "getValidators";

    /// Build transaction to execute ValidatorControl.addValidator contract method to add a new Validator
    ///
    /// # Params
    /// - `client` client connected to the network where contract will be executed
    /// - `from` transaction sender account address
    /// - `validator_address` validator address to be added
    ///
    /// # Returns
    /// Write transaction to sign and submit
    pub async fn build_add_validator_transaction(
        client: &LedgerClient,
        from: &Address,
        validator_address: &Address,
    ) -> VdrResult<Transaction> {
        debug!(
            "{} txn build has started. Sender: {}, validator address: {}",
            Self::METHOD_ADD_VALIDATOR,
            from.value(),
            validator_address.value()
        );

        let transaction = TransactionBuilder::new()
            .set_contract(Self::CONTRACT_NAME)
            .set_method(Self::METHOD_ADD_VALIDATOR)
            .add_param(validator_address.clone().try_into()?)
            .set_type(TransactionType::Write)
            .set_from(from)
            .build(client)
            .await;

        info!(
            "{} txn build has finished. Result: {:?}",
            Self::METHOD_ADD_VALIDATOR,
            transaction
        );

        transaction
    }

    /// Build transaction to execute ValidatorControl.removeValidator contract method to remove an existing Validator
    ///
    /// # Params
    /// - `client` client connected to the network where contract will be executed
    /// - `from` transaction sender account address
    /// - `validator_address` validator address to be removed
    ///
    /// # Returns
    /// Write transaction to sign and submit
    pub async fn build_remove_validator_transaction(
        client: &LedgerClient,
        from: &Address,
        validator_address: &Address,
    ) -> VdrResult<Transaction> {
        debug!(
            "{} txn build has started. Sender: {}, validator address: {}",
            Self::METHOD_REMOVE_VALIDATOR,
            from.value(),
            validator_address.value()
        );

        let transaction = TransactionBuilder::new()
            .set_contract(Self::CONTRACT_NAME)
            .set_method(Self::METHOD_REMOVE_VALIDATOR)
            .add_param(validator_address.clone().try_into()?)
            .set_type(TransactionType::Write)
            .set_from(from)
            .build(client)
            .await;

        info!(
            "{} txn build has finished. Result: {:?}",
            Self::METHOD_REMOVE_VALIDATOR,
            transaction
        );

        transaction
    }

    /// Build transaction to execute ValidatorControl.getValidators contract method to get existing validators
    ///
    /// # Params
    /// - `client` client connected to the network where contract will be executed
    ///
    /// # Returns
    /// Read transaction to submit
    pub async fn build_get_validators_transaction(client: &LedgerClient) -> VdrResult<Transaction> {
        debug!("{} txn build has started", Self::METHOD_GET_VALIDATORS,);

        let transaction = TransactionBuilder::new()
            .set_contract(Self::CONTRACT_NAME)
            .set_method(Self::METHOD_GET_VALIDATORS)
            .set_type(TransactionType::Read)
            .build(client)
            .await;

        info!(
            "{} txn build has finished. Result: {:?}",
            Self::METHOD_GET_VALIDATORS,
            transaction
        );

        transaction
    }

    /// Parse the result of execution ValidatorControl.getValidators contract method to get existing validators
    ///
    /// # Params
    /// - `client` client connected to the network where contract will be executed
    /// - `bytes` result bytes returned from the ledger
    ///
    /// # Returns
    /// Parsed validator addresses
    pub fn parse_get_validators_result(
        client: &LedgerClient,
        bytes: &[u8],
    ) -> VdrResult<ValidatorAddresses> {
        debug!(
            "{} result parse has started. Bytes to parse: {:?}",
            Self::METHOD_GET_VALIDATORS,
            bytes
        );

        let result = TransactionParser::new()
            .set_contract(Self::CONTRACT_NAME)
            .set_method(Self::METHOD_GET_VALIDATORS)
            .parse::<ValidatorAddresses>(client, bytes);

        info!(
            "{} result parse has finished. Result: {:?}",
            Self::METHOD_GET_VALIDATORS,
            result
        );

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::test::{
            mock_client, CHAIN_ID, DEFAULT_NONCE, TRUSTEE_ACC, VALIDATOR_CONTROL_ADDRESS,
        },
        utils::init_env_logger,
    };
    use once_cell::sync::Lazy;

    pub static VALIDATOR_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::new("0x93917cadbace5dfce132b991732c6cda9bcc5b8a"));

    mod build_add_validator_transaction {
        use super::*;

        #[async_std::test]
        async fn build_add_validator_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = ValidatorControl::build_add_validator_transaction(
                &client,
                &TRUSTEE_ACC,
                &VALIDATOR_ADDRESS,
            )
            .await
            .unwrap();
            let expected_data = [
                77, 35, 140, 142, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 145, 124, 173, 186, 206,
                93, 252, 225, 50, 185, 145, 115, 44, 108, 218, 155, 204, 91, 138,
            ];

            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: VALIDATOR_CONTROL_ADDRESS.to_string(),
                nonce: Some(DEFAULT_NONCE),
                chain_id: CHAIN_ID,
                data: expected_data.into(),
                signature: None,
            };

            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_remove_validator_transaction {
        use super::*;

        #[async_std::test]
        async fn build_remove_validator_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = ValidatorControl::build_remove_validator_transaction(
                &client,
                &TRUSTEE_ACC,
                &VALIDATOR_ADDRESS,
            )
            .await
            .unwrap();
            let expected_data = [
                64, 161, 65, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 145, 124, 173, 186, 206,
                93, 252, 225, 50, 185, 145, 115, 44, 108, 218, 155, 204, 91, 138,
            ];

            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: VALIDATOR_CONTROL_ADDRESS.to_string(),
                nonce: Some(DEFAULT_NONCE),
                chain_id: CHAIN_ID,
                data: expected_data.into(),
                signature: None,
            };

            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_get_validators_transaction {
        use super::*;

        #[async_std::test]
        async fn build_get_validators_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let transaction = ValidatorControl::build_get_validators_transaction(&client)
                .await
                .unwrap();
            let encoded_method = [183, 171, 77, 181];

            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: VALIDATOR_CONTROL_ADDRESS.to_string(),
                nonce: None,
                chain_id: CHAIN_ID,
                data: encoded_method.into(),
                signature: None,
            };

            assert_eq!(expected_transaction, transaction);
        }
    }

    mod parse_get_validators_result {
        use std::vec;

        use super::*;

        #[test]
        fn parse_get_validators_result_test() {
            let client = mock_client();
            let validator_list_bytes = [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 145, 124, 173,
                186, 206, 93, 252, 225, 50, 185, 145, 115, 44, 108, 218, 155, 204, 91, 138, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 169, 124, 154, 175, 4, 241, 143, 48, 20, 195, 46,
                3, 109, 208, 172, 118, 218, 95, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 206, 65,
                47, 152, 131, 119, 227, 31, 77, 15, 241, 45, 116, 223, 115, 181, 28, 66, 208, 202,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 152, 193, 51, 68, 150, 97, 74, 237, 73, 210,
                232, 21, 38, 208, 137, 247, 38, 79, 237, 156,
            ];
            let expected_validator_list = vec![
                Address::new("0x93917cadbace5dfce132b991732c6cda9bcc5b8a"),
                Address::new("0x27a97c9aaf04f18f3014c32e036dd0ac76da5f18"),
                Address::new("0xce412f988377e31f4d0ff12d74df73b51c42d0ca"),
                Address::new("0x98c1334496614aed49d2e81526d089f7264fed9c"),
            ]; // initial localnet validator list

            let parse_get_validators_result =
                ValidatorControl::parse_get_validators_result(&client, &validator_list_bytes)
                    .unwrap();

            assert_eq!(expected_validator_list, parse_get_validators_result);
        }
    }
}
