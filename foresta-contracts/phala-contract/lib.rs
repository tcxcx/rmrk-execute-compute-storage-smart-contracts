#![cfg_attr(not(feature = "std"), no_std, no_main)]

extern crate alloc;

pub mod error;
pub mod utils;

pub use scale::{
    Decode,
    Encode,
};

// pink_extension is short for Phala Ink! extension
use crate::alloc::string::ToString;
use pink::{
    chain_extension::signing::derive_sr25519_key,
    PinkEnvironment,
};
use pink_extension as pink;

use ink::{
    env::{
        call::{
            build_call,
            ExecutionInput,
            Selector,
        },
        DefaultEnvironment,
    },
    storage::Mapping,
};
use openbrush::contracts::psp34::Id;

#[pink::contract(env = PinkEnvironment)]
mod schrodinger {
    use super::*;
    use alloc::{
        format,
        string::String,
        vec::Vec,
    };
    use pink::{
        http_get,
        http_post,
    };

    use crate::error::PhalaError;
    use utils::utils::{
        hash_message,
        is_nft_owner,
    };

    use aes_gcm_siv::{
        aead::{
            Aead,
            KeyInit,
            Nonce,
        },
        Aes256GcmSiv,
    };
    use cipher::{
        consts::{
            U12,
            U32,
        },
        generic_array::GenericArray,
    };
    use ink_storage::Mapping;

    const SIGNATURE_VALID_TIME_IN_MS: u64 = 5 * 60 * 1000;

    pub type CustomResult<T> = Result<T, PhalaError>;

    type NftId = u8;
    type Cid = String;

    #[ink(storage)]
    pub struct SchrodingerContract {
        private_key: Vec<u8>,
        salt: Vec<u8>,
        cid_map: Mapping<NftId, Cid>,
        owner: AccountId,
        owner_restriction: bool,
        contract_id: String,
        rpc_api: String,
        ipfs_endpoint: String,
        database_endpoint: String,
        algo_nft_contract: AccountId,
    }

    impl SchrodingerContract {
        #[ink(constructor)]
        pub fn new(
            contract_id: String,
            rpc_api: String,
            ipfs_endpoint: String,
            database_endpoint: String,
            owner_restriction: bool,
            algo_nft_contract: AccountId,
        ) -> Self {
            // Default constructor
            let salt = b"981781668367".to_vec();
            let private_key = derive_sr25519_key(&salt);
            let owner = Self::env().caller();
            let cid_map = Mapping::default();

            Self {
                private_key,
                salt,
                cid_map,
                owner,
                contract_id,
                owner_restriction,
                rpc_api,
                ipfs_endpoint,
                database_endpoint,
                algo_nft_contract,
            }
        }

        #[ink(message)]
        pub fn set_cid(&mut self, nft_id: u8, cid: String) -> CustomResult<String> {
            if !self.caller_is_contract_owner() {
                return Err(PhalaError::NoPermission);
            }
            self.cid_map.insert(nft_id, &cid);

            Ok(String::from("Done"))
        }

        #[ink(message)]
        pub fn set_cid_with_nft(
            &mut self,
            nft_id: u8,
            cid: String,
            unix_timestamp: u64,
            signature: String,
        ) -> CustomResult<String> {
            let hashed_message = Self::check_timestamp_and_generate_message(unix_timestamp)?;

            if !is_nft_owner(
                signature,
                hashed_message,
                nft_id,
                self.contract_id.to_string(),
                self.rpc_api.to_string(),
            ) {
                return Err(PhalaError::NotNftOwner);
            }

            if self.owner_restriction && !self.caller_is_contract_owner() {
                return Err(PhalaError::NoPermission);
            }

            self.cid_map.insert(nft_id, &cid);

            Ok(String::from("Done"))
        }

        #[ink(message)]
        pub fn get_cid(&self, nft_id: u8) -> CustomResult<String> {
            let cid = self.cid_map.get(nft_id);
            if cid.is_none() {
                return Err(PhalaError::CidMissingFordNftId);
            }
            Ok(format!("{}", cid.unwrap()))
        }

        #[ink(message)]
        pub fn set_owner(&mut self, new_owner: AccountId) -> CustomResult<String> {
            if !self.caller_is_contract_owner() {
                return Err(PhalaError::NoPermission);
            }
            self.owner = new_owner;

            Ok(String::from("Done"))
        }

