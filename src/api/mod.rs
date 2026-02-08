/// API client and functionality for ClinicalTrials.gov
pub mod client;
pub mod models;
pub mod search;

pub use client::ClinicalTrialsClient;
pub use search::*;
