// ==============================================================================
// VRAM-Synth: Generate Synthetic Data - Complete Implementation
// Building on your existing Rust canister and Svelte frontend
// ==============================================================================

// 1. ADD TO YOUR EXISTING RUST CANISTER (lib.rs)
// ==============================================================================

use candid::{CandidType, Deserialize};
use serde::{Serialize};
use std::collections::HashMap;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};

// Add these new structs to your existing canister code
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct DataProfile {
    pub dataset_id: String,
    pub column_analysis: Vec<ColumnProfile>,
    pub quality_score: u8,
    pub privacy_risks: Vec<PrivacyRisk>,
    pub correlations: Vec<ColumnCorrelation>,
    pub recommendations: Vec<String>,
    pub total_rows: u32,
    pub total_columns: u32,
    pub created_at: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ColumnProfile {
    pub name: String,
    pub data_type: String, // "numeric", "categorical", "text", "email", "phone", "id", "date"
    pub unique_count: u32,
    pub missing_count: u32,
    pub total_count: u32,
    pub examples: Vec<String>,
    pub stats: Option<NumericStats>,
    pub top_values: Option<Vec<(String, u32)>>,
    pub privacy_level: String, // "low", "medium", "high"
    pub pii_detected: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct NumericStats {
    pub mean: f64,
    pub std: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct PrivacyRisk {
    pub risk_type: String, // "pii", "high_uniqueness", "small_groups"
    pub column: String,
    pub severity: String, // "low", "medium", "high"
    pub description: String,
    pub recommendation: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ColumnCorrelation {
    pub col1: String,
    pub col2: String,
    pub correlation: f64,
    pub correlation_type: String, // "strong", "moderate", "weak"
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct SyntheticDataRequest {
    pub dataset_id: String,
    pub num_records: u32,
    pub privacy_level: String, // "low", "medium", "high"
    pub custom_prompt: Option<String>,
    pub preserve_correlations: bool,
    pub hipaa_compliant: bool,
    pub medical_mode: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct SyntheticDataJob {
    pub job_id: String,
    pub dataset_id: String,
    pub status: String, // "pending", "processing", "completed", "failed"
    pub progress: u8, // 0-100
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub result_dataset_id: Option<String>,
    pub error_message: Option<String>,
    pub settings: SyntheticDataRequest,
}

// Add to your existing STABLE_DATA or create new stable storage
thread_local! {
    static DATA_PROFILES: RefCell<StableBTreeMap<String, DataProfile, Memory>> = 
        RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))));
    
    static SYNTHETIC_JOBS: RefCell<StableBTreeMap<String, SyntheticDataJob, Memory>> = 
        RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))));
}

// ==============================================================================
// DATA PROFILING FUNCTIONS
// ==============================================================================

#[ic_cdk::update]
pub async fn analyze_dataset(dataset_id: String) -> Result<DataProfile, String> {
    let caller = ic_cdk::caller();
    
    // Get the dataset from your existing storage
    let note = STABLE_DATA.with(|data| {
        data.borrow()
            .iter()
            .find(|(_, note)| note.id == dataset_id && note.owner == caller)
            .map(|(_, note)| note.clone())
    }).ok_or("Dataset not found or access denied")?;
    
    // Parse CSV content
    let csv_content = decrypt_note_content(&note)?;
    let profile = analyze_csv_content(&csv_content, &dataset_id)?;
    
    // Store profile
    DATA_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(dataset_id.clone(), profile.clone())
    });
    
    Ok(profile)
}

fn analyze_csv_content(csv_content: &str, dataset_id: &str) -> Result<DataProfile, String> {
    let lines: Vec<&str> = csv_content.lines().collect();
    if lines.is_empty() {
        return Err("Empty dataset".to_string());
    }
    
    let headers: Vec<&str> = lines[0].split(',').map(|h| h.trim().trim_matches('"')).collect();
    let data_rows: Vec<Vec<&str>> = lines[1..]
        .iter()
        .map(|line| line.split(',').map(|cell| cell.trim().trim_matches('"')).collect())
        .collect();
    
    let mut column_profiles = Vec::new();
    let mut privacy_risks = Vec::new();
    
    for (col_idx, header) in headers.iter().enumerate() {
        let column_data: Vec<&str> = data_rows
            .iter()
            .filter_map(|row| row.get(col_idx))
            .copied()
            .collect();
        
        let profile = analyze_column(header, &column_data);
        
        // Check for privacy risks
        if profile.pii_detected {
            privacy_risks.push(PrivacyRisk {
                risk_type: "pii".to_string(),
                column: header.to_string(),
                severity: "high".to_string(),
                description: format!("Column '{}' may contain personally identifiable information", header),
                recommendation: "Consider anonymizing or removing this column".to_string(),
            });
        }
        
        if profile.unique_count as f32 / profile.total_count as f32 > 0.9 {
            privacy_risks.push(PrivacyRisk {
                risk_type: "high_uniqueness".to_string(),
                column: header.to_string(),
                severity: "medium".to_string(),
                description: format!("Column '{}' has high uniqueness (potential identifier)", header),
                recommendation: "Consider generalizing values or adding noise".to_string(),
            });
        }
        
        column_profiles.push(profile);
    }
    
    // Calculate correlations (simplified)
    let correlations = calculate_correlations(&column_profiles, &data_rows);
    
    // Calculate quality score
    let quality_score = calculate_quality_score(&column_profiles, &privacy_risks);
    
    // Generate recommendations
    let recommendations = generate_recommendations(&column_profiles, &privacy_risks);
    
    Ok(DataProfile {
        dataset_id: dataset_id.to_string(),
        column_analysis: column_profiles,
        quality_score,
        privacy_risks,
        correlations,
        recommendations,
        total_rows: data_rows.len() as u32,
        total_columns: headers.len() as u32,
        created_at: ic_cdk::api::time(),
    })
}

