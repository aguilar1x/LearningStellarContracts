#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

use rating::{rate_seller_system, update_weighted_rating};
use reputation::{add_reputation_score_history, reputation_score_calculate};

mod datatypes;
mod history;
mod rating;
mod reputation;
mod test;

#[contract]
pub struct RatingSystemContract;

#[contracttype]
pub enum DataKey {
    RatingHistory(Address),
    WeightedRating(Address),
    ReputationHistory(Address),
}

#[contractimpl]
impl RatingSystemContract {

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

    pub fn seller_reputation_score(env: Env, seller: Address) -> u32 {
        // Validate seller address
        if seller.to_string().is_empty() {
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
