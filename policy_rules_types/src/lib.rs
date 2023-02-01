use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, env, near_bindgen, PanicOnDefault};
use crate::policy::{AllPolicies, init_policies};

pub mod policy;
pub mod types;
pub mod utils;
pub mod prices;
mod tests;
