/// Export functionality for contact data
pub mod csv_export;
pub mod google_sheets;
pub mod redcap;

use crate::contact::StudyContacts;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Csv,
    GoogleSheets,
    Redcap,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub format: ExportFormat,
    pub output_path: Option<String>,
    pub include_headers: bool,
    pub individual_emails_only: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::Csv,
            output_path: None,
            include_headers: true,
            individual_emails_only: false,
        }
    }
}

/// Main export function that routes to appropriate exporter
pub fn export_contacts(
    contacts: &[StudyContacts],
    options: &ExportOptions,
) -> Result<String> {
    match options.format {
        ExportFormat::Csv => csv_export::export(contacts, options),
        ExportFormat::GoogleSheets => google_sheets::export(contacts, options),
        ExportFormat::Redcap => redcap::export(contacts, options),
        ExportFormat::Json => export_json(contacts, options),
    }
}

fn export_json(contacts: &[StudyContacts], _options: &ExportOptions) -> Result<String> {
    Ok(serde_json::to_string_pretty(contacts)?)
}