fn analyze_column(header: &str, data: &[&str]) -> ColumnProfile {
    let total_count = data.len() as u32;
    let missing_count = data.iter().filter(|&&cell| cell.is_empty() || cell == "NULL" || cell == "NaN").count() as u32;
    let non_empty_data: Vec<&str> = data.iter().filter(|&&cell| !cell.is_empty() && cell != "NULL" && cell != "NaN").copied().collect();
    
    let mut unique_values = std::collections::HashSet::new();
    for &value in &non_empty_data {
        unique_values.insert(value);
    }
    let unique_count = unique_values.len() as u32;
    
    // Data type detection
    let data_type = detect_data_type(header, &non_empty_data);
    let pii_detected = detect_pii(header, &non_empty_data);
    
    // Privacy level assessment
    let privacy_level = if pii_detected {
        "high".to_string()
    } else if unique_count as f32 / total_count as f32 > 0.8 {
        "medium".to_string()
    } else {
        "low".to_string()
    };
    
    // Get examples (first 3 non-empty values)
    let examples: Vec<String> = non_empty_data.iter().take(3).map(|&s| s.to_string()).collect();
    
    // Calculate statistics for numeric data
    let stats = if data_type == "numeric" {
        calculate_numeric_stats(&non_empty_data)
    } else {
        None
    };
    
    // Get top values for categorical data
    let top_values = if data_type == "categorical" && unique_count <= 20 {
        let mut value_counts = std::collections::HashMap::new();
        for &value in &non_empty_data {
            *value_counts.entry(value).or_insert(0) += 1;
        }
        let mut sorted_values: Vec<(String, u32)> = value_counts.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        sorted_values.sort_by(|a, b| b.1.cmp(&a.1));
        Some(sorted_values.into_iter().take(5).collect())
    } else {
        None
    };
    
    ColumnProfile {
        name: header.to_string(),
        data_type,
        unique_count,
        missing_count,
        total_count,
        examples,
        stats,
        top_values,
        privacy_level,
        pii_detected,
    }
}

fn detect_data_type(header: &str, data: &[&str]) -> String {
    let header_lower = header.to_lowercase();
    
    // Check for ID patterns
    if header_lower.contains("id") || header_lower.contains("uuid") || header_lower.contains("key") {
        return "id".to_string();
    }
    
    // Check for email patterns
    if data.iter().any(|&value| value.contains('@') && value.contains('.')) {
        return "email".to_string();
    }
    
    // Check for phone patterns
    if data.iter().any(|&value| value.chars().filter(|c| c.is_ascii_digit()).count() >= 10) {
        return "phone".to_string();
    }
    
    // Check if numeric
    let numeric_count = data.iter()
        .filter(|&&value| value.parse::<f64>().is_ok())
        .count();
    
    if numeric_count as f64 / data.len() as f64 > 0.8 {
        return "numeric".to_string();
    }
    
    // Check for date patterns (simplified)
    if data.iter().any(|&value| value.contains('/') || value.contains('-') && value.len() >= 8) {
        return "date".to_string();
    }
    
    // Check unique values for categorical vs text
    let unique_ratio = data.iter().collect::<std::collections::HashSet<_>>().len() as f64 / data.len() as f64;
    
    if unique_ratio < 0.1 || data.iter().collect::<std::collections::HashSet<_>>().len() < 20 {
        "categorical".to_string()
    } else {
        "text".to_string()
    }
}

fn detect_pii(header: &str, data: &[&str]) -> bool {
    let header_lower = header.to_lowercase();
    
    // PII indicators in column names
    let pii_keywords = ["name", "email", "phone", "address", "ssn", "social", "credit", "card", "license"];
    if pii_keywords.iter().any(|&keyword| header_lower.contains(keyword)) {
        return true;
    }
    
    // Pattern detection in data
    for &value in data.iter().take(10) { // Check first 10 values
        // Email pattern
        if value.contains('@') && value.contains('.') {
            return true;
        }
        
        // Phone pattern (simplified)
        if value.len() >= 10 && value.chars().filter(|c| c.is_ascii_digit()).count() >= 10 {
            return true;
        }
        
        // SSN pattern (XXX-XX-XXXX or similar)
        if value.len() >= 9 && value.matches(char::is_numeric).count() >= 9 {
            return true;
        }
    }
    
    false
}

fn calculate_numeric_stats(data: &[&str]) -> Option<NumericStats> {
    let numbers: Vec<f64> = data.iter()
        .filter_map(|&s| s.parse::<f64>().ok())
        .collect();
    
    if numbers.is_empty() {
        return None;
    }
    
    let sum: f64 = numbers.iter().sum();
    let mean = sum / numbers.len() as f64;
    
    let variance: f64 = numbers.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / numbers.len() as f64;
    let std = variance.sqrt();
    
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let median = if sorted_numbers.len() % 2 == 0 {
        (sorted_numbers[sorted_numbers.len() / 2 - 1] + sorted_numbers[sorted_numbers.len() / 2]) / 2.0
    } else {
        sorted_numbers[sorted_numbers.len() / 2]
    };
    
    Some(NumericStats {
        mean,
        std,
        min: sorted_numbers[0],
        max: sorted_numbers[sorted_numbers.len() - 1],
        median,
    })
}

fn calculate_correlations(_columns: &