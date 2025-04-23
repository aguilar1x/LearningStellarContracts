use crate::DataKey;
use crate::{datatypes::Error, rating::Rating, reputation::ReputationRecord};
use soroban_sdk::{Address, Env, Vec};

pub fn _get_rating_history(env: Env, seller: &Address) -> Result<Vec<Rating>, Error> {
    let key = DataKey::RatingHistory(seller.clone());
    match env.storage().instance().get(&key) {
        Some(ratings) => Ok(ratings),
        None => Err(Error::RatingHistoryNotFound),
    }
}

pub fn get_reputation_history(env: Env, seller: &Address) -> Result<Vec<ReputationRecord>, Error> {
    let key = DataKey::ReputationHistory(seller.clone());
    match env.storage().instance().get(&key) {
        Some(reputation_record) => Ok(reputation_record),
        None => Err(Error::ReputationHistoryNotFound),
    }
}

