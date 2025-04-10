#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

use rating::{rate_seller_system, update_weighted_rating};
use reputation::{add_reputation_score_history, reputation_score_calculate};

mod datatypes;
mod history;
mod rating;
mod reputation;
mod test;

/// RatingSystemContract is a Soroban smart contract that implements a decentralized rating and reputation system.
/// It allows buyers to rate sellers and maintains a weighted rating system along with reputation scores.

#[contract]
pub struct RatingSystemContract;

/// DataKey enum defines the different types of data stored in the contract's storage.
/// Each variant is associated with an Address to maintain separate data for each seller.

#[contracttype] 
pub enum DataKey {
    /// Stores the complete rating history for a seller
    RatingHistory(Address),
    /// Stores the weighted rating data for a seller
    WeightedRating(Address),
    /// Stores the reputation score history for a seller
    ReputationHistory(Address),
}

#[contractimpl]
impl RatingSystemContract {

    /// Allows a buyer to rate a seller with a score between 1 and 5.
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `seller` - The address of the seller being rated
    /// * `buyer` - The address of the buyer providing the rating
    /// * `rating` - The rating score (1-5)
    /// * `weight` - The weight of the rating (higher weights have more impact)
    /// * `feedback` - Optional feedback text from the buyer
    /// 
    /// # Panics
    /// * If buyer and seller are the same address
    /// * If rating is not between 1 and 5

    pub fn rate_seller(
        env: Env,
        seller: Address,
        buyer: Address,
        rating: u32,
        weight: u32,
        feedback: Option<String>,
    ) {
        // Prevent self-rating
        if buyer == seller {
            panic!("Buyer and seller cannot be the same address");
        }

        // Ensure rating is between 1 and 5
        if !(1..=5).contains(&rating) {
            panic!("Rating must be between 1 and 5");
        }
        // Process seller's rating
        if rate_seller_system(
            env.clone(),
            seller.clone(),
            buyer.clone(),
            rating,
            weight,
            feedback,
        ) {
            // update seller's weighted rating data
            update_weighted_rating(env.clone(), seller.clone(), rating, weight);
        }

        env.events().publish(
            (Symbol::new(&env, "buyer_rate_seller"), seller.clone()),
            rating,
        );
    }

    /// Calculates and updates a seller's reputation score based on their rating history.
    /// The reputation score is a value between 1 and 5 that reflects the seller's overall performance.
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `seller` - The address of the seller whose reputation is being calculated
    /// 
    /// # Returns
    /// * `u32` - The calculated reputation score (1-5)
    /// 
    /// # Panics
    /// * If the seller address is invalid
    
    pub fn seller_reputation_score(env: Env, seller: Address) -> u32 {
        // Validate seller address
        if seller.to_string().is_empty() || seller.to_string().len() == 0 {
            panic!("Seller address is invalid");
        }

        // calculate seller reputation score
        let reputation_score = reputation_score_calculate(env.clone(), seller.clone());
        add_reputation_score_history(env.clone(), seller.clone(), reputation_score);

        env.events().publish(
            (Symbol::new(&env, "reputation_score_updated"), seller),
            reputation_score,
        );
        reputation_score
    }
}
