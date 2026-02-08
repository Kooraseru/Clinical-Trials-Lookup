/// Google Sheets export functionality
use crate::contact::StudyContacts;
use crate::export::ExportOptions;
use anyhow::Result;

pub fn export(contacts: &[StudyContacts], options: &ExportOptions) -> Result<String> {
    // Generate instructions for Google Sheets import
    let mut output = String::new();
    
    output.push_str("=== GOOGLE SHEETS EXPORT ===\n\n");
    output.push_str("To import this data into Google Sheets:\n\n");
    output.push_str("1. Create a new Google Sheet\n");
    output.push_str("2. Go to File > Import\n");
    output.push_str("3. Choose 'Upload' tab\n");
    output.push_str("4. Upload the CSV file generated below\n");
    output.push_str("5. Select 'Replace current sheet' or 'Insert new sheet'\n");
    output.push_str("6. Import location: Choose your preference\n\n");
    
    output.push_str("--- OR USING GOOGLE SHEETS API ---\n\n");
    output.push_str("If Google Sheets API is configured:\n");
    output.push_str("1. Set up OAuth2 credentials from Google Cloud Console\n");
    output.push_str("2. Save credentials as 'client_secret.json'\n");
    output.push_str("3. Run with --google-sheets flag\n\n");
    
    #[cfg(feature = "google-sheets")]
    {
        output.push_str("Google Sheets API is ENABLED\n");
        output.push_str("Use --spreadsheet-id to directly upload to a sheet\n\n");
    }
    
    #[cfg(not(feature = "google-sheets"))]
    {
        output.push_str("Google Sheets API is NOT ENABLED\n");
        output.push_str("Rebuild with --features google-sheets to enable direct upload\n\n");
    }
    
    output.push_str("=== CSV DATA FOR MANUAL IMPORT ===\n\n");
    
    // Generate CSV data
    let csv_data = super::csv_export::export(contacts, options)?;
    output.push_str(&csv_data);
    
    output.push_str("\n=== GOOGLE SHEETS FORMULA EXAMPLES ===\n\n");
    output.push_str("Count unique emails:\n");
    output.push_str("  =COUNTA(UNIQUE(E2:E))\n\n");
    output.push_str("Filter by status:\n");
    output.push_str("  =FILTER(A2:H, F2:F=\"Central Contact\")\n\n");
    output.push_str("Group by institution:\n");
    output.push_str("  =QUERY(A2:H, \"SELECT G, COUNT(E) GROUP BY G\")\n\n");
    
    Ok(output)
}
