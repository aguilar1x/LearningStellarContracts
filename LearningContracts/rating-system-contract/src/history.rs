use crate::DataKey;
use crate::{datatypes::Error, rating::Rating, reputation::ReputationRecord};
use soroban_sdk::{Address, Env, Vec};

/// Retrieves the complete rating history for a given seller.
/// This function provides access to all ratings ever received by a seller.
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `seller` - The address of the seller
/// 
/// # Returns
/// * `Result<Vec<Rating>, Error>` - The rating history or an error if not found

pub fn _get_rating_history(env: Env, seller: &Address) -> Result<Vec<Rating>, Error> {
    let key = DataKey::RatingHistory(seller.clone());
    match env.storage().instance().get(&key) {
        Some(ratings) => Ok(ratings),
        None => Err(Error::RatingHistoryNotFound),
    }
}

/// Retrieves the reputation score history for a given seller.
/// This function provides access to all reputation scores ever calculated for a seller.
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `seller` - The address of the seller
/// 
/// # Returns
/// * `Result<Vec<ReputationRecord>, Error>` - The reputation history or an error if not found

pub fn get_reputation_history(env: Env, seller: Address) -> Result<Vec<ReputationRecord>, Error> {
    let key = DataKey::ReputationHistory(seller.clone());
    match env.storage().instance().get(&key) {
        Some(reputation_record) => Ok(reputation_record),
        None => Err(Error::ReputationHistoryNotFound),
    }
}
