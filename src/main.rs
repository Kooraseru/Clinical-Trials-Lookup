mod api;
mod batch;
mod contact;
mod export;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use std::path::PathBuf;

use api::{ClinicalTrialsClient, QueryBuilder, SearchTemplates};
use contact::{extract_contacts, print_contacts};
use export::{ExportFormat, ExportOptions};
use export::redcap_api::{RedcapConfig, upload_to_redcap, extract_csv_from_redcap_export, UploadResult};

#[derive(Parser)]
#[command(name = "Clinical Trials Lookup")]
#[command(about = "Search ClinicalTrials.gov and extract contact information", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search by condition/disease
    Condition {
        /// The condition or disease to search for
        condition: String,
        
        /// Only show recruiting studies
        #[arg(short, long)]
        recruiting: bool,
        
        /// Maximum number of studies to retrieve
        #[arg(short, long, default_value_t = 10)]
        max: usize,
    },
    
    /// Search by intervention/treatment
    Intervention {
        /// The intervention or treatment to search for
        intervention: String,
        
        /// Maximum number of studies to retrieve
        #[arg(short, long, default_value_t = 10)]
        max: usize,
    },
    
    /// Search by location
    Location {
        /// Location to search (city, state, country)
        location: String,
        
        /// Maximum number of studies to retrieve
        #[arg(short, long, default_value_t = 10)]
        max: usize,
    },
    
    /// Search by sponsor/organization
    Sponsor {
        /// Sponsor or organization name
        sponsor: String,
        
        /// Maximum number of studies to retrieve
        #[arg(short, long, default_value_t = 10)]
        max: usize,
    },
    
    /// Get a specific study by NCT ID
    Study {
        /// NCT ID (e.g., NCT12345678)
        nct_id: String,
    },
    
    /// Custom search with multiple parameters
    Custom {
        /// Query string
        #[arg(short, long)]
        query: Option<String>,
        
        /// Condition
        #[arg(short, long)]
        condition: Option<String>,
        
        /// Intervention
        #[arg(short, long)]
        intervention: Option<String>,
        
        /// Location
        #[arg(short, long)]
        location: Option<String>,
        
        /// Status filter
        #[arg(short, long)]
        status: Option<String>,
        
        /// Study type filter
        #[arg(long)]
        study_type: Option<String>,
        
        /// Start date from (YYYY-MM-DD)
        #[arg(long)]
        start_date_from: Option<String>,
        
        /// Start date to (YYYY-MM-DD)
        #[arg(long)]
        start_date_to: Option<String>,
        
        /// Completion date from (YYYY-MM-DD)
        #[arg(long)]
        completion_date_from: Option<String>,
        
        /// Completion date to (YYYY-MM-DD)
        #[arg(long)]
        completion_date_to: Option<String>,
        
        /// Maximum number of studies to retrieve
        #[arg(short, long, default_value_t = 10)]
        max: usize,
        
        /// Export format
        #[arg(short, long, value_enum)]
        format: Option<ExportFormatArg>,
        
        /// Output file path
        #[arg(long)]
        output: Option<PathBuf>,
        
        /// Only export individual emails
        #[arg(long)]
        individual_only: bool,
        
        /// REDCap API URL (e.g., https://redcap.vumc.org/api/)
        #[arg(long)]
        redcap_url: Option<String>,
        
        /// REDCap API token for direct upload
        #[arg(long)]
        redcap_token: Option<String>,
        
        /// Only search for studies in the United States
        #[arg(long)]
        us_only: bool,
    },
    
    /// List all available search parameters
    Parameters,
    
    /// Show API information
    Info,
    
    /// Run batch searches from a config file
    Batch {
        /// Path to batch configuration file (CSV or JSON)
        config_file: PathBuf,
        
        /// Export format
        #[arg(short, long, value_enum, default_value = "csv")]
        format: ExportFormatArg,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Only export individual emails (exclude institutional)
        #[arg(long)]
        individual_only: bool,
        
        /// REDCap API URL (e.g., https://redcap.vumc.org/api/)
        #[arg(long)]
        redcap_url: Option<String>,
        
        /// REDCap API token for direct upload
        #[arg(long)]
        redcap_token: Option<String>,
        
        /// Only search for studies in the United States
        #[arg(long)]
        us_only: bool,
    },
    
    /// Generate a template for batch processing
    Template {
        /// Format for template (csv or json)
        #[arg(value_enum, default_value = "csv")]
        format: TemplateFormat,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Export existing results to different formats
    Export {
        /// Input CSV file with contacts
        input: PathBuf,
        
        /// Export format
        #[arg(short, long, value_enum)]
        format: ExportFormatArg,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Only export individual emails
        #[arg(long)]
        individual_only: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum ExportFormatArg {
    Csv,
    GoogleSheets,
    Redcap,
    Json,
}

impl From<ExportFormatArg> for ExportFormat {
    fn from(arg: ExportFormatArg) -> Self {
        match arg {
            ExportFormatArg::Csv => ExportFormat::Csv,
            ExportFormatArg::GoogleSheets => ExportFormat::GoogleSheets,
            ExportFormatArg::Redcap => ExportFormat::Redcap,
            ExportFormatArg::Json => ExportFormat::Json,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum TemplateFormat {
    Csv,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Create API client
    let client = ClinicalTrialsClient::new()?;
    
    match cli.command {
        Commands::Condition { condition, recruiting, max } => {
            println!("{}", format!("Searching for studies about: {}", condition).bright_green().bold());
            
            let query = if recruiting {
                SearchTemplates::recruiting_by_condition(&condition)
            } else {
                QueryBuilder::new().condition(&condition)
            };
            
            search_and_display(&client, query, max)?;
        }
        
        Commands::Intervention { intervention, max } => {
            println!("{}", format!("Searching for studies with intervention: {}", intervention).bright_green().bold());
            
            let query = QueryBuilder::new().intervention(&intervention);
            search_and_display(&client, query, max)?;
        }
        
        Commands::Location { location, max } => {
            println!("{}", format!("Searching for studies in: {}", location).bright_green().bold());
            
            let query = SearchTemplates::by_location(&location);
            search_and_display(&client, query, max)?;
        }
        
        Commands::Sponsor { sponsor, max } => {
            println!("{}", format!("Searching for studies by sponsor: {}", sponsor).bright_green().bold());
            
            let query = SearchTemplates::by_sponsor(&sponsor);
            search_and_display(&client, query, max)?;
        }
        
        Commands::Study { nct_id } => {
            println!("{}", format!("Fetching study: {}", nct_id).bright_green().bold());
            
            let study = client.get_study(&nct_id)?;
            let contacts = extract_contacts(&study);
            print_contacts(&contacts, true);
        }
        
        Commands::Custom { query, condition, intervention, location, status, study_type,
                          start_date_from, start_date_to, completion_date_from, completion_date_to, max,
                          format, output, individual_only, redcap_url, redcap_token, us_only } => {
            println!("{}", "Performing custom search...".bright_green().bold());
            
            let mut builder = QueryBuilder::new();
            
            if let Some(q) = query {
                builder = builder.query(&q);
            }
            if let Some(c) = condition {
                builder = builder.condition(&c);
            }
            if let Some(i) = intervention {
                builder = builder.intervention(&i);
            }
            if let Some(l) = location {
                builder = builder.location(&l);
            } else if us_only {
                // If no location specified but us_only is true, filter for United States
                builder = builder.location("United States");
                println!("{}", "🇺🇸 Filtering for United States studies only".bright_blue());
            }
            if let Some(s) = status {
                builder = builder.status(&s);
            }
            if let Some(st) = study_type {
                builder = builder.study_type(&st);
            }
            
            // Handle date ranges
            match (start_date_from, start_date_to) {
                (Some(from), Some(to)) => builder = builder.start_date_range(&from, &to),
                (Some(from), None) => builder = builder.start_date_from(&from),
                (None, Some(to)) => builder = builder.start_date_to(&to),
                _ => {}
            }
            
            match (completion_date_from, completion_date_to) {
                (Some(from), Some(to)) => builder = builder.completion_date_range(&from, &to),
                (Some(from), None) => builder = builder.completion_date_from(&from),
                (None, Some(to)) => builder = builder.completion_date_to(&to),
                _ => {}
            }
            
            // If format/output specified, do export instead of display
            if format.is_some() || output.is_some() {
                let query = builder.page_size(max.min(1000));
                let params = query.build();
                
                println!("{}", "Fetching studies...".cyan());
                let response = client.search_studies(&params)?;
                
                println!("{}", format!("Found {} studies", response.studies.len()).green());
                
                let mut all_contacts = Vec::new();
                for study in &response.studies {
                    let contacts = extract_contacts(study);
                    if !contacts.contacts.is_empty() {
                        all_contacts.push(contacts);
                    }
                }
                
                println!("{}", format!("Studies with contacts: {}", all_contacts.len()).green());
                
                let export_format = format.unwrap_or(ExportFormatArg::Csv);
                let export_options = ExportOptions {
                    format: export_format.clone().into(),
                    output_path: output.as_ref().map(|p| p.to_string_lossy().to_string()),
                    include_headers: true,
                    individual_emails_only: individual_only,
                };
                
                let export_data = export::export_contacts(&all_contacts, &export_options)?;
                
                // Handle REDCap API upload
                if let (Some(url), Some(token)) = (redcap_url, redcap_token) {
                    if matches!(export_format, ExportFormatArg::Redcap) {
                        let config = RedcapConfig {
                            api_url: url,
                            api_token: token,
                        };
                        
                        let csv_data = extract_csv_from_redcap_export(&export_data);
                        match upload_to_redcap(&csv_data, &config) {
                            Ok(result) => {
                                if result.success {
                                    println!("\n{}", "✓ UPLOAD SUCCESSFUL!".bright_green().bold());
                                    println!("Records imported: {}", result.records_imported.to_string().bright_green());
                                    println!("Record IDs: {:?}", result.ids);
                                    println!("HTTP Status: {}", result.response_code);
                                } else {
                                    println!("\n{}", "✗ UPLOAD FAILED".red().bold());
                                    println!("HTTP Status: {}", result.response_code);
                                    println!("Message: {}", result.message);
                                    println!("{}", "Saving to file instead...".yellow());
                                }
                            }
                            Err(e) => {
                                println!("\n{}", "✗ UPLOAD ERROR".red().bold());
                                println!("Error: {}", e);
                                println!("{}", "Saving to file instead...".yellow());
                            }
                        }
                    } else {
                        println!("{}", "Warning: REDCap API parameters provided but format is not 'redcap'".yellow());
                    }
                }
                
                if let Some(path) = output {
                    std::fs::write(&path, &export_data)?;
                    println!("\n{}", format!("Export saved to: {}", path.display()).bright_green().bold());
                } else {
                    println!("\n{}", export_data);
                }
            } else {
                search_and_display(&client, builder, max)?;
            }
        }
        
        Commands::Parameters => {
            print_available_parameters();
        }
        
        Commands::Info => {
            show_api_info(&client)?;
        }
        
        Commands::Batch { config_file, format, output, individual_only, redcap_url, redcap_token, us_only } => {
            println!("{}", "Running batch search from configuration file...".bright_green().bold());
            
            let config = if config_file.extension().and_then(|s| s.to_str()) == Some("json") {
                batch::load_from_json(&config_file)?
            } else {
                batch::load_from_csv(&config_file)?
            };
            
            run_batch_searches(&client, config, format.clone().into(), output.clone(), individual_only, redcap_url, redcap_token, us_only)?;
        }
        
        Commands::Template { format, output } => {
            let template = match format {
                TemplateFormat::Csv => batch::generate_template_csv(),
                TemplateFormat::Json => batch::generate_template_json(),
            };
            
            if let Some(path) = output {
                std::fs::write(&path, &template)?;
                println!("{}", format!("Template saved to: {}", path.display()).green());
            } else {
                println!("{}", "=== BATCH PROCESSING TEMPLATE ===".bright_cyan().bold());
                println!("\n{}", template);
            }
        }
        
        Commands::Export { input, format, output: _, individual_only: _ } => {
            println!("{}", "Converting export format...".bright_green().bold());
            
            // For now, print instructions
            // In a full implementation, you'd parse the input CSV and re-export
            println!("{}", format!("Input: {}", input.display()).cyan());
            println!("{}", format!("Format: {:?}", format).cyan());
            println!("{}", "Feature coming soon: Load existing CSV and convert to other formats".yellow());
        }
    }
    
    Ok(())
}

fn run_batch_searches(
    client: &ClinicalTrialsClient,
    config: batch::BatchConfig,
    format: ExportFormat,
    output: Option<PathBuf>,
    individual_only: bool,
    redcap_url: Option<String>,
    redcap_token: Option<String>,
    us_only: bool,
) -> Result<()> {
    let mut all_contacts = Vec::new();
    
    if us_only {
        println!("{}", "🇺🇸 US-only filter enabled for all searches".bright_blue());
    }
    
    for (idx, params) in config.searches.iter().enumerate() {
        let default_name = format!("Search {}", idx + 1);
        let search_name = params.name.as_deref().unwrap_or(&default_name);
        println!("\n{}", format!("Running search: {}", search_name).bright_cyan().bold());
        
        let mut query = QueryBuilder::new();
        
        if let Some(c) = &params.condition {
            query = query.condition(c);
        }
        if let Some(i) = &params.intervention {
            query = query.intervention(i);
        }
        if let Some(l) = &params.location {
            query = query.location(l);
        } else if us_only {
            // Apply US filter if no location specified
            query = query.location("United States");
        }
        if let Some(s) = &params.sponsor {
            query = query.organization(s);
        }
        if let Some(st) = &params.status {
            query = query.status(st);
        }
        if let Some(st) = &params.study_type {
            query = query.study_type(st);
        }
        
        // Add date filtering
        match (&params.start_date_from, &params.start_date_to) {
            (Some(from), Some(to)) => query = query.start_date_range(from, to),
            (Some(from), None) => query = query.start_date_from(from),
            (None, Some(to)) => query = query.start_date_to(to),
            _ => {}
        }
        
        match (&params.completion_date_from, &params.completion_date_to) {
            (Some(from), Some(to)) => query = query.completion_date_range(from, to),
            (Some(from), None) => query = query.completion_date_from(from),
            (None, Some(to)) => query = query.completion_date_to(to),
            _ => {}
        }
        
        let max = params.max_results.unwrap_or(100);
        query = query.page_size(max.min(1000));
        
        let search_params = query.build();
        
        println!("{}", "Fetching studies...".cyan());
        let response = client.search_studies(&search_params)?;
        
        println!("{}", format!("  Found {} studies", response.studies.len()).green());
        
        for study in &response.studies {
            let contacts = extract_contacts(study);
            if !contacts.contacts.is_empty() {
                all_contacts.push(contacts);
            }
        }
    }
    
    println!("\n{}", "=== BATCH RESULTS ===".bright_cyan().bold());
    println!("Total studies with contacts: {}", all_contacts.len().to_string().green());
    
    let total_contacts: usize = all_contacts.iter().map(|s| s.contacts.len()).sum();
    println!("Total contacts found: {}", total_contacts.to_string().green());
    
    // Export
    let export_options = ExportOptions {
        format: format.clone(),
        output_path: output.as_ref().map(|p| p.to_string_lossy().to_string()),
        include_headers: true,
        individual_emails_only: individual_only,
    };
    
    let export_data = export::export_contacts(&all_contacts, &export_options)?;
    
    // Handle REDCap API upload
    if let (Some(url), Some(token)) = (redcap_url, redcap_token) {
        if matches!(format, ExportFormat::Redcap) {
            let config = RedcapConfig {
                api_url: url,
                api_token: token,
            };
            
            let csv_data = extract_csv_from_redcap_export(&export_data);
            match upload_to_redcap(&csv_data, &config) {
                Ok(result) => {
                    if result.success {
                        println!("\n{}", "✓ UPLOAD SUCCESSFUL!".bright_green().bold());
                        println!("Records imported: {}", result.records_imported.to_string().bright_green());
                        println!("Record IDs: {:?}", result.ids);
                        println!("HTTP Status: {}", result.response_code);
                    } else {
                        println!("\n{}", "✗ UPLOAD FAILED".red().bold());
                        println!("HTTP Status: {}", result.response_code);
                        println!("Message: {}", result.message);
                        println!("{}", "Saving to file instead...".yellow());
                    }
                }
                Err(e) => {
                    println!("\n{}", "✗ UPLOAD ERROR".red().bold());
                    println!("Error: {}", e);
                    println!("{}", "Saving to file instead...".yellow());
                }
            }
        } else {
            println!("{}", "Warning: REDCap API parameters provided but format is not 'redcap'".yellow());
        }
    }
    
    if let Some(path) = output {
        std::fs::write(&path, &export_data)?;
        println!("\n{}", format!("Export saved to: {}", path.display()).bright_green().bold());
    } else {
        println!("\n{}", export_data);
    }
    
    Ok(())
}

fn search_and_display(client: &ClinicalTrialsClient, query: QueryBuilder, max: usize) -> Result<()> {
    let query = query.page_size(max.min(1000));
    let params = query.build();
    
    println!("{}", "Fetching studies...".cyan());
    let response = client.search_studies(&params)?;
    
    if response.studies.is_empty() {
        println!("{}", "No studies found.".yellow());
        return Ok(());
    }
    
    println!("{}", format!("Found {} studies", response.studies.len()).green());
    
    // Extract and display contacts
    let mut all_contacts = Vec::new();
    
    for (i, study) in response.studies.iter().enumerate() {
        let contacts = extract_contacts(study);
        
        if !contacts.contacts.is_empty() {
            all_contacts.push(contacts.clone());
            print_contacts(&contacts, false);
        } else {
            println!("\n{} {} ({}): {}",
                (i + 1).to_string().bright_black(),
                study.protocol_section.identification_module.nct_id.bright_white(),
                "No contacts".yellow(),
                study.protocol_section
                    .identification_module
                    .brief_title
                    .as_deref()
                    .unwrap_or("Untitled")
            );
        }
    }
    
    // Offer to export
    if !all_contacts.is_empty() {
        println!("\n{}", "Summary:".bright_cyan().bold());
        println!("  Studies with contacts: {}", all_contacts.len().to_string().green());
        
        let total_contacts: usize = all_contacts.iter().map(|s| s.contacts.len()).sum();
        println!("  Total contacts found: {}", total_contacts.to_string().green());
        
        let individual_count: usize = all_contacts.iter()
            .map(|s| s.individual_emails().len())
            .sum();
        println!("  Individual emails: {}", individual_count.to_string().bright_green());
        
        // Generate CSV by default
        let export_options = ExportOptions {
            format: ExportFormat::Csv,
            output_path: None,
            include_headers: true,
            individual_emails_only: false,
        };
        
        let csv = export::export_contacts(&all_contacts, &export_options)?;
        println!("\n{}", "CSV Export (copy below):".bright_yellow().bold());
        println!("{}", "-".repeat(80).bright_black());
        println!("{}", csv);
    }
    
    Ok(())
}

fn show_api_info(client: &ClinicalTrialsClient) -> Result<()> {
    println!("{}", "ClinicalTrials.gov API Information".bright_cyan().bold());
    println!();
    
    println!("{}", "Fetching API version...".cyan());
    let version = client.get_version()?;
    println!("{}", serde_json::to_string_pretty(&version)?);
    
    println!("\n{}", "Fetching database statistics...".cyan());
    let stats = client.get_stats_size()?;
    println!("{}", serde_json::to_string_pretty(&stats)?);
    
    println!("\n{}", "Search Capabilities:".green().bold());
    println!("  • Search by condition/disease");
    println!("  • Search by intervention/treatment");
    println!("  • Search by location (city, state, country)");
    println!("  • Search by sponsor/organization");
    println!("  • Filter by recruitment status");
    println!("  • Filter by study type");
    println!("  • Custom complex queries");
    
    println!("\n{}", "Contact Information Extracted:".green().bold());
    println!("  • Central contacts (study-wide)");
    println!("  • Overall study officials");
    println!("  • Location-specific contacts");
    println!("  • Responsible party information");
    println!("  • Point of contact (from results)");
    
    println!("\n{}", "Email Prioritization:".green().bold());
    println!("  • Individual emails (preferred)");
    println!("  • Institutional/research office emails (fallback)");
    
    Ok(())
}

fn print_available_parameters() {
    println!("{}", "=== Available Search Parameters ===\n".bright_cyan().bold());
    
    println!("{}", "Basic Search Parameters:".bright_yellow().bold());
    println!("  {} - Full text search query", "--query".green());
    println!("  {} - Medical condition or disease", "--condition".green());
    println!("  {} - Intervention or treatment", "--intervention".green());
    println!("  {} - Geographic location (city, state, country)", "--location".green());
    println!("  {} - Sponsor or organization name", "--sponsor".green());
    
    println!("\n{}", "Filter Parameters:".bright_yellow().bold());
    println!("  {} - Recruitment status", "--status".green());
    println!("    Values: RECRUITING, COMPLETED, ACTIVE_NOT_RECRUITING,");
    println!("            WITHDRAWN, SUSPENDED, TERMINATED, NOT_YET_RECRUITING");
    
    println!("\n  {} - Type of study", "--study-type".green());
    println!("    Values: INTERVENTIONAL, OBSERVATIONAL, EXPANDED_ACCESS");
    
    println!("\n{}", "Date Range Parameters:".bright_yellow().bold());
    println!("  {} - Study start date from (YYYY-MM-DD)", "--start-date-from".green());
    println!("  {} - Study start date to (YYYY-MM-DD)", "--start-date-to".green());
    println!("  {} - Primary completion date from (YYYY-MM-DD)", "--completion-date-from".green());
    println!("  {} - Primary completion date to (YYYY-MM-DD)", "--completion-date-to".green());
    
    println!("\n{}", "Result Limits:".bright_yellow().bold());
    println!("  {} - Maximum number of results to fetch", "--max".green());
    println!("  {} - Minimum results in batch config", "min_results".green());
    println!("  {} - Maximum results in batch config", "max_results".green());
    
    println!("\n{}", "Batch Configuration Parameters (CSV/JSON):".bright_yellow().bold());
    println!("  {} - Search name/identifier", "name".cyan());
    println!("  {} - Medical condition", "condition".cyan());
    println!("  {} - Intervention/treatment", "intervention".cyan());
    println!("  {} - Location", "location".cyan());
    println!("  {} - Sponsor/organization", "sponsor".cyan());
    println!("  {} - Recruitment status", "status".cyan());
    println!("  {} - Study type", "study_type".cyan());
    println!("  {} - Start date from (YYYY-MM-DD)", "start_date_from".cyan());
    println!("  {} - Start date to (YYYY-MM-DD)", "start_date_to".cyan());
    println!("  {} - Completion date from (YYYY-MM-DD)", "completion_date_from".cyan());
    println!("  {} - Completion date to (YYYY-MM-DD)", "completion_date_to".cyan());
    println!("  {} - Minimum results", "min_results".cyan());
    println!("  {} - Maximum results", "max_results".cyan());
    
    println!("\n{}", "Example Usage:".bright_yellow().bold());
    println!("  # Search with date range");
    println!("  {} custom --condition \"diabetes\" \\", "cargo run --".bright_black());
    println!("    --start-date-from \"2024-01-01\" \\");
    println!("    --start-date-to \"2025-12-31\" \\");
    println!("    --status \"RECRUITING\" \\");
    println!("    --max 50");
    
    println!("\n  # Generate batch template");
    println!("  {} template csv --output searches.csv", "cargo run --".bright_black());
    
    println!("\n  # Run batch search with date filters");
    println!("  {} batch searches.csv --output results.csv", "cargo run --".bright_black());
    
    println!("\n{}", "For more information:".bright_yellow().bold());
    println!("  {} - Show API info", "cargo run -- info".bright_black());
    println!("  {} - Generate search template", "cargo run -- template csv".bright_black());
    println!("  {} - View help for a command", "cargo run -- <command> --help".bright_black());
    println!();
}