#![allow(dead_code)]

use super::traits::*;
use super::{Invoice, NewInvoice, User};
use anyhow::Result;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewInvoiceBuilder<'a> {
    owner_id: Option<i64>,
    title: Option<&'a str>,
}

impl<'a> Builder<NewInvoice<'a>> for NewInvoiceBuilder<'a> {
    type Output = NewInvoice<'a>;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(owner_id), Some(title)) = (self.owner_id, self.title) {
            Some(Self::Output { owner_id, title })
        } else {
            None
        }
    }
}

impl<'a> NewInvoiceBuilder<'a> {
    pub fn owner(&'a mut self, owner: &User) -> &'a mut Self {
        self.owner_id = Some(owner.user_id);
        self
    }

    pub fn owner_id(&'a mut self, owner_id: i64) -> &'a mut Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn title(&'a mut self, title: &'a str) -> &'a mut Self {
        self.title = Some(title);
        self
    }
}

impl<'a> HasBuilder<NewInvoiceBuilder<'a>, Self> for NewInvoice<'a> {}
impl<'a> NewInvoice<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<Invoice> {
        todo!()
    }
}

impl<'a> HasBuilder<NewInvoiceBuilder<'a>, NewInvoice<'a>> for Invoice {}
