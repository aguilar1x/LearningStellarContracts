use soroban_sdk::contracterror;

/// Custom error types for the Rating System Contract.
/// These errors are used to handle specific failure cases in the contract.
/// 
/// The errors are represented as a u32 to be compatible with Soroban's error handling system.
/// Each error has a unique numeric code for identification.

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {

    /// Error code 1: Indicates that a requested reputation history record was not found in storage.
    /// This typically occurs when trying to access reputation data for a seller who has no history.

    ReputationHistoryNotFound = 1,
    
    /// Error code 2: Indicates that a requested rating history record was not found in storage.
    /// This typically occurs when trying to access rating data for a seller who has no ratings.
    
    RatingHistoryNotFound = 2,
}
