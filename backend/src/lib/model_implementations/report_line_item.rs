#![allow(dead_code)]

use super::traits::*;
use super::{NewReportLineItem, Report, ReportLineItem};
use anyhow::Result;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportLineItemBuilder<'a> {
    report_id: Option<i64>,
    item_name: Option<&'a str>,
    item_price_usd: Option<diesel::data_types::Cents>,
}

impl<'a> Builder<NewReportLineItem<'a>> for NewReportLineItemBuilder<'a> {
    type Output = NewReportLineItem<'a>;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(report_id), Some(item_name), Some(item_price_usd)) =
            (self.report_id, self.item_name, self.item_price_usd)
        {
            Some(Self::Output {
                report_id,
                item_name,
                item_price_usd,
            })
        } else {
            None
        }
    }
}

impl<'a> NewReportLineItemBuilder<'a> {
    pub fn report(&mut self, report: &Report) -> &mut Self {
        self.report_id = Some(report.id);
        self
    }

    pub fn report_id(&mut self, report_id: i64) -> &mut Self {
        self.report_id = Some(report_id);
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

impl<'a> HasBuilder<NewReportLineItemBuilder<'a>, Self> for NewReportLineItem<'a> {}
impl<'a> NewReportLineItem<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<ReportLineItem> {
        todo!()
    }
}

impl<'a> HasBuilder<NewReportLineItemBuilder<'a>, NewReportLineItem<'a>> for ReportLineItem {}
