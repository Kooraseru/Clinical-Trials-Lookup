# Clinical Trials Lookup

A Rust application for searching ClinicalTrials.gov and extracting contact information for research survey distribution.

## Features

- **Multiple Search Methods**: Condition, intervention, location, sponsor, custom queries
- **Batch Processing**: Run multiple searches from CSV/JSON configuration files  
- **Export Formats**: CSV, Google Sheets, REDCap, JSON
- **REDCap API Integration**: Direct upload to REDCap projects with detailed logging
- **Contact Extraction**: Automatically extracts names, roles, emails, affiliations, phones
- **Email Prioritization**: Distinguishes individual vs institutional emails
- **Date Range Filtering**: Filter studies by start date and completion date (YYYY-MM-DD)
- **US-Only Filter**: `--us-only` flag to search only United States studies
- **Rate Limit Detection**: Automatic detection and reporting of API rate limits
- **Parameters Command**: List all available search parameters and filters

## Quick Start

```bash
# Build
cargo build --release

# Single search
cargo run -- condition "diabetes" --max 20

# Batch search with REDCap direct upload
cargo run -- batch searches.csv \
  --format redcap \
  --redcap-url "https://redcap.vumc.org/api/" \
  --redcap-token "YOUR_API_TOKEN"
```

## Documentation

**[See USAGE.md for comprehensive documentation](USAGE.md)**

**[See DATE_FILTERING.md for date filtering guide](DATE_FILTERING.md)**

Includes:
- Complete usage guide
- Table-based batch processing
- Export format details (CSV, Google Sheets, REDCap, JSON)
- Example workflows
- Configuration templates
- Best practices

## Key Commands

```bash
# List all available search parameters
cargo run -- parameters

# Generate configuration template
cargo run -- template csv --output searches.csv

# Run batch searches
cargo run -- batch searches.csv --format redcap --output contacts.csv

# Single searches
cargo run -- condition "cancer" --recruiting --max 50
cargo run -- location "California" --max 30
cargo run -- sponsor "Stanford University"

# Search with date ranges
cargo run -- custom --condition "diabetes" \
  --start-date-from "2024-01-01" \
  --start-date-to "2025-12-31" \
  --status "RECRUITING" --max 50

# Direct upload to REDCap
cargo run -- custom --condition "diabetes" \
  --start-date-from "2016-02-08" \
  --start-date-to "2026-02-08" \
  --max 2000 \
  --us-only \
  --format redcap \
  --individual-only \
  --redcap-url "https://redcap.vumc.org/api/" \
  --redcap-token "YOUR_API_TOKEN"
```

## Batch Processing Example

Create `searches.csv`:
```csv
name,condition,location,status,max_results
Diabetes CA,diabetes,California,RECRUITING,100
Cancer Trials,cancer,Boston,RECRUITING,50
```

Run:
```bash
cargo run -- batch searches.csv --format redcap --output results.csv
```

## Export Formats

- **CSV**: Standard format for spreadsheets
- **Google Sheets**: Includes import instructions and formulas
- **REDCap**: Formatted for REDCap import with survey tracking fields
- **JSON**: Structured data for programmatic use

## Use Cases

- Distribute research surveys
- Find study collaborators
- Contact researchers for follow-up
- Build research networks
- Gather data for meta-analysis

## Requirements

- Rust 1.70+
- Internet connection for API access