# Clinical Trials Lookup

A comprehensive Rust application for searching ClinicalTrials.gov and extracting contact information for research survey distribution. Supports batch processing, multiple export formats (CSV, Google Sheets, REDCap), and flexible search parameters.

## Features

- **Multiple Search Methods**: Search by condition, intervention, location, sponsor, or custom queries
- **Batch Processing**: Run multiple searches from a configuration file (CSV or JSON)
- **Date Range Filtering**: Filter studies by start/completion dates  
- **Contact Extraction**: Automatically extracts contact information including:
  - Name, Role/Title
  - Email address (prioritizes individual over institutional emails)
  - Affiliation, Phone number
- **Multiple Export Formats**:
  - **CSV**: Standard comma-separated values
  - **Google Sheets**: CSV with import instructions and API setup guide
  - **REDCap**: Formatted for REDCap data import with survey tracking fields
  - **JSON**: Structured JSON format
- **Email Prioritization**: Distinguishes between individual and institutional emails
- **Pretty CLI Output**: Colored, formatted output for easy reading

## Quick Start

### Installation

```bash
git clone <repository-url>
cd Clinical-Trials-Lookup
cargo build --release
```

The binary will be at `target/release/clinical_trials_lookup` (or `.exe` on Windows).

### Basic Usage

```bash
# Single search  
cargo run -- condition "diabetes" --max 20

# Batch search with REDCap export
cargo run -- batch searches.csv --format redcap --output contacts.csv
```

## Usage Guide

### 1. Single Search Commands

#### Search by Condition

```bash
# Basic search
cargo run -- condition "diabetes"

# Only recruiting studies
cargo run -- condition "diabetes" --recruiting

# Limit results
cargo run -- condition "breast cancer" --max 50
```

#### Search by Intervention/Treatment

```bash
# Search for drug trials
cargo run -- intervention "aspirin"

# Search for specific treatments
cargo run -- intervention "immunotherapy" --max 15
```

#### Search by Location

```bash
# By city and state
cargo run -- location "Boston, MA"

# By country
cargo run -- location "United States"

# By institution
cargo run -- location "Mayo Clinic"
```

#### Search by Sponsor

```bash
# Pharmaceutical company
cargo run -- sponsor "Pfizer"

# University
cargo run -- sponsor "Johns Hopkins University"
```

#### Get Specific Study

```bash
cargo run -- study NCT05123456
```

#### Custom Search with Multiple Parameters

```bash
# Complex search
cargo run -- custom \
  --condition "diabetes" \
  --location "California" \
  --status "RECRUITING" \
  --max 25

# By intervention and condition
cargo run -- custom \
  --intervention "metformin" \
  --condition "type 2 diabetes"

# With date range filtering
cargo run -- custom \
  --condition "diabetes" \
  --start-date-from "2024-01-01" \
  --start-date-to "2025-12-31" \
  --status "RECRUITING" \
  --max 50

# Filter by completion date
cargo run -- custom \
  --condition "cancer" \
  --completion-date-from "2020-01-01" \
  --completion-date-to "2023-12-31" \
  --status "COMPLETED"
```

#### List All Available Parameters

```bash
# Show all search parameters and their values
cargo run -- parameters
```

This displays:
- Basic search parameters (condition, intervention, location, sponsor, query)
- Filter parameters (status, study_type)
- Date range parameters (start_date_from/to, completion_date_from/to)
- Batch configuration parameters
- Usage examples

### 2. Batch Processing (Table-Based Searches)

Batch processing allows you to run multiple searches at once using a configuration file.

#### Generate a Template

```bash
# Generate CSV template
cargo run -- template csv --output searches.csv

# Generate JSON template  
cargo run -- template json --output searches.json
```

#### CSV Template Format

The CSV template has the following columns:

| Column | Description | Example |
|--------|-------------|---------|
| name | Search name/identifier | "Diabetes Studies" |
| condition | Medical condition | "diabetes" |
| intervention | Treatment/drug | "metformin" |
| location | Geographic location | "California" |
| sponsor | Organization name | "Stanford" |
| status | Recruitment status | "RECRUITING" |
| study_type | Type of study | "INTERVENTIONAL" |
| start_date_from | Minimum start date | "2020-01-01" |
| start_date_to | Maximum start date | "2025-12-31" |
| completion_date_from | Min completion date | "2023-01-01" |
| completion_date_to | Max completion date | "" |
| min_results | Minimum results | 10 |
| max_results | Maximum results | 100 |

**Example CSV:**

```csv
name,condition,intervention,location,sponsor,status,study_type,start_date_from,start_date_to,completion_date_from,completion_date_to,min_results,max_results
Diabetes CA,diabetes,,,Stanford,RECRUITING,,2020-01-01,2025-12-31,,,10,100
Cancer Trials,cancer,,Boston,,,INTERVENTIONAL,2023-01-01,,,,20,50
COVID Vaccines,COVID-19,vaccine,,,RECRUITING,,2020-01-01,2024-12-31,,,5,30
```

