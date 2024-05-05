extern crate alloc;
use ink::env::DefaultEnvironment;
pub use scale::{Decode, Encode};
use openbrush::traits::String;
use algo_nft::algo_nft::algo_nft::AlgorithmNFT;
use execute_nft::execute_nft::execute_nft::ExecutionNFT;

#[ink::contract(env = DefaultEnvironment)]
pub mod rmrk_proxy {
    use super::*;
    pub type Id = u64;

    #[derive(Encode, Decode, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAuthorized,
        NotFound,
        StorageError,
        UnexpectedError,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct RmrkProxy {
        algo_nft: AlgorithmNFT,
        execute_nft: ExecutionNFT,
        owner: AccountId,
        schrodinger_contract: AccountId,
    }

    impl RmrkProxy {
        #[ink(constructor)]
        pub fn new(
            algo_nft_address: AccountId,
            execute_nft_address: AccountId,
            schrodinger_contract_address: AccountId,
        ) -> Self {
            let caller = Self::env().caller();
            Self {
                algo_nft: AlgorithmNFT::at(algo_nft_address),
                execute_nft: ExecutionNFT::at(execute_nft_address),
                owner: caller,
                schrodinger_contract: schrodinger_contract_address,
            }
        }

        #[ink(message)]
        pub fn request_execution(
            &mut self,
            token_id: u64,
            user: AccountId,
        ) -> Result<()> { 
            // Check if the user has the right to execute the algorithm
            if !self.execute_nft.is_owner(Id::u64(token_id), user) {
                return Err(Error::NotAuthorized);
            }

            // Get the CID from AlgorithmNFT
            let cid = self
                .algo_nft
                .fetch_algorithm_data(Id::u64(token_id))
                .map_err(|_| Error::NotFound)?;

            // Call the SchrodingerContract for execution
            let schrodinger_contract = SchrodingerContract::at(self.schrodinger_contract);
            schrodinger_contract.execute_algorithm(cid).map_err(|_| Error::UnexpectedError)?;

            Ok(())
        }
    }
}