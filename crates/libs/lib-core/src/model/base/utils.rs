use crate::model::base::{CommonIden, DbBmc, TimestampIden, TimestampType, ids::generate_id_for_table};
use chrono::{DateTime, Utc};
use modql::field::{SeaField, SeaFields};
use sea_query::IntoIden;

/// This method must be called when a model controller intends to create its entity.
pub fn prep_fields_for_create<MC>(fields: &mut SeaFields, user_id: &str)
where
    MC: DbBmc,
{
    if MC::has_id() {
        let id = generate_id_for_table(MC::TABLE);
        fields.push(SeaField::new(CommonIden::Id, id));
    }

    if MC::has_owner_id() {
        fields.push(SeaField::new(CommonIden::OwnerId.into_iden(), user_id));
    }
    
    if MC::has_user_id() {
        fields.push(SeaField::new(CommonIden::UserId.into_iden(), user_id));
    }
    
    // Add timestamps ONLY if table has them
    add_timestamps_for_create(fields, user_id, MC::timestamp_fields());
}

/// This method must be calledwhen a Model Controller plans to update its entity.
/// Assumes fields are NOT already present.
pub fn prep_fields_for_update<MC>(fields: &mut SeaFields, user_id: &str)
where
    MC: DbBmc,
{
    // Update timestamps ONLY if table has them
    add_timestamps_for_update(fields, user_id, MC::timestamp_fields());
}

/// Update the timestamps info for create
/// (e.g., cid, ctime, and mid, mtime will be updated with the same values)
fn add_timestamps_for_create(fields: &mut SeaFields, user_id: &str, timestamp_type: TimestampType) {
    let now: DateTime<Utc> = Utc::now();
    match timestamp_type {
        TimestampType::Full => {
            // For tables with cid, ctime, mid, mtime
            fields.push(SeaField::new(TimestampIden::Cid, user_id));
            fields.push(SeaField::new(TimestampIden::Ctime, now));
            fields.push(SeaField::new(TimestampIden::Mid, user_id));
            fields.push(SeaField::new(TimestampIden::Mtime, now));
        }
        TimestampType::CtimeOnly => {
            // For tables only with ctime
            fields.push(SeaField::new(TimestampIden::Ctime, now));
        }
        TimestampType::CtimeMtime => {
            // For tables with ctime and mtime
            fields.push(SeaField::new(TimestampIden::Ctime, now));
            fields.push(SeaField::new(TimestampIden::Mtime, now));
        }
        TimestampType::None => {}
    }
}

/// Update the timestamps info only for update.
/// (.e.g., only mid, mtime will be udpated)
fn add_timestamps_for_update(fields: &mut SeaFields, user_id: &str, timestamp_type: TimestampType) {
    let now: DateTime<Utc> = Utc::now();
    match timestamp_type {
        TimestampType::Full => {
            // Only mid, mtime
            fields.push(SeaField::new(TimestampIden::Mid, user_id));
            fields.push(SeaField::new(TimestampIden::Mtime, now));
        }
        TimestampType::CtimeOnly => {
            // Do not add anything (only ctime when create)
        }
        TimestampType::CtimeMtime => {
            // Only mtime
            fields.push(SeaField::new(TimestampIden::Mtime, now));
        }
        TimestampType::None => {}
    }
}