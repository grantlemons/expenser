#![allow(dead_code)]

use super::traits::*;
use super::{Invoice, InvoicePermissions, NewInvoicePermissions, User};
use anyhow::Result;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewInvoicePermissionsBuilder {
    borrower_id: Option<i64>,
    invoice_id: Option<i64>,
    read_access: bool,
    write_access: bool,
}

impl Builder<NewInvoicePermissions> for NewInvoicePermissionsBuilder {
    type Output = NewInvoicePermissions;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(borrower_id), Some(invoice_id)) = (self.borrower_id, self.invoice_id) {
            Some(Self::Output {
                borrower_id,
                invoice_id,
                read_access: self.read_access,
                write_access: self.read_access,
            })
        } else {
            None
        }
    }
}

impl NewInvoicePermissionsBuilder {
    pub fn borrower(&mut self, borrower: &User) -> &mut Self {
        self.borrower_id = Some(borrower.user_id);
        self
    }

    pub fn borrower_id(&mut self, borrower_id: i64) -> &mut Self {
        self.borrower_id = Some(borrower_id);
        self
    }

    pub fn invoice(&mut self, invoice: &Invoice) -> &mut Self {
        self.invoice_id = Some(invoice.invoice_id);
        self
    }

    pub fn invoice_id(&mut self, invoice_id: i64) -> &mut Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn read_access(&mut self, access: bool) -> &mut Self {
        self.read_access = access;
        self
    }

    pub fn write_access(&mut self, access: bool) -> &mut Self {
        self.write_access = access;
        self
    }
}

impl HasBuilder<NewInvoicePermissionsBuilder, Self> for NewInvoicePermissions {}
impl NewInvoicePermissions {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<InvoicePermissions> {
        todo!()
    }
}

impl HasBuilder<NewInvoicePermissionsBuilder, NewInvoicePermissions> for InvoicePermissions {}