#### JSON Template Format

```json
{
  "searches": [
    {
      "name": "Diabetes Studies",
      "condition": "diabetes",
      "status": "RECRUITING",
      "max_results": 100
    },
    {
      "name": "Cancer Trials in California",
      "condition": "cancer",
      "location": "California",
      "status": "RECRUITING",
      "study_type": "INTERVENTIONAL",
      "start_date_from": "2023-01-01",
      "max_results": 50
    }
  ],
  "export_format": "csv",
  "output_file": "contacts_export.csv",
  "individual_emails_only": true
}
```

#### Run Batch Searches

```bash
# From CSV file
cargo run -- batch searches.csv

# From CSV with custom export format
cargo run -- batch searches.csv --format redcap --output results.csv

# From JSON file
cargo run -- batch searches.json --format google-sheets --output results.txt

# Only export individual emails (exclude generic institutional emails)
cargo run -- batch searches.csv --individual-only --output contacts.csv
```

### 3. Export Formats

#### CSV Export (Default)

Standard CSV with headers:
- NCT ID, Study Title, Contact Name, Role, Email, Source, Affiliation, Phone

```bash
cargo run -- condition "diabetes" --max 20 > contacts.csv
```

#### Google Sheets Export

Includes:
- CSV data for manual import
- Instructions for Google Sheets import wizard
- API setup guide for direct upload (if feature enabled)
- Useful Google Sheets formulas for data analysis

```bash
cargo run -- batch searches.csv --format google-sheets --output gsheets.txt
```

**Import to Google Sheets:**
1. Open Google Sheets
2. File → Import → Upload
3. Select the CSV portion
4. Choose import options

#### REDCap Export

Includes:
- REDCap-formatted CSV with `record_id`
- Complete data dictionary for REDCap project setup
- Survey tracking fields (email_sent, email_opened, survey_completed)
- Import instructions and API upload command

```bash
cargo run -- batch searches.csv --format redcap --output redcap_import.csv
```

**REDCap Fields Included:**
- Study and contact information
- `email_sent` - Survey sent flag (0/1)
- `email_opened` - Survey opened flag (0/1)
- `survey_completed` - Survey completed flag (0/1)
- `response_date` - Date of response
- `notes` - Additional notes field

**Import to REDCap:**
1. Create REDCap project with provided data dictionary
2. Go to Data Import Tool
3. Upload the CSV file
4. Follow import wizard

#### JSON Export

Structured JSON format for programmatic use:

```bash
cargo run -- condition "cancer" --max 10 --format json --output contacts.json
```

### 4. Common Workflows

#### Workflow 1: Survey Distribution to Multiple Study Types

```bash
# Create config file
cargo run -- template csv --output multi_search.csv

# Edit multi_search.csv with your criteria:
# - Diabetes studies in California
# - Cancer trials in Boston
# - COVID-19 vaccine studies

# Run batch search with REDCap export
cargo run -- batch multi_search.csv \
  --format redcap \
  --output survey_contacts.csv \
  --individual-only
```

#### Workflow 2: Find Contacts for Specific Date Range

Create `recent_studies.csv`:
```csv
name,condition,status,start_date_from,start_date_to,max_results
Recent Diabetes,diabetes,RECRUITING,2024-01-01,2025-12-31,50
Recent Cancer,cancer,RECRUITING,2024-01-01,2025-12-31,50
```

```bash
cargo run -- batch recent_studies.csv --output recent_contacts.csv
```

#### Workflow 3: Export for Google Sheets Analysis

```bash
cargo run -- batch searches.csv \
  --format google-sheets \
  --output gsheets_import.txt

# Open Google Sheets and import the CSV portion
# Use provided formulas for analysis
```

### 5. API Information

```bash
# Get API version and capabilities
cargo run -- info
```

## Search Parameters

**To see all available parameters, run:**
```bash
cargo run -- parameters
```

### Status Values
- `RECRUITING` - Currently recruiting
- `ACTIVE_NOT_RECRUITING` - Active but not recruiting
- `COMPLETED` - Completed
- `WITHDRAWN` - Withdrawn
- `SUSPENDED` - Suspended
- `TERMINATED` - Terminated
- `NOT_YET_RECRUITING` - Not yet recruiting

### Study Types
- `INTERVENTIONAL` - Interventional studies
- `OBSERVATIONAL` - Observational studies  
- `EXPANDED_ACCESS` - Expanded access

### Date Parameters
- `--start-date-from` - Study start date from (YYYY-MM-DD)
- `--start-date-to` - Study start date to (YYYY-MM-DD)
- `--completion-date-from` - Primary completion date from (YYYY-MM-DD)
- `--completion-date-to` - Primary completion date to (YYYY-MM-DD)

