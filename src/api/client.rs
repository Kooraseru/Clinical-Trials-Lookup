use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;
use colored::Colorize;

use super::models::{StudiesResponse, Study};

const API_BASE_URL: &str = "https://clinicaltrials.gov/api/v2";

/// Client for interacting with ClinicalTrials.gov API
pub struct ClinicalTrialsClient {
    client: Client,
    base_url: String,
}

impl ClinicalTrialsClient {
    /// Create a new API client
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("clinical-trials-lookup/0.1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: API_BASE_URL.to_string(),
        })
    }

    /// Get API version information
    pub fn get_version(&self) -> Result<Value> {
        let url = format!("{}/version", self.base_url);
        let response = self.client.get(&url).send()
            .context("Failed to get API version")?;
        
        response.json()
            .context("Failed to parse version response")
    }

    /// Search for studies with query parameters
    pub fn search_studies(&self, params: &HashMap<String, String>) -> Result<StudiesResponse> {
        let url = format!("{}/studies", self.base_url);
        
        let response = self.client
            .get(&url)
            .query(params)
            .send()
            .context("Failed to send search request")?;

        let status = response.status();
        
        // Check for rate limiting
        if status.as_u16() == 429 {
            let error_text = response.text().unwrap_or_default();
            eprintln!("\n{}", "⚠️  RATE LIMIT EXCEEDED".bright_yellow().bold());
            eprintln!("The ClinicalTrials.gov API has rate limited your requests.");
            eprintln!("Please wait a few minutes before trying again.");
            anyhow::bail!("API rate limit exceeded (429): {}", error_text);
        }
        
        if !status.is_success() {
            let error_text = response.text().unwrap_or_default();
            anyhow::bail!("API request failed with status {}: {}", status, error_text);
        }

        response.json()
            .context("Failed to parse studies response")
    }

    /// Get a specific study by NCT ID
    pub fn get_study(&self, nct_id: &str) -> Result<Study> {
        let url = format!("{}/studies/{}", self.base_url, nct_id);
        
        let response = self.client
            .get(&url)
            .send()
            .context(format!("Failed to get study {}", nct_id))?;

        let status = response.status();
        
        // Check for rate limiting
        if status.as_u16() == 429 {
            let error_text = response.text().unwrap_or_default();
            eprintln!("\n{}", "⚠️  RATE LIMIT EXCEEDED".bright_yellow().bold());
            eprintln!("The ClinicalTrials.gov API has rate limited your requests.");
            eprintln!("Please wait a few minutes before trying again.");
            anyhow::bail!("API rate limit exceeded (429): {}", error_text);
        }
        
        if !status.is_success() {
            let error_text = response.text().unwrap_or_default();
            anyhow::bail!("API request failed with status {}: {}", status, error_text);
        }

        response.json()
            .context("Failed to parse study response")
    }

    /// Get all studies matching query with pagination
    #[allow(dead_code)]
    pub fn search_all_studies(&self, params: &HashMap<String, String>) -> Result<Vec<Study>> {
        let mut all_studies = Vec::new();
        let mut current_params = params.clone();
        
        loop {
            let response = self.search_studies(&current_params)?;
            all_studies.extend(response.studies);
            
            // Check if there are more pages
            match response.next_page_token {
                Some(token) if !token.is_empty() => {
                    current_params.insert("pageToken".to_string(), token);
                }
                _ => break,
            }
        }
        
        Ok(all_studies)
    }

    /// Get field metadata
    #[allow(dead_code)]
    pub fn get_metadata(&self) -> Result<Value> {
        let url = format!("{}/studies/metadata", self.base_url);
        let response = self.client.get(&url).send()
            .context("Failed to get metadata")?;
        
        response.json()
            .context("Failed to parse metadata response")
    }

    /// Get searchable areas
    #[allow(dead_code)]
    pub fn get_search_areas(&self) -> Result<Value> {
        let url = format!("{}/studies/search-areas", self.base_url);
        let response = self.client.get(&url).send()
            .context("Failed to get search areas")?;
        
        response.json()
            .context("Failed to parse search areas response")
    }

    /// Get statistics about database size
    pub fn get_stats_size(&self) -> Result<Value> {
        let url = format!("{}/stats/size", self.base_url);
        let response = self.client.get(&url).send()
            .context("Failed to get stats")?;
        
        response.json()
            .context("Failed to parse stats response")
    }
}

impl Default for ClinicalTrialsClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ClinicalTrialsClient::new();
        assert!(client.is_ok());
    }
}
