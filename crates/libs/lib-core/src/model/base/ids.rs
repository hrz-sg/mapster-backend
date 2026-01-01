#![allow(dead_code)]

use nanoid::nanoid;
use std::collections::HashMap;
use crate::model::{Error, Result};

/// Length of the random part (NanoID)
const NANOID_LENGTH: usize = 21;
/// Length of entity prefix (e.g., "usr_", "pst_")
const PREFIX_LENGTH: usize = 4;
/// Length of prefix letters only (without underscore)
const PREFIX_LETTERS_LENGTH: usize = 3;
/// Total ID length including prefix
pub const TOTAL_ID_LENGTH: usize = PREFIX_LENGTH + NANOID_LENGTH; // 25

fn _generate_nanoid() -> String {
    nanoid!(NANOID_LENGTH)
}

pub fn generate_user_id() -> String {
    format!("usr_{}", _generate_nanoid())
}

pub fn generate_post_id() -> String {
    format!("pst_{}", _generate_nanoid())
}

pub fn generate_comment_id() -> String {
    format!("cmt_{}", _generate_nanoid())
}

pub fn generate_journey_id() -> String {
    format!("jny_{}", _generate_nanoid())
}

pub fn generate_post_media_id() -> String {
    format!("pme_{}", _generate_nanoid())
}

pub fn generate_comment_media_id() -> String {
    format!("cme_{}", _generate_nanoid())
}

pub fn generate_collection_id() -> String {
    format!("col_{}", _generate_nanoid())
}

/// Check if string is valid user ID
pub fn is_user_id(id: &str) -> bool {
    id.starts_with("usr_") && id.len() == TOTAL_ID_LENGTH
}

/// Generate ID with custom 3-letter prefix (e.g., "usr" -> "usr_abc123...")
pub fn generate_id_with_prefix(prefix: &str) -> Result<String> {
    // Validate prefix is exactly 3 lowercase letters
    if prefix.len() != PREFIX_LETTERS_LENGTH {
        return Err(Error::InvalidIdPrefixLength(format!(
            "Prefix must be {} letters, got '{}' ({} chars)",
            PREFIX_LETTERS_LENGTH, prefix, prefix.len()
        )));
    }
    
    if !prefix.chars().all(|c| c.is_ascii_lowercase()) {
        return Err(Error::InvalidIdPrefixFormat(format!(
            "Prefix must be lowercase letters, got '{}'", prefix
        )));
    }
    
    Ok(format!("{}_{}", prefix, _generate_nanoid()))
}

/// ---- Table ID generation
pub fn generate_id_for_table(table: &str) -> String {
    match table {
        "user" => generate_user_id(),
        "post" => generate_post_id(),
        "comment" => generate_comment_id(),
        "journey" => generate_journey_id(),
        "post_media" => generate_post_media_id(),
        "comment_media" => generate_comment_media_id(),
        "post_collection" => generate_collection_id(),
        _ => {
            // Fallback: try to extract prefix
            let prefix = extract_prefix_from_table_name(table);
            format!("{}_{}", prefix, _generate_nanoid())
        }
    }
}

/// --- Extract prefix from the table
fn extract_prefix_from_table_name(table: &str) -> String {
    let mapping: HashMap<&str, &str> = [
        ("users", "usr"),
        ("posts", "pst"),
        ("comments", "cmt"),
        ("journeys", "jny"),
        ("post_media", "pme"),
        ("comment_media", "cme"),
        ("post_collections", "col"),
    ].iter().cloned().collect();
    
    mapping.get(table).unwrap_or(&"gen").to_string()
}

/// Check IDs for tables
pub fn validate_id_for_table(id: &str, table: &str) -> Result<()> {
    let expected_prefix = match table {
        "user" => "usr_",
        "post" => "pst_",
        "comment" => "cmt_",
        "journey" => "jny_",
        "post_media" => "pme_",
        "comment_media" => "cme_",
        "post_collection" => "col_",
        _ => return Ok(()), // DO not check other tables
    };
    
    if !id.starts_with(expected_prefix) || id.len() != TOTAL_ID_LENGTH {
        return Err(Error::InvalidIdFormat(format!(
            "Invalid ID for table '{}': {}", table, id
        )));
    }
    
    Ok(())
}