        #[ink(message)]
        pub fn encrypt_content(&self, content: String) -> CustomResult<String> {
            let key: &GenericArray<u8, U32> = GenericArray::from_slice(&self.private_key[..32]);
            let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&self.salt);

            // Encrypt payload
            let cipher = Aes256GcmSiv::new(key.into());
            let encrypted_content: Vec<u8> =
                cipher.encrypt(nonce, content.as_bytes().as_ref()).unwrap();

            Ok(format!("{}", hex::encode(&encrypted_content)))
        }

        #[ink(message)]
        pub fn deposit_content(
            &self,
            nft_id: u8,
            unix_timestamp: u64,
            signature: String,
        ) -> CustomResult<String> {
            let hashed_message = match Self::check_timestamp_and_generate_message(unix_timestamp) {
                Ok(value) => value,
                Err(value) => return Err(value),
            };
            if !is_nft_owner(
                signature,
                hashed_message,
                nft_id,
                self.contract_id.to_string(),
                self.rpc_api.to_string(),
            ) {
                return Err(PhalaError::NotNftOwner);
            }

            let cid = self.cid_map.get(nft_id);
            if cid.is_none() {
                return Err(PhalaError::CidMissingFordNftId);
            }

            let encrypted_content = self.download_encrypted_content(cid.unwrap())?;
            let decrypted_content = self.decrypt_content(encrypted_content)?;

            let deposit_result = self.deposit_to_database(nft_id, decrypted_content)?;

            Ok(deposit_result)
        }

        fn download_encrypted_content(&self, cid: String) -> CustomResult<String> {
            let response = http_get!(format!("{}/{}", self.ipfs_endpoint.to_string(), cid));
            if response.status_code != 200 {
                return Err(PhalaError::DownloadError);
            }
            let encrypted_content = match String::from_utf8(response.body) {
                Ok(value) => value,
                Err(e) => return Err(PhalaError::DecryptionError),
            };
            Ok(encrypted_content)
        }

        fn deposit_to_database(
            &self,
            nft_id: u8,
            decrypted_content: String,
        ) -> CustomResult<String> {
            let payload = format!(
                "{{\"nft_id\":\"{}\",\"content\":\"{}\"}}",
                nft_id, decrypted_content
            );
            let response = http_post!(self.database_endpoint.to_string(), payload);
            if response.status_code != 200 {
                return Err(PhalaError::DatabaseError);
            }
            let deposit_result = match String::from_utf8(response.body) {
                Ok(value) => value,
                Err(e) => return Err(PhalaError::DatabaseError),
            };
            Ok(deposit_result)
        }

        fn decrypt_content(&self, encrypted_content: String) -> CustomResult<String> {
            let content_decoded =
                hex::decode(encrypted_content).map_err(|_| PhalaError::DecryptionError)?;

            let key: &GenericArray<u8, U32> = GenericArray::from_slice(&self.private_key[..32]);
            let cipher = Aes256GcmSiv::new(key.into());
            let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&self.salt);

            let decrypted_content = cipher
                .decrypt(nonce, content_decoded.as_ref())
                .map_err(|_| PhalaError::DecryptionError)?;

