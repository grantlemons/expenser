use crate::schema::*;
use diesel::prelude::*;

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

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub profile_picture: Option<&'a [u8]>,
    pub password_hash: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = reports)]
pub struct Report {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = reports)]
pub struct NewReport<'a> {
    pub owner_id: i64,
    pub title: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_proof)]
pub struct ReportProof {
    pub id: i64,
    pub report_id: i64,
    pub data: Vec<u8>,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_proof)]
pub struct NewReportProof<'a> {
    pub report_id: i64,
    pub data: &'a [u8],
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = borrower_id))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_permissions)]
pub struct ReportPermissions {
    pub id: i64,
    pub borrower_id: i64,
    pub report_id: i64,
    pub read_access: bool,
    pub write_access: bool,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Report))]
#[diesel(table_name = report_permissions)]
pub struct NewReportPermissions {
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
pub struct NewReportLineItem<'a> {
    pub report_id: i64,
    pub item_name: &'a str,
    pub item_price_usd: diesel::data_types::Cents,
}
