/// Batch processing and table-based parameter input
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Search parameters for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    pub name: Option<String>,
    pub condition: Option<String>,
    pub intervention: Option<String>,
    pub location: Option<String>,
    pub sponsor: Option<String>,
    pub status: Option<String>,
    pub study_type: Option<String>,
    pub start_date_from: Option<String>,
    pub start_date_to: Option<String>,
    pub completion_date_from: Option<String>,
    pub completion_date_to: Option<String>,
    pub min_results: Option<usize>,
    pub max_results: Option<usize>,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            name: None,
            condition: None,
            intervention: None,
            location: None,
            sponsor: None,
            status: None,
            study_type: None,
            start_date_from: None,
            start_date_to: None,
            completion_date_from: None,
            completion_date_to: None,
            min_results: None,
            max_results: Some(100),
        }
    }
}

/// Batch of multiple search queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub searches: Vec<SearchParams>,
    pub export_format: String,
    pub output_file: Option<String>,
    pub individual_emails_only: bool,
}

/// Load batch configuration from CSV file
pub fn load_from_csv(path: &Path) -> Result<BatchConfig> {
    let content = fs::read_to_string(path)
        .context("Failed to read CSV file")?;
    
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut searches = Vec::new();
    
    for result in rdr.deserialize() {
        let params: SearchParams = result
            .context("Failed to parse CSV row")?;
        searches.push(params);
    }
    
    Ok(BatchConfig {
        searches,
        export_format: "csv".to_string(),
        output_file: None,
        individual_emails_only: false,
    })
}

/// Load batch configuration from JSON file
pub fn load_from_json(path: &Path) -> Result<BatchConfig> {
    let content = fs::read_to_string(path)
        .context("Failed to read JSON file")?;
    
    serde_json::from_str(&content)
        .context("Failed to parse JSON")
}

/// Generate a template CSV for batch processing
pub fn generate_template_csv() -> String {
    let mut csv = String::new();
    csv.push_str("name,condition,intervention,location,sponsor,status,study_type,");
    csv.push_str("start_date_from,start_date_to,completion_date_from,completion_date_to,");
    csv.push_str("min_results,max_results\n");
    
    // Example row
    csv.push_str("Diabetes Studies,diabetes,,,,,RECRUITING,");
    csv.push_str("2020-01-01,2025-12-31,,,10,100\n");
    
    // Example row 2
    csv.push_str("Cancer Trials CA,cancer,,California,,RECRUITING,INTERVENTIONAL,");
    csv.push_str("2023-01-01,,,,20,50\n");
    
    csv
}

/// Generate a template JSON for batch processing
pub fn generate_template_json() -> String {
    let template = BatchConfig {
        searches: vec![
            SearchParams {
                name: Some("Diabetes Studies".to_string()),
                condition: Some("diabetes".to_string()),
                status: Some("RECRUITING".to_string()),
                max_results: Some(100),
                ..Default::default()
            },
            SearchParams {
                name: Some("Cancer Trials in California".to_string()),
                condition: Some("cancer".to_string()),
                location: Some("California".to_string()),
                status: Some("RECRUITING".to_string()),
                study_type: Some("INTERVENTIONAL".to_string()),
                start_date_from: Some("2023-01-01".to_string()),
                max_results: Some(50),
                ..Default::default()
            },
        ],
        export_format: "csv".to_string(),
        output_file: Some("contacts_export.csv".to_string()),
        individual_emails_only: true,
    };
    
    serde_json::to_string_pretty(&template).unwrap()
}
