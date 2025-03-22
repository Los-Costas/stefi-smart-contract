use core::fmt;
use soroban_sdk::contracterror;

#[derive(Debug, Copy, Clone, PartialEq)]
#[contracterror]
pub enum ContractError {
    AdminNotFound = 1,
    AssetAlreadyCreated = 2,
    TotalCannotBeZero = 3,
    AssetNotFound = 4,
    SignerHaveInsufficientBalance = 5,
    SignerIsNotAssetProvider = 6,
    TimeLimitHasExpired = 7,
    AssetWithoutPurchaseOption = 8,
    GracePeriodHasBeenReached = 9,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::AdminNotFound => write!(f, "Admin not found"),
            ContractError::AssetAlreadyCreated => write!(f, "Asset already created"),
            ContractError::TotalCannotBeZero => write!(f, "Total cannot be zero"),
            ContractError::AssetNotFound => write!(f, "Asset not found"),
            ContractError::SignerHaveInsufficientBalance => write!(f, "Signer have insufficient balance"),
            ContractError::SignerIsNotAssetProvider => write!(f, "Signer is not asset provider"),
            ContractError::TimeLimitHasExpired => write!(f, "The payment deadline has been reached"),
            ContractError::AssetWithoutPurchaseOption => write!(f, "Asset without purchase option"),
            ContractError::GracePeriodHasBeenReached => write!(f, "The grace period to make your payment has been reached."),
        }
    }
}