**In batch CSV/JSON:**
- `start_date_from`, `start_date_to`
- `completion_date_from`, `completion_date_to`

### Date Formats
- `YYYY-MM-DD` (e.g., `2024-01-15`)
- Dates are in UTC
- Use single dates or ranges
- Can filter by both start and completion dates simultaneously

## Contact Information Sources

The application extracts contacts from:

1. **Central Contacts** (study-wide contacts with emails)
2. **Location Contacts** (facility-specific contacts with emails)
3. **Point of Contact** (results contacts with emails)
4. **Overall Officials** (usually no email in API)
5. **Responsible Party** (usually no email in API)

## Email Prioritization

**Individual Emails (Preferred):**
- Personal email addresses
- More likely to be direct contacts
- Excludes generic patterns: `info@`, `contact@`, `admin@`, `support@`, `office@`, `research@`, `clinical@`, `trials@`, `study@`, `recruitment@`, `coordinator@`

**Institutional Emails (Fallback):**
- Research office or department emails
- Used when individual emails unavailable

## Project Structure

```
src/
├── main.rs           # CLI and command handlers
├── api/              # ClinicalTrials.gov API client
│   ├── client.rs     # HTTP client
│   ├── models.rs     # Data structures
│   └── search.rs     # Query builder
├── contact.rs        # Contact extraction logic
├── export/           # Export formats
│   ├── csv_export.rs # CSV exporter
│   ├── google_sheets.rs # Google Sheets export
│   └── redcap.rs     # REDCap export
└── batch/            # Batch processing
    └── mod.rs        # Config file handling
```

## Examples

### Example 1: Multi-Center Cancer Study Contacts

`cancer_centers.csv`:
```csv
name,condition,location,status,max_results
Dana-Farber,cancer,Boston,RECRUITING,30
MD Anderson,cancer,Houston,RECRUITING,30
Mayo Clinic,cancer,Rochester,RECRUITING,30
```

```bash
cargo run -- batch cancer_centers.csv --output cancer_contacts.csv
```

### Example 2: Recent Drug Trials with REDCap Tracking

`recent_drugs.json`:
```json
{
  "searches": [
    {
      "name": "New Cancer Drugs",
      "condition": "cancer",
      "study_type": "INTERVENTIONAL",
      "status": "RECRUITING",
      "start_date_from": "2024-01-01",
      "max_results": 100
    }
  ],
  "export_format": "redcap",
  "output_file": "drug_trials.csv",
  "individual_emails_only": true
}
```

```bash
cargo run -- batch recent_drugs.json
```

### Example 3: Multi-Condition Survey Distribution

```bash
# Generate template
cargo run -- template csv --output conditions.csv

# Edit to include: diabetes, hypertension, obesity

# Run and export for Google Sheets
cargo run -- batch conditions.csv --format google-sheets --output gsheets.txt
```

## Best Practices

1. **Start with Templates**: Use `template` command to create properly formatted config files
2. **Test Searches First**: Run individual searches before batch processing
3. **Use Date Ranges**: Filter by start/completion dates for relevant studies
4. **Individual Emails**: Use `--individual-only` for better response rates
5. **Export to REDCap**: Use REDCap format for survey tracking and follow-up
6. **Verify Data**: Check exported contacts before sending surveys
7. **Respect Privacy**: Use contact information only for legitimate research

## Troubleshooting

### No Results
- Try broader search terms
- Remove date filters
- Check API status with `info` command

### No Contacts
- Focus on RECRUITING studies
- Remove `--individual-only` flag
- Many studies lack public email addresses

### Export Issues
- Verify file permissions for output path
- Check disk space
- Ensure valid CSV/JSON format for batch files

## API Reference

- **Base URL**: `https://clinicaltrials.gov/api/v2`
- **Documentation**: [ClinicalTrials.gov API](https://clinicaltrials.gov/data-api/api)
- **Rate Limiting**: None explicitly stated, but be respectful
- **Updates**: Daily (Mon-Fri, ~9 AM ET)

## Use Cases

1. **Survey Distribution**: Find contacts for distributing research surveys
2. **Study Recruitment**: Identify collaborators or study sites  
3. **Literature Review**: Contact study authors for follow-up
4. **Network Building**: Connect with researchers in fields
5. **Meta-Analysis**: Contact authors for additional data

## Contributing

Areas for improvement:
- Direct Google Sheets API integration
- REDCap API upload
- Email validation
- Duplicate detection
- Additional export formats (Excel, PDF)

## Disclaimer

This tool is for research purposes only. Respect privacy of study contacts and use information responsibly. Follow your institution's IRB guidelines and ethical standards when contacting study personnel.

## Acknowledgments

- Data: ClinicalTrials.gov
- API: [ClinicalTrials.gov Data API](https://clinicaltrials.gov/data-api)