            Ok(String::from_utf8(decrypted_content).map_err(|_| PhalaError::DecryptionError)?)
        }

        #[ink(message)]
        pub fn decrypt_and_execute(&self, algo_id: Id) -> Result<(), PhalaError> {
            let selector = ink::selector_bytes!("fetch_algorithm_data");
            let cid_result: Result<String, PhalaError> = build_call::<DefaultEnvironment>()
                .call(self.algo_nft_contract)
                .exec_input(ExecutionInput::new(Selector::new(selector)).push_arg(algo_id.clone()))
                .returns::<String>()
                .try_invoke()
                .map_err(|_| PhalaError::CrossContractCallFailed)
                .and_then(|res| res.map_err(|_| PhalaError::CrossContractCallFailed));

            let cid = cid_result?;
            self.decrypt_and_execute_inner(algo_id, cid)
        }

        fn decrypt_and_execute_inner(&self, algo_id: Id, cid: String) -> Result<(), PhalaError> {
            let encrypted_content = self.download_encrypted_content(cid)?;
            let decrypted_content = self.decrypt_content(encrypted_content)?;

            let nft_id = match algo_id {
                Id::U8(id) => id,
                _ => return Err(PhalaError::InvalidAlgoId),
            };
            self.deposit_to_database(nft_id, decrypted_content)?;

            Ok(())
        }

        // HELPERS
        fn caller_is_contract_owner(&mut self) -> bool {
            let owner = String::from(format!("{:?}", &self.owner));
            let caller = String::from(format!("{:?}", Self::env().caller()));

            return owner == caller;
        }

        // Check that signature/timestamp was generated before block timestamp and it should be at most 5 minutes old.
        // If Timestamp is valid returned hashed message consisting of hardcoded message and timestamp.
        fn check_timestamp_and_generate_message(
            unix_timestamp: u64,
        ) -> Result<[u8; 32], PhalaError> {
            let block_timestamp = Self::env().block_timestamp();
            if unix_timestamp > block_timestamp
                || block_timestamp.abs_diff(unix_timestamp) >= SIGNATURE_VALID_TIME_IN_MS
            {
                return Err(PhalaError::BadTimestamp);
            }
            let timestamped_message = format!("APILLON_REQUEST_MSG: {}", unix_timestamp);
            let hashed_message = hash_message(&timestamped_message.to_string());
            Ok(hashed_message)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use pink_extension::chain_extension::{
            mock::mock_http_request,
            HttpResponse,
        };
        const TEST_CONTRACT_ADDRESS: &str = "51e044373c4ba5a3d6eef0f7f7502b3d2f60276f";
        const TEST_RPC_API: &str = "https://rpc.api.moonbeam.network/";
        const TEST_IPFS_ENDPOINT: &str = "https://ipfs.apillon.io/ipfs/";
        const TEST_NFT_ID: u8 = 1;
        const TEST_CID: &str = "QmZJTqJzHFt2kSDVWGWUXcgomDSBby1sTtiJcs3LXjXNnC";
        const TEST_DECRYPTED_CONTENT: &str = "test_string";
        const TEST_ENCRYPTED_CONTENT: &str =
            "53bfb3715cb5c28a6949d36d0e551a2434d10ad5415aaf783786d0";
        const TEST_MESSAGE_SIGNATURE: &str = "30d121c70f1f79d8b3212e3cdd24de3bf1a16fc5c3d14880fb80e5299897b4466ec10ac81893d0713ff2bf14feab30f3b8226a6e0b5eb2bec739d512815d4b2a1c";
        const TEST_SIGNATURE_TIMESTAMP: u64 = 1701688728000;
        const TEST_DB_ENDPOINT: &str = "http:localhost:8000";

        pub mod mock_ext {
            pub struct HttpResponse {
                pub status_code: u16,
                pub body: Vec<u8>,
            }
        }

        // TEST HELPERS
        fn test_accounts() -> ink::env::test::DefaultAccounts<PinkEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_block_timestamp(timestamp: u64) {
            ink::env::test::set_block_timestamp::<Environment>(timestamp);
        }

        fn set_caller(caller: AccountId) {
            ink::env::test::set_caller::<Environment>(caller);
        }

        fn default_headers() -> Vec<(String, String)> {
            vec![("Content-Type".to_string(), "application/json".to_string())]
        }
        fn get_contract(restrict_to_owner: bool, _database_endpoint: &str) -> SchrodingerContract {
            pink_extension_runtime::mock_ext::mock_all_ext();
            let algo_nft_contract = AccountId::from([0x01; 32]);
            SchrodingerContract::new(
                TEST_CONTRACT_ADDRESS.to_string(),
                TEST_RPC_API.to_string(),
                TEST_IPFS_ENDPOINT.to_string(),
                _database_endpoint.to_string(),
                restrict_to_owner,
                algo_nft_contract,
            )
        }

        fn setup() -> SchrodingerContract {
            // Construct your contract here, possibly setting up initial state as needed.
            let contract = SchrodingerContract::new(
                TEST_CONTRACT_ADDRESS.to_string(),
                TEST_RPC_API.to_string(),
                TEST_IPFS_ENDPOINT.to_string(),
                TEST_DB_ENDPOINT.to_string(),
                true,
                AccountId::from([0x01; 32]),
            );
            set_block_timestamp(1701688728000);
            contract
        }

        // TESTS
        // GET SET CID TESTS
        #[ink::test]
        fn new_creates_contract_correctly() {
            let contract = get_contract(true, "https://example.com/database");

            assert_eq!(contract.contract_id, TEST_CONTRACT_ADDRESS);
            assert_eq!(contract.rpc_api, TEST_RPC_API);
            assert_eq!(contract.ipfs_endpoint, TEST_IPFS_ENDPOINT);
            assert_eq!(contract.owner_restriction, true);
        }

        #[ink::test]
        fn contract_owner_can_set_and_get_cid() {
            let mut contract = get_contract(true, "https://example.com/database");

            let result = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());

            assert_eq!(result.unwrap(), "Done");
            assert_eq!(contract.get_cid(TEST_NFT_ID).unwrap(), TEST_CID);
        }

        #[ink::test]
        fn get_cid_works_for_all_users() {
            let mut contract = get_contract(true, "https://example.com/database");
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);

            assert_eq!(contract.get_cid(TEST_NFT_ID).unwrap(), TEST_CID.to_string());
        }

        #[ink::test]
        fn get_cid_fails_if_cid_not_set_for_nft_id() {
            let contract = get_contract(true, "https://example.com/database");

            assert_eq!(contract.get_cid(2), Err(PhalaError::CidMissingFordNftId));
        }

        #[ink::test]
        fn non_contract_owner_cant_set_cid() {
            let mut contract = get_contract(true, "https://example.com/database");
            let accounts = test_accounts();
            _ = contract.set_owner(accounts.alice);
            set_caller(accounts.bob);

            assert_eq!(
                contract.set_cid(TEST_NFT_ID, TEST_CID.to_string()),
                Err(PhalaError::NoPermission)
            );
        }

        #[ink::test]
        fn contract_owner_not_owning_nft_cant_set_cid_with_nft() {
            let mut contract = get_contract(false, "https://example.com/database");
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            set_caller(test_accounts().bob);

            let result = contract.set_cid_with_nft(
                2,
                TEST_CID.to_string(),
                TEST_SIGNATURE_TIMESTAMP,
                "invalid_signature".to_string(),
            );

            assert_eq!(result, Err(PhalaError::NotNftOwner));
            assert_eq!(contract.get_cid(2), Err(PhalaError::CidMissingFordNftId));
        }

        #[ink::test]
        fn nft_owner_can_set_cid_with_nft() {
            let mut contract = get_contract(false, "https://example.com/database");
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let result = contract.set_cid_with_nft(
                TEST_NFT_ID,
                TEST_CID.to_string(),
                TEST_SIGNATURE_TIMESTAMP,
                TEST_MESSAGE_SIGNATURE.to_string(),
            );

            assert_eq!(result.unwrap(), "Done");
            assert_eq!(contract.get_cid(TEST_NFT_ID).unwrap(), TEST_CID);
        }

        // SET OWNER TESTS
        #[ink::test]
        fn contract_owner_can_set_new_contract_owner() {
            let mut contract = get_contract(true, "https://example.com/database");

            assert_eq!(contract.set_owner(test_accounts().alice).unwrap(), "Done");
        }

        #[ink::test]
        fn non_contract_owner_cant_set_new_contract_owner() {
            let mut contract = get_contract(true, "https://example.com/database");
            let accounts = test_accounts();
            set_caller(accounts.bob);

            assert_eq!(
                contract.set_owner(accounts.alice),
                Err(PhalaError::NoPermission)
            );
        }

        // ENCRYPT CONTENT TESTS
        #[ink::test]
        fn anyone_can_encrypt_content() {
            let contract = get_contract(true, "https://example.com/database");
            set_caller(test_accounts().bob);

            let result = contract.encrypt_content(TEST_DECRYPTED_CONTENT.to_string());

            assert_eq!(result.unwrap(), TEST_ENCRYPTED_CONTENT);
        }

        // DOWNLOAD ENCRYPTED CONTENT TESTS
        #[ink::test]
        fn download_encrypted_content_succeeds_with_valid_cid() {
            let mut contract = get_contract(true, "https://example.com/database");
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());

            let result = contract.download_encrypted_content(TEST_CID.to_string());

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), TEST_ENCRYPTED_CONTENT);
        }

        #[ink::test]
        fn download_encrypted_content_fails_with_invalid_cid() {
            let contract = get_contract(true, "https://example.com/database");
            let invalid_cid = "invalid_cid";

            mock_http_request(|_| {
                HttpResponse {
                    status_code: 404,
                    body: "Not Found".as_bytes().to_vec(),
                    headers: default_headers(),
                    reason_phrase: "Not Found".to_string(),
                }
            });

            let result = contract.download_encrypted_content(invalid_cid.to_string());

            assert!(result.is_err());
        }
        // DECRYPT CONTENT TESTS
        #[ink::test]
        fn decrypt_content_succeeds_with_valid_encrypted_content() {
            let contract = get_contract(true, "https://example.com/database");

            let result = contract.decrypt_content(TEST_ENCRYPTED_CONTENT.to_string());

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), TEST_DECRYPTED_CONTENT);
        }

        #[ink::test]
        fn decrypt_content_fails_with_invalid_encrypted_content() {
            let contract = get_contract(true, "https://example.com/database");
            let invalid_encrypted_content = "invalid_encrypted_content";

            let result = contract.decrypt_content(invalid_encrypted_content.to_string());

            assert!(result.is_err());
        }

        // DEPOSIT TO DATABASE TESTS
        #[ink::test]
        fn deposit_to_database_succeeds_with_valid_data() {
            let contract = get_contract(true, "https://example.com/database");

            mock_http_request(|_| {
                HttpResponse {
                    status_code: 200,
                    body: "Deposit successful".as_bytes().to_vec(),
                    headers: default_headers(),
                    reason_phrase: "OK".to_string(),
                }
            });

            let result =
                contract.deposit_to_database(TEST_NFT_ID, TEST_DECRYPTED_CONTENT.to_string());

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Deposit successful");
        }

        #[ink::test]
        fn deposit_to_database_fails_with_invalid_endpoint() {
            let contract = get_contract(true, "https://invalid.endpoint");

            mock_http_request(|_| {
                HttpResponse {
                    status_code: 404,
                    body: "Not Found".as_bytes().to_vec(),
                    headers: default_headers(),
                    reason_phrase: "Not Found".to_string(),
                }
            });

            let result =
                contract.deposit_to_database(TEST_NFT_ID, TEST_DECRYPTED_CONTENT.to_string());

            assert!(result.is_err());
        }

        // DEPOSIT CONTENT TESTS
        #[ink::test]
        fn deposit_content_succeeds_with_valid_data() {
            let mut contract = get_contract(true, "https://example.com/database");
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            mock_http_request(|_| {
                HttpResponse {
                    status_code: 200,
                    body: TEST_ENCRYPTED_CONTENT.as_bytes().to_vec(),
                    headers: default_headers(),
                    reason_phrase: "OK".to_string(),
                }
            });

            let result = contract.deposit_content(
                TEST_NFT_ID,
                TEST_SIGNATURE_TIMESTAMP,
                TEST_MESSAGE_SIGNATURE.to_string(),
            );

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Deposit successful");
        }

        #[ink::test]
        fn deposit_content_fails_with_expired_signature_timestamp() {
            let mut contract = get_contract(true, "https://example.com/database");
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let expired_timestamp = TEST_SIGNATURE_TIMESTAMP + SIGNATURE_VALID_TIME_IN_MS;
            let result = contract.deposit_content(
                TEST_NFT_ID,
                expired_timestamp,
                TEST_MESSAGE_SIGNATURE.to_string(),
            );

            assert!(result.is_err());
        }

        #[ink::test]
        fn decrypt_and_execute_works() {
            let mut contract = setup();

            let expected_cid = "QmExampleCid".to_string();
            let algo_id = TEST_NFT_ID;

            // Directly insert the expected_cid into the contract's map for testing
            contract.cid_map.insert(algo_id, &expected_cid);

            // Mock the HTTP requests
            mock_http_request(|_| {
                HttpResponse {
                    status_code: 200,
                    body: "53bfb3715cb5c28a6949d36d0e551a2434d10ad5415aaf783786d0"
                        .as_bytes()
                        .to_vec(),
                    headers: default_headers(),
                    reason_phrase: "OK".to_string(),
                }
            });

            let result = contract.decrypt_and_execute(Id::U8(algo_id));
            assert!(result.is_ok());
        }
    }
}
