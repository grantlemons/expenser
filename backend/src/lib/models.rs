use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub profile_picture: Option<Vec<u8>>,
    pub password_hash: String,
}

#[derive(Deserialize, Insertable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub profile_picture: Option<Vec<u8>>,
    pub password_hash: String,
}

#[derive(Serialize, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = reports)]
pub struct Report {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Insertable, Debug, PartialEq)]
#[diesel(table_name = reports)]
pub struct NewReport {
    pub owner_id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_proof)]
pub struct ReportProof {
    pub id: i64,
    pub report_id: i64,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Insertable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_proof)]
pub struct NewReportProof {
    pub report_id: i64,
    pub data: Vec<u8>,
}

#[derive(Serialize, Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = borrower_id))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_access)]
pub struct ReportAccess {
    pub id: i64,
    pub borrower_id: i64,
    pub report_id: i64,
    pub read_access: bool,
    pub write_access: bool,
}

#[derive(Deserialize, Insertable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_access)]
pub struct NewReportAccess {
    pub borrower_id: i64,
    pub report_id: i64,
    pub read_access: bool,
    pub write_access: bool,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_line_items)]
pub struct ReportLineItem {
    pub id: i64,
    pub report_id: i64,
    pub item_name: String,
    pub item_price_usd: diesel::data_types::Cents,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_line_items)]
pub struct NewReportLineItem {
    pub report_id: i64,
    pub item_name: String,
    pub item_price_usd: diesel::data_types::Cents,
}
