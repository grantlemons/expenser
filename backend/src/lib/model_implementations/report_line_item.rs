#![allow(dead_code)]

use super::traits::*;
use super::{NewReportLineItem, Report, ReportLineItem};
use anyhow::Result;
use diesel::prelude::*;
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
    pub fn insert(&self, conn: &mut PgConnection) -> Result<ReportLineItem> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::insert_into(dsl::report_line_items)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl<'a> HasBuilder<NewReportLineItemBuilder<'a>, NewReportLineItem<'a>> for ReportLineItem {}
impl<'a> ReportLineItem {
    pub fn delete(id: i64, conn: &mut PgConnection) -> Result<usize> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::delete(dsl::report_line_items.filter(dsl::id.eq(id))).execute(conn)?;

        Ok(res)
    }

    fn update_using_cents(
        &self,
        report_id: i64,
        name: &'a str,
        price_usd: diesel::data_types::Cents,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::update(self)
            .set((
                dsl::report_id.eq(report_id),
                dsl::item_name.eq(name),
                dsl::item_price_usd.eq(price_usd),
            ))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update(
        &self,
        report_id: i64,
        name: &'a str,
        price_usd: i64,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use diesel::data_types::Cents;
        let price_usd_cents = Cents(price_usd * 100);

        self.update_using_cents(report_id, name, price_usd_cents, conn)
    }

    pub fn replace(&self, new: &NewReportLineItem, conn: &mut PgConnection) -> Result<Self> {
        self.update_using_cents(new.report_id, new.item_name, new.item_price_usd, conn)
    }

    pub fn update_report_id(&self, report_id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::update(self)
            .set(dsl::report_id.eq(report_id))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_report(&self, report: &Report, conn: &mut PgConnection) -> Result<Self> {
        self.update_report_id(report.id, conn)
    }

    pub fn update_item_name(&self, name: &'a str, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::update(self)
            .set(dsl::item_name.eq(name))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_item_price_usd_cents(
        &self,
        price_usd: diesel::data_types::Cents,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::update(self)
            .set(dsl::item_price_usd.eq(price_usd))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_item_price_usd(&self, price_usd: i64, conn: &mut PgConnection) -> Result<Self> {
        use diesel::data_types::Cents;
        let price_usd_cents = Cents(price_usd * 100);

        self.update_item_price_usd_cents(price_usd_cents, conn)
    }
}
