/// Contact extraction and management for clinical trials
use crate::api::models::Study;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Extracted contact information with role and email
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExtractedContact {
    pub name: String,
    pub role: String,
    pub email: String,
    pub source: ContactSource,
    pub affiliation: Option<String>,
    pub phone: Option<String>,
}

/// Source of the contact information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ContactSource {
    CentralContact,
    OverallOfficial,
    LocationContact { facility: String, location: String },
    PointOfContact,
    ResponsibleParty,
}

impl std::fmt::Display for ContactSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactSource::CentralContact => write!(f, "Central Contact"),
            ContactSource::OverallOfficial => write!(f, "Overall Official"),
            ContactSource::LocationContact { facility, location } => {
                write!(f, "Location Contact ({}, {})", facility, location)
            }
            ContactSource::PointOfContact => write!(f, "Point of Contact"),
            ContactSource::ResponsibleParty => write!(f, "Responsible Party"),
        }
    }
}

/// Study contacts extracted from a clinical trial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyContacts {
    pub nct_id: String,
    pub study_title: String,
    pub contacts: Vec<ExtractedContact>,
}

impl StudyContacts {
    /// Get contacts with valid individual emails
    pub fn individual_emails(&self) -> Vec<&ExtractedContact> {
        self.contacts
            .iter()
            .filter(|c| is_likely_individual_email(&c.email))
            .collect()
    }

    /// Get contacts with institutional emails
    #[allow(dead_code)]
    pub fn institutional_emails(&self) -> Vec<&ExtractedContact> {
        self.contacts
            .iter()
            .filter(|c| !is_likely_individual_email(&c.email))
            .collect()
    }

    /// Get all unique emails
    #[allow(dead_code)]
    pub fn unique_emails(&self) -> HashSet<String> {
        self.contacts.iter().map(|c| c.email.clone()).collect()
    }
}

