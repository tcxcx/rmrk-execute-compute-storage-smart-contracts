
use scale::{Decode, Encode};
use openbrush::contracts::ownable::OwnableError;

#[derive(Encode, Decode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AlgoExecuteError {
    NotAuthorized,
    NotFound,
    UnexpectedError,
    InvalidExecutionNFT,
    UnauthorizedAccess,
    DependencyError(String),
}

impl From<OwnableError> for AlgoExecuteError {
    fn from(_err: OwnableError) -> Self {
        AlgoExecuteError::NotAuthorized
    }
}