use crate::DataKey;
use soroban_sdk::{contracttype, Address, Env, String, Symbol, Vec};

/// Represents a single rating entry in the system.
/// Contains all information about a buyer's rating of a seller.

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Rating {
    /// The address of the buyer who provided the rating
    pub buyer: Address,
    /// The numerical rating value (1-5)
    pub rating: u32,
    /// The weight of this rating (higher weights have more impact on the final score)
    pub weight: u32,
    /// Optional feedback text from the buyer
    pub feedback: Option<String>,
}

/// Minimum allowed rating value
pub const MIN_RATING: u32 = 1;
/// Maximum allowed rating value
pub const MAX_RATING: u32 = 5;

/// Processes a new rating submission for a seller.
/// This function handles the core rating logic and storage.
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `seller` - The address of the seller being rated
/// * `buyer` - The address of the buyer providing the rating
/// * `rating` - The rating value (1-5)
/// * `weight` - The weight of the rating
/// * `feedback` - Optional feedback text
/// 
/// # Returns
/// * `bool` - True if the rating was successfully processed
/// 
/// # Panics
/// * If the rating is not within the valid range (1-5)

pub fn rate_seller_system(
    env: Env,
    seller: Address,
    buyer: Address,
    rating: u32,
    weight: u32,
    feedback: Option<String>,
) -> bool {
    // Validate rating range
    if !(MIN_RATING..=MAX_RATING).contains(&rating) {
        panic!("rating value must be in range 1 to 5");
    }

    // Create a new rating record
    let seller_rating = Rating {
        buyer,
        rating,
        weight,
        feedback,
    };

    let key = DataKey::RatingHistory(seller.clone());

    // fetch existing ratings or initialize new vector
    let mut ratings: Vec<Rating> = match env.storage().instance().get(&key) {
        Some(x) => x,
        None => Vec::new(&env),
    };

    // Add new rating data in vector
    ratings.push_back(seller_rating);

    // Update seller ratings in storage
    env.storage().instance().set(&key, &ratings);

    env.events().publish(
        (Symbol::new(&env, "buyer_rate_the_seller"), seller.clone()),
        rating,
    );

    true
}

/// Updates the weighted rating statistics for a seller.
/// This function maintains running totals of weighted ratings and total weights.
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `seller` - The address of the seller
/// * `rating` - The rating value to add
/// * `weight` - The weight of the rating

pub fn update_weighted_rating(env: Env, seller: Address, rating: u32, weight: u32) {
    // Fetch existing weighted rating and total weight or initialize to zero
    let key = DataKey::WeightedRating(seller.clone());

    let (mut total_weighted_rating, mut total_weight): (u32, u32) =
        match env.storage().instance().get(&key) {
            Some((x, y)) => (x, y),
            None => (0, 0),
        };

    // Update total weighted rating and weight
    total_weighted_rating += rating * weight;
    total_weight += weight;

    // save updated values in storage
    env.storage()
        .instance()
        .set(&key, &(total_weighted_rating, total_weight));

    env.events().publish(
        (Symbol::new(&env, "updated_weighted_rating"), seller.clone()),
        weight,
    );
}

/// Calculates the weighted average rating for a seller.
/// This is used as the basis for determining the seller's reputation score.
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `seller` - The address of the seller
/// 
/// # Returns
/// * `f32` - The calculated weighted average rating
/// 
/// # Panics
/// * If no ratings are available for the seller

pub fn calculate_weighted_rating(env: Env, seller: Address) -> f32 {
    let key = DataKey::WeightedRating(seller.clone());
    // Fetch existing total weighted rating and total weight or initialize to zero
    let (total_weighted_rating, total_weight): (u32, u32) = match env.storage().instance().get(&key)
    {
        Some((x, y)) => (x, y),
        None => (0, 0),
    };

    // ensure there are ratings to calculate
    if total_weight == 0 {
        panic!("No rating available");
    }

    // Compute weighted rating
    let weighted_rating = (total_weighted_rating / total_weight) as f32;

    env.events().publish(
        (
            Symbol::new(&env, "calculated_weighted_rating"),
            seller.clone(),
        ),
        weighted_rating as u32,
    );

    weighted_rating
}
