/// Search query builder for ClinicalTrials.gov API
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueryBuilder {
    params: HashMap<String, String>,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Set a query string for full-text search
    /// Example: "cancer AND (lung OR breast)"
    pub fn query(mut self, query: &str) -> Self {
        self.params.insert("query.term".to_string(), query.to_string());
        self
    }

    /// Search by condition/disease
    /// Example: "diabetes", "breast cancer"
    pub fn condition(mut self, condition: &str) -> Self {
        self.params.insert("query.cond".to_string(), condition.to_string());
        self
    }

    /// Search by intervention/treatment
    /// Example: "aspirin", "radiation therapy"
    pub fn intervention(mut self, intervention: &str) -> Self {
        self.params.insert("query.intr".to_string(), intervention.to_string());
        self
    }

    /// Search by location (city, state, country)
    /// Example: "Boston, MA", "United States"
    pub fn location(mut self, location: &str) -> Self {
        self.params.insert("query.locn".to_string(), location.to_string());
        self
    }

    /// Search by study title
    pub fn title(mut self, title: &str) -> Self {
        self.params.insert("query.titles".to_string(), title.to_string());
        self
    }

    /// Search by organization/sponsor
    pub fn organization(mut self, org: &str) -> Self {
        self.params.insert("query.lead".to_string(), org.to_string());
        self
    }

    /// Search by principal investigator name
    pub fn investigator(mut self, name: &str) -> Self {
        self.params.insert("query.ov".to_string(), name.to_string());
        self
    }

    /// Filter by overall recruitment status
    /// Values: "RECRUITING", "COMPLETED", "ACTIVE_NOT_RECRUITING", etc.
    pub fn status(mut self, status: &str) -> Self {
        self.params.insert("filter.overallStatus".to_string(), status.to_string());
        self
    }

    /// Filter by study type
    /// Values: "INTERVENTIONAL", "OBSERVATIONAL", "EXPANDED_ACCESS"
    pub fn study_type(mut self, study_type: &str) -> Self {
        self.params.insert("filter.studyType".to_string(), study_type.to_string());
        self
    }

    /// Filter by start date (from)
    /// Format: YYYY-MM-DD
    pub fn start_date_from(mut self, date: &str) -> Self {
        let filter = format!("AREA[StartDate]RANGE[{}, MAX]", date);
        self.add_advanced_filter(&filter);
        self
    }

    /// Filter by start date (to)
    /// Format: YYYY-MM-DD
    pub fn start_date_to(mut self, date: &str) -> Self {
        let filter = format!("AREA[StartDate]RANGE[MIN, {}]", date);
        self.add_advanced_filter(&filter);
        self
    }

    /// Filter by start date range
    /// Format: YYYY-MM-DD
    pub fn start_date_range(mut self, from: &str, to: &str) -> Self {
        let filter = format!("AREA[StartDate]RANGE[{}, {}]", from, to);
        self.add_advanced_filter(&filter);
        self
    }

    /// Filter by primary completion date (from)
    /// Format: YYYY-MM-DD
    pub fn completion_date_from(mut self, date: &str) -> Self {
        let filter = format!("AREA[PrimaryCompletionDate]RANGE[{}, MAX]", date);
        self.add_advanced_filter(&filter);
        self
    }

    /// Filter by primary completion date (to)
    /// Format: YYYY-MM-DD
    pub fn completion_date_to(mut self, date: &str) -> Self {
        let filter = format!("AREA[PrimaryCompletionDate]RANGE[MIN, {}]", date);
        self.add_advanced_filter(&filter);
        self
    }

    /// Filter by completion date range
    /// Format: YYYY-MM-DD
    pub fn completion_date_range(mut self, from: &str, to: &str) -> Self {
        let filter = format!("AREA[PrimaryCompletionDate]RANGE[{}, {}]", from, to);
        self.add_advanced_filter(&filter);
        self
    }

    /// Helper to add advanced filters (can combine multiple)
    fn add_advanced_filter(&mut self, filter: &str) {
        if let Some(existing) = self.params.get("filter.advanced") {
            self.params.insert("filter.advanced".to_string(), 
                format!("{} AND {}", existing, filter));
        } else {
            self.params.insert("filter.advanced".to_string(), filter.to_string());
        }
    }

    /// Set the number of results per page (max 1000)
    pub fn page_size(mut self, size: usize) -> Self {
        self.params.insert("pageSize".to_string(), size.to_string());
        self
    }

    /// Set the page token for pagination
    pub fn page_token(mut self, token: &str) -> Self {
        self.params.insert("pageToken".to_string(), token.to_string());
        self
    }

    /// Set fields to return in the response
    /// Example: "NCTId,BriefTitle,Condition"
    pub fn fields(mut self, fields: &str) -> Self {
        self.params.insert("fields".to_string(), fields.to_string());
        self
    }

    /// Advanced query with custom parameters
    pub fn custom_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    /// Build the query parameters
    pub fn build(&self) -> HashMap<String, String> {
        self.params.clone()
    }

    /// Build query string for URL
    pub fn to_query_string(&self) -> String {
        self.params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&")
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined search templates
pub struct SearchTemplates;

impl SearchTemplates {
    /// Search for actively recruiting studies by condition
    pub fn recruiting_by_condition(condition: &str) -> QueryBuilder {
        QueryBuilder::new()
            .condition(condition)
            .status("RECRUITING")
            .page_size(100)
    }

    /// Search for studies by sponsor/organization
    pub fn by_sponsor(sponsor: &str) -> QueryBuilder {
        QueryBuilder::new()
            .organization(sponsor)
            .page_size(100)
    }

    /// Search for studies by location
    pub fn by_location(location: &str) -> QueryBuilder {
        QueryBuilder::new()
            .location(location)
            .page_size(100)
    }

    /// Search for interventional drug trials
    pub fn drug_trials(drug_name: &str) -> QueryBuilder {
        QueryBuilder::new()
            .intervention(drug_name)
            .study_type("INTERVENTIONAL")
            .page_size(100)
    }

    /// Get studies with contact information
    pub fn with_contacts() -> QueryBuilder {
        QueryBuilder::new()
            .status("RECRUITING")
            .page_size(100)
    }
}
