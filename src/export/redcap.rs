/// REDCap export functionality
use crate::contact::StudyContacts;
use crate::export::ExportOptions;
use anyhow::Result;

pub fn export(contacts: &[StudyContacts], options: &ExportOptions) -> Result<String> {
    let mut output = String::new();
    
    output.push_str("=== REDCAP DATA IMPORT ===\n\n");
    output.push_str("This export generates REDCap-compatible CSV format.\n\n");
    
    output.push_str("SETUP INSTRUCTIONS:\n");
    output.push_str("1. Create a new REDCap project or use existing\n");
    output.push_str("2. Set up the following fields in your data dictionary:\n\n");
    
    output.push_str("RECOMMENDED REDCAP DATA DICTIONARY:\n");
    output.push_str("-----------------------------------\n");
    output.push_str("Variable Name      | Field Label              | Field Type | Validation\n");
    output.push_str("record_id          | Record ID                | text       | \n");
    output.push_str("nct_id             | NCT ID                   | text       | \n");
    output.push_str("study_title        | Study Title              | text       | \n");
    output.push_str("contact_name       | Contact Name             | text       | \n");
    output.push_str("contact_role       | Role                     | text       | \n");
    output.push_str("contact_email      | Email                    | text       | email\n");
    output.push_str("contact_source     | Contact Source           | text       | \n");
    output.push_str("contact_affiliation| Affiliation              | text       | \n");
    output.push_str("contact_phone      | Phone                    | text       | phone\n");
    output.push_str("email_sent         | Survey Sent              | radio      | 0, No | 1, Yes\n");
    output.push_str("email_opened       | Survey Opened            | radio      | 0, No | 1, Yes\n");
    output.push_str("survey_completed   | Survey Completed         | radio      | 0, No | 1, Yes\n");
    output.push_str("response_date      | Response Date            | text       | date_mdy\n");
    output.push_str("notes              | Notes                    | notes      | \n\n");
    
    output.push_str("=== REDCAP-FORMATTED CSV ===\n\n");
    
    // Generate REDCap CSV with record_id
    if options.include_headers {
        output.push_str("record_id,nct_id,study_title,contact_name,contact_role,contact_email,");
        output.push_str("contact_source,contact_affiliation,contact_phone,email_sent,email_opened,");
        output.push_str("survey_completed,response_date,notes\n");
    }
    
    let mut record_id = 1;
    for study in contacts {
        let contacts_to_export = if options.individual_emails_only {
            study.individual_emails()
        } else {
            study.contacts.iter().collect()
        };
        
        for contact in contacts_to_export {
            output.push_str(&format!(
                "{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",,,,,\n",
                record_id,
                study.nct_id,
                escape_csv(&study.study_title),
                escape_csv(&contact.name),
                escape_csv(&contact.role),
                contact.email,
                contact.source,
                escape_csv(&contact.affiliation.as_deref().unwrap_or("")),
                contact.phone.as_deref().unwrap_or("")
            ));
            record_id += 1;
        }
    }
    
    output.push_str("\n=== IMPORT STEPS ===\n\n");
    output.push_str("1. Go to your REDCap project\n");
    output.push_str("2. Click 'Data Import Tool' in the left menu\n");
    output.push_str("3. Upload the CSV data above\n");
    output.push_str("4. Follow the import wizard\n");
    output.push_str("5. Verify the data after import\n\n");
    
    output.push_str("=== API UPLOAD (Advanced) ===\n\n");
    output.push_str("To upload via REDCap API:\n");
    output.push_str("1. Get your API token from REDCap project settings\n");
    output.push_str("2. Use the following curl command:\n\n");
    output.push_str("curl -X POST https://[your-redcap-url]/api/ \\\n");
    output.push_str("  -d token=[your-api-token] \\\n");
    output.push_str("  -d content=record \\\n");
    output.push_str("  -d format=csv \\\n");
    output.push_str("  -d type=flat \\\n");
    output.push_str("  -d overwriteBehavior=normal \\\n");
    output.push_str("  -d data=\"$(cat contacts.csv)\"\n\n");
    
    Ok(output)
}

fn escape_csv(s: &str) -> String {
    s.replace('"', "\"\"")
}
