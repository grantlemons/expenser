use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub profile_picture: Option<Vec<u8>>,
    pub password_hash: String,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_id: i64,
    pub username: &'a str,
    pub email: &'a str,
    pub profile_picture: Option<&'a [u8]>,
    pub password_hash: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(invoice_id))]
#[diesel(table_name = invoices)]
pub struct Invoice {
    pub invoice_id: i64,
    pub owner_id: i64,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = invoices)]
pub struct NewInvoice {
    pub invoice_id: i64,
    pub owner_id: i64,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(proof_id))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_proof)]
pub struct InvoiceProof {
    pub proof_id: i64,
    pub invoice_id: i64,
    pub data: Vec<u8>,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_proof)]
pub struct NewInvoiceProof<'a> {
    pub proof_id: i64,
    pub invoice_id: i64,
    pub data: &'a [u8],
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(access_id))]
#[diesel(belongs_to(User, foreign_key = borrower_id))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_permissions)]
pub struct InvoicePermissions {
    pub access_id: i64,
    pub borrower_id: i64,
    pub invoice_id: i64,
    pub read_access: bool,
    pub write_access: bool,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(User, foreign_key = borrower_id))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_permissions)]
pub struct NewInvoicePermissions {
    pub access_id: i64,
    pub borrower_id: i64,
    pub invoice_id: i64,
    pub read_access: bool,
    pub write_access: bool,
}
