use soroban_sdk::{contracttype, Address, Env, Symbol, Vec};

use crate::{history, DataKey};

/// Represents a single reputation score record in the system.
/// Each record contains the score and the timestamp when it was calculated.

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct ReputationRecord {
    /// The numerical reputation score (1-5)
    pub score: u32,
    /// The timestamp when the reputation score was calculated
    pub timestamp: u64,
}


/// Calculates the reputation score for a seller based on their weighted rating.
/// The reputation score is a value between 1 and 5 that reflects the seller's overall performance.
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `seller` - The address of the seller
/// 
/// # Returns
/// * `u32` - The calculated reputation score (1-5)

pub fn reputation_score_calculate(env: Env, seller: Address) -> u32 {
    // fetch the weighted rating for the seller
    let x = crate::rating::calculate_weighted_rating(env, seller);

    // Determine the reputation score based on the rating range
    match x {
        x if x <= 1.0 => 1,
        x if x <= 2.0 => 2,
        x if x <= 3.0 => 3,
        x if x <= 4.0 => 4,
        _ => 5,
    }
}

/// Adds a new reputation score record to the seller's history.
/// This function maintains a chronological history of reputation scores.
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `seller` - The address of the seller
/// * `score` - The reputation score to record

pub fn add_reputation_score_history(env: Env, seller: Address, score: u32) {
    // Get current ledger timestamp
    let timestamp = env.ledger().timestamp();

    // Retrieve existing reputation history or initialize a new vector
    let mut reputation_history: Vec<ReputationRecord> =
        match history::get_reputation_history(env.clone(), seller.clone()) {
            Ok(history) => history,
            Err(_) => Vec::new(&env),
        };

    // Create a new reputation record
    let new_record = ReputationRecord { score, timestamp };

    // Add the new record to the history
    reputation_history.push_back(new_record.clone());

    // Update the seller's reputation history in storage
    let key = DataKey::ReputationHistory(seller.clone());
    env.storage().instance().set(&key, &reputation_history);

    env.events().publish(
        (Symbol::new(&env, "added_score_in_history"), seller.clone()),
        new_record,
    );
}
