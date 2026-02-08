use serde::{Deserialize, Serialize};

/// Root response structure for studies query
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StudiesResponse {
    pub studies: Vec<Study>,
    pub next_page_token: Option<String>,
    pub total_count: Option<usize>,
}

/// Individual study record
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Study {
    pub protocol_section: ProtocolSection,
    #[serde(default)]
    pub results_section: Option<ResultsSection>,
    #[serde(default)]
    pub derived_section: Option<DerivedSection>,
}

/// Protocol section containing study details
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSection {
    pub identification_module: IdentificationModule,
    #[serde(default)]
    pub contacts_locations_module: Option<ContactsLocationsModule>,
    #[serde(default)]
    pub sponsor_collaborators_module: Option<SponsorCollaboratorsModule>,
    #[serde(default)]
    pub status_module: Option<StatusModule>,
    #[serde(default)]
    pub description_module: Option<DescriptionModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationModule {
    pub nct_id: String,
    #[serde(default)]
    pub brief_title: Option<String>,
    #[serde(default)]
    pub official_title: Option<String>,
    #[serde(default)]
    pub organization: Option<Organization>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub full_name: String,
    #[serde(default)]
    pub class: Option<String>,
}

/// Contacts and locations module
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContactsLocationsModule {
    #[serde(default)]
    pub central_contacts: Option<Vec<Contact>>,
    #[serde(default)]
    pub overall_officials: Option<Vec<Official>>,
    #[serde(default)]
    pub locations: Option<Vec<Location>>,
}

/// Contact information
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub phone_ext: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

/// Overall official information
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Official {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub affiliation: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
}

/// Location/facility information
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(default)]
    pub facility: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub zip: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub contacts: Option<Vec<Contact>>,
}

/// Sponsor and collaborators information
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SponsorCollaboratorsModule {
    #[serde(default)]
    pub lead_sponsor: Option<Sponsor>,
    #[serde(default)]
    pub collaborators: Option<Vec<Sponsor>>,
    #[serde(default)]
    pub responsible_party: Option<ResponsibleParty>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub name: String,
    #[serde(default)]
    pub class: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponsibleParty {
    #[serde(default)]
    pub investigator_full_name: Option<String>,
    #[serde(default)]
    pub investigator_title: Option<String>,
    #[serde(default)]
    pub investigator_affiliation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusModule {
    #[serde(default)]
    pub overall_status: Option<String>,
    #[serde(default)]
    pub status_verified_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionModule {
    #[serde(default)]
    pub brief_summary: Option<String>,
    #[serde(default)]
    pub detailed_description: Option<String>,
}

/// Results section for studies with posted results
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultsSection {
    #[serde(default)]
    pub more_info_module: Option<MoreInfoModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoreInfoModule {
    #[serde(default)]
    pub point_of_contact: Option<PointOfContact>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PointOfContact {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub organization: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub phone_ext: Option<String>,
}

/// Derived section with additional computed data
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DerivedSection {
    // Can be extended with derived fields as needed
}
