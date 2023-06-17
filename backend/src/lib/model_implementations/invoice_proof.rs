#![allow(dead_code)]

use super::traits::*;
use super::{Invoice, InvoiceProof, NewInvoiceProof};
use anyhow::Result;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewInvoiceProofBuilder<'a> {
    invoice_id: Option<i64>,
    data: Option<&'a [u8]>,
}

impl<'a> Builder<NewInvoiceProof<'a>> for NewInvoiceProofBuilder<'a> {
    type Output = NewInvoiceProof<'a>;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(invoice_id), Some(data)) = (self.invoice_id, self.data) {
            Some(Self::Output { invoice_id, data })
        } else {
            None
        }
    }
}

impl<'a> NewInvoiceProofBuilder<'a> {
    pub fn invoice(&'a mut self, invoice: Invoice) -> &'a mut Self {
        self.invoice_id = Some(invoice.invoice_id);
        self
    }

    pub fn invoice_id(&'a mut self, invoice_id: i64) -> &'a mut Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn data(&'a mut self, data: &'a [u8]) -> &'a mut Self {
        self.data = Some(data);
        self
    }
}

impl<'a> HasBuilder<NewInvoiceProofBuilder<'a>, Self> for NewInvoiceProof<'a> {}
impl<'a> NewInvoiceProof<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<InvoiceProof> {
        todo!()
    }
}

impl<'a> HasBuilder<NewInvoiceProofBuilder<'a>, NewInvoiceProof<'a>> for InvoiceProof {}