/// Extract all contacts from a study
pub fn extract_contacts(study: &Study) -> StudyContacts {
    let mut contacts = Vec::new();
    let protocol = &study.protocol_section;
    
    let nct_id = protocol.identification_module.nct_id.clone();
    let study_title = protocol
        .identification_module
        .brief_title
        .clone()
        .or_else(|| protocol.identification_module.official_title.clone())
        .unwrap_or_else(|| "Untitled Study".to_string());

    // Extract central contacts
    if let Some(contacts_module) = &protocol.contacts_locations_module {
        if let Some(central_contacts) = &contacts_module.central_contacts {
            for contact in central_contacts {
                if let (Some(name), Some(email)) = (&contact.name, &contact.email) {
                    if !email.is_empty() {
                        contacts.push(ExtractedContact {
                            name: name.clone(),
                            role: contact.role.clone().unwrap_or_else(|| "Contact".to_string()),
                            email: email.clone(),
                            source: ContactSource::CentralContact,
                            affiliation: None,
                            phone: contact.phone.clone(),
                        });
                    }
                }
            }
        }

        // Extract overall officials
        if let Some(officials) = &contacts_module.overall_officials {
            for official in officials {
                if let Some(name) = &official.name {
                    // Note: Overall officials typically don't have email in the API
                    // but we include them for completeness
                    contacts.push(ExtractedContact {
                        name: name.clone(),
                        role: official.role.clone().unwrap_or_else(|| "Official".to_string()),
                        email: String::new(), // Usually not available
                        source: ContactSource::OverallOfficial,
                        affiliation: official.affiliation.clone(),
                        phone: None,
                    });
                }
            }
        }

        // Extract location contacts
        if let Some(locations) = &contacts_module.locations {
            for location in locations {
                let facility = location.facility.clone().unwrap_or_else(|| "Unknown Facility".to_string());
                let location_str = format!(
                    "{}, {}",
                    location.city.clone().unwrap_or_default(),
                    location.country.clone().unwrap_or_default()
                );

                if let Some(location_contacts) = &location.contacts {
                    for contact in location_contacts {
                        if let (Some(name), Some(email)) = (&contact.name, &contact.email) {
                            if !email.is_empty() {
                                contacts.push(ExtractedContact {
                                    name: name.clone(),
                                    role: contact.role.clone().unwrap_or_else(|| "Contact".to_string()),
                                    email: email.clone(),
                                    source: ContactSource::LocationContact {
                                        facility: facility.clone(),
                                        location: location_str.clone(),
                                    },
                                    affiliation: Some(facility.clone()),
                                    phone: contact.phone.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Extract responsible party information
    if let Some(sponsor_module) = &protocol.sponsor_collaborators_module {
        if let Some(responsible_party) = &sponsor_module.responsible_party {
            if let Some(name) = &responsible_party.investigator_full_name {
                contacts.push(ExtractedContact {
                    name: name.clone(),
                    role: responsible_party
                        .investigator_title
                        .clone()
                        .unwrap_or_else(|| "Responsible Party".to_string()),
                    email: String::new(), // Usually not available
                    source: ContactSource::ResponsibleParty,
                    affiliation: responsible_party.investigator_affiliation.clone(),
                    phone: None,
                });
            }
        }
    }

    // Extract point of contact from results section
    if let Some(results) = &study.results_section {
        if let Some(more_info) = &results.more_info_module {
            if let Some(poc) = &more_info.point_of_contact {
                if let (Some(title), Some(email)) = (&poc.title, &poc.email) {
                    if !email.is_empty() {
                        contacts.push(ExtractedContact {
                            name: title.clone(),
                            role: "Point of Contact".to_string(),
                            email: email.clone(),
                            source: ContactSource::PointOfContact,
                            affiliation: poc.organization.clone(),
                            phone: poc.phone.clone(),
                        });
                    }
                }
            }
        }
    }

    StudyContacts {
        nct_id,
        study_title,
        contacts: contacts.into_iter().filter(|c| !c.email.is_empty()).collect(),
    }
}

/// Check if an email is likely an individual email (not generic/institutional)
fn is_likely_individual_email(email: &str) -> bool {
    let email_lower = email.to_lowercase();
    
    // Generic/institutional patterns
    let generic_patterns = [
        "info@", "contact@", "admin@", "support@",
        "office@", "research@", "clinical@", "trials@",
        "study@", "recruitment@", "coordinator@",
    ];

    // Check if it matches any generic pattern
    !generic_patterns.iter().any(|pattern| email_lower.starts_with(pattern))
}

/// Export contacts to CSV format
#[allow(dead_code)]
pub fn export_to_csv(studies: &[StudyContacts]) -> Result<String, Box<dyn std::error::Error>> {
    let mut csv = String::from("NCT ID,Study Title,Name,Role,Email,Source,Affiliation,Phone\n");
    
    for study in studies {
        for contact in &study.contacts {
            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                study.nct_id,
                study.study_title.replace('"', "\"\""),
                contact.name.replace('"', "\"\""),
                contact.role.replace('"', "\"\""),
                contact.email,
                contact.source,
                contact.affiliation.as_deref().unwrap_or("").replace('"', "\"\""),
                contact.phone.as_deref().unwrap_or("")
            ));
        }
    }
    
    Ok(csv)
}

/// Print contacts in a human-readable format
pub fn print_contacts(study_contacts: &StudyContacts, show_all: bool) {
    use colored::Colorize;

    println!("\n{}", "=".repeat(80).bright_blue());
    println!("{}: {}", "Study".bright_cyan().bold(), study_contacts.study_title);
    println!("{}: {}", "NCT ID".bright_cyan().bold(), study_contacts.nct_id);
    println!("{}", "=".repeat(80).bright_blue());

    if study_contacts.contacts.is_empty() {
        println!("{}", "No contact information available".yellow());
        return;
    }

    // Prioritize individual emails unless showing all
    let individual: Vec<_> = if !show_all {
        study_contacts.individual_emails()
    } else {
        Vec::new()
    };

    let contacts_to_show: Vec<&ExtractedContact> = if show_all {
        study_contacts.contacts.iter().collect()
    } else if !individual.is_empty() {
        println!("\n{}", "Individual Contacts (Preferred):".green().bold());
        individual
    } else {
        println!("\n{}", "Institutional Contacts:".yellow().bold());
        study_contacts.contacts.iter().collect()
    };

    for (i, contact) in contacts_to_show.iter().enumerate() {
        println!("\n{}. {}", i + 1, contact.name.bright_white().bold());
        println!("   {}: {}", "Role".cyan(), contact.role);
        println!("   {}: {}", "Email".green(), contact.email);
        println!("   {}: {}", "Source".blue(), contact.source);
        
        if let Some(aff) = &contact.affiliation {
            println!("   {}: {}", "Affiliation".magenta(), aff);
        }
        
        if let Some(phone) = &contact.phone {
            println!("   {}: {}", "Phone".yellow(), phone);
        }
    }

    println!();
}
