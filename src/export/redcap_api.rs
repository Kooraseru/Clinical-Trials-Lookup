/// REDCap API integration for direct data upload
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::collections::HashMap;

/// REDCap API configuration
#[derive(Debug, Clone)]
pub struct RedcapConfig {
    /// REDCap API URL (e.g., https://redcap.vumc.org/api/)
    pub api_url: String,
    /// API token for authentication
    pub api_token: String,
}

/// REDCap API upload response details
#[derive(Debug)]
pub struct UploadResult {
    pub success: bool,
    pub records_imported: usize,
    pub ids: Vec<String>,
    pub response_code: u16,
    pub message: String,
}

/// Upload CSV data to REDCap via API
pub fn upload_to_redcap(csv_data: &str, config: &RedcapConfig) -> Result<UploadResult> {
    let client = Client::new();
    
    println!("\n{}", "=== REDCap API Upload ===".to_string());
    println!("API URL: {}", config.api_url);
    println!("Token: {}...{}", &config.api_token[..4], &config.api_token[config.api_token.len()-4..]);
    
    // Count records being uploaded
    let record_count = csv_data.lines().filter(|line| !line.trim().is_empty()).count().saturating_sub(1); // -1 for header
    println!("CSV Data: {} bytes, {} records", csv_data.len(), record_count);
    
    // Build form parameters matching the Java example format exactly
    let mut params = HashMap::new();
    params.insert("token", config.api_token.as_str());
    params.insert("content", "record");
    params.insert("action", "import");
    params.insert("format", "csv");
    params.insert("type", "flat");
    params.insert("overwriteBehavior", "normal");
    params.insert("forceAutoNumber", "false");
    params.insert("data", csv_data);
    params.insert("returnContent", "ids");  // Changed from "count" to "ids" to match user's format
    params.insert("returnFormat", "json");
    
    println!("\nSending POST request...");
    
    // Make POST request
    let response = client
        .post(&config.api_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .context("Failed to send request to REDCap API")?;
    
    let status = response.status();
    let status_code = status.as_u16();
    let response_text = response.text()
        .context("Failed to read response from REDCap API")?;
    
    println!("Response Code: {}", status_code);
    println!("Response Body: {}", response_text);
    
    if !status.is_success() {
        return Ok(UploadResult {
            success: false,
            records_imported: 0,
            ids: vec![],
            response_code: status_code,
            message: format!("HTTP {} - {}", status_code, response_text),
        });
    }
    
    // Parse JSON response to extract IDs
    let ids: Vec<String> = if let Ok(parsed) = serde_json::from_str::<Vec<String>>(&response_text) {
        parsed
    } else {
        // Fallback: if it's not a JSON array, treat as single value
        vec![response_text.clone()]
    };
    
    Ok(UploadResult {
        success: true,
        records_imported: ids.len(),
        ids,
        response_code: status_code,
        message: response_text,
    })
}

/// Extract just the CSV portion from REDCap export format
pub fn extract_csv_from_redcap_export(full_export: &str) -> String {
    // The redcap export includes instructions and data dictionary
    // We need to extract just the CSV portion
    
    // Look for the CSV header line
    if let Some(start_idx) = full_export.find("record_id,nct_id,study_title") {
        // Find where the CSV data ends (before the import steps section)
        if let Some(end_idx) = full_export[start_idx..].find("\n\n=== IMPORT STEPS ===") {
            return full_export[start_idx..start_idx + end_idx].to_string();
        }
        // If no end marker, take everything after the header
        return full_export[start_idx..].to_string();
    }
    
    // Fallback: return the full export if we can't parse it
    full_export.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_csv() {
        let export = r#"=== REDCAP DATA IMPORT ===

Some instructions...

=== REDCAP-FORMATTED CSV ===

record_id,nct_id,study_title,contact_name
1,NCT123,Test Study,Dr. Smith

=== IMPORT STEPS ===

1. Upload the file
"#;
        
        let csv = extract_csv_from_redcap_export(export);
        assert!(csv.contains("record_id"));
        assert!(csv.contains("Dr. Smith"));
        assert!(!csv.contains("IMPORT STEPS"));
    }
}
