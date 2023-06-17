use super::{Invoice, InvoiceLineItem, NewInvoiceLineItem};
use anyhow::Result;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewInvoiceLineItemBuilder<'a> {
    invoice_id: Option<i64>,
    item_name: Option<&'a str>,
    item_price_usd: Option<diesel::data_types::Cents>,
}

impl<'a> NewInvoiceLineItemBuilder<'a> {
    pub fn invoice(&mut self, invoice: Invoice) -> &mut Self {
        self.invoice_id = Some(invoice.invoice_id);
        self
    }

    pub fn invoice_id(&mut self, invoice_id: i64) -> &mut Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    pub fn item_name(&mut self, item_name: &'a str) -> &mut Self {
        self.item_name = Some(item_name);
        self
    }

    pub fn item_price_usd(&mut self, price_usd: i64) -> &mut Self {
        use diesel::data_types::Cents;

        let price_usd_cents = Cents(price_usd * 100);

        self.item_price_usd = Some(price_usd_cents);
        self
    }
}

impl<'a> NewInvoiceLineItem<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<InvoiceLineItem> {
        todo!()
    }
}

impl InvoiceLineItem {
    pub fn builder<'a>() -> NewInvoiceLineItemBuilder<'a> {
        NewInvoiceLineItemBuilder::default()
    }
}
