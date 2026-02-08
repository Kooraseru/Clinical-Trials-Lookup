/// CSV export functionality
use crate::contact::StudyContacts;
use crate::export::ExportOptions;
use anyhow::Result;

pub fn export(contacts: &[StudyContacts], options: &ExportOptions) -> Result<String> {
    let mut csv = String::new();
    
    if options.include_headers {
        csv.push_str("NCT ID,Study Title,Contact Name,Role,Email,Source,Affiliation,Phone\n");
    }
    
    for study in contacts {
        let contacts_to_export = if options.individual_emails_only {
            study.individual_emails()
        } else {
            study.contacts.iter().collect()
        };
        
        for contact in contacts_to_export {
            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                study.nct_id,
                escape_csv(&study.study_title),
                escape_csv(&contact.name),
                escape_csv(&contact.role),
                contact.email,
                contact.source,
                escape_csv(&contact.affiliation.as_deref().unwrap_or("")),
                contact.phone.as_deref().unwrap_or("")
            ));
        }
    }
    
    Ok(csv)
}

fn escape_csv(s: &str) -> String {
    s.replace('"', "\"\"")
}
