#![allow(dead_code)]

use super::traits::*;
use super::{NewReportLineItem, Report, ReportLineItem};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportLineItemBuilder {
    report_id: Option<i64>,
    item_name: Option<String>,
    item_price_usd: Option<diesel::data_types::Cents>,
}

impl Builder<NewReportLineItem> for NewReportLineItemBuilder {
    type Output = NewReportLineItem;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(report_id), Some(item_name), Some(item_price_usd)) =
            (self.report_id, self.item_name.clone(), self.item_price_usd)
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

impl NewReportLineItemBuilder {
    pub fn report(&mut self, report: &Report) -> &mut Self {
        self.report_id = Some(report.id);
        self
    }

    pub fn report_id(&mut self, report_id: i64) -> &mut Self {
        self.report_id = Some(report_id);
        self
    }

    pub fn item_name(&mut self, item_name: String) -> &mut Self {
        self.item_name = Some(item_name);
        self
    }

    pub fn item_price_usd(&mut self, price_usd: f64) -> &mut Self {
        use diesel::data_types::Cents;

        let price_usd_cents = Cents((price_usd * 100.0).trunc() as i64);

        self.item_price_usd = Some(price_usd_cents);
        self
    }
}

impl HasBuilder<NewReportLineItemBuilder, Self> for NewReportLineItem {}
impl NewReportLineItem {
    pub fn insert(&self, conn: &mut PgConnection) -> Result<ReportLineItem> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::insert_into(dsl::report_line_items)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl HasBuilder<NewReportLineItemBuilder, NewReportLineItem> for ReportLineItem {}
impl ReportLineItem {
    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::report_line_items::dsl;

        diesel::delete(dsl::report_line_items).execute(conn)?;

        Ok(())
    }

    pub fn clear_by_report(report_id: i64, conn: &mut PgConnection) -> Result<()> {
        use crate::schema::report_line_items::dsl;

        diesel::delete(dsl::report_line_items.filter(dsl::report_id.eq(report_id)))
            .execute(conn)?;

        Ok(())
    }

    pub fn get_by_id(id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = dsl::report_line_items.filter(dsl::id.eq(id)).first(conn)?;

        Ok(res)
    }

    pub fn get_by_report(report_id: i64, conn: &mut PgConnection) -> Result<Vec<Self>> {
        use crate::schema::report_line_items::dsl;

        let res = dsl::report_line_items
            .filter(dsl::report_id.eq(report_id))
            .select(Self::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn get_by_path(path_ids: (i64, i64), conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = dsl::report_line_items
            .filter(dsl::report_id.eq(path_ids.0))
            .filter(dsl::id.eq(path_ids.1))
            .first(conn)?;

        Ok(res)
    }

    pub fn delete(path_ids: (i64, i64), conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::delete(
            dsl::report_line_items
                .filter(dsl::report_id.eq(path_ids.0))
                .filter(dsl::id.eq(path_ids.1)),
        )
        .get_result(conn)?;

        Ok(res)
    }

    fn update_using_cents(
        path_ids: (i64, i64),
        report_id: i64,
        name: &str,
        price_usd: diesel::data_types::Cents,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::report_line_items::dsl;

        let res = diesel::update(
            dsl::report_line_items
                .filter(dsl::report_id.eq(path_ids.0))
                .filter(dsl::id.eq(path_ids.1)),
        )
        .set((
            dsl::report_id.eq(report_id),
            dsl::item_name.eq(name),
            dsl::item_price_usd.eq(price_usd),
        ))
        .get_result(conn)?;

        Ok(res)
    }

    pub fn update(
        path_ids: (i64, i64),
        report_id: i64,
        name: &str,
        price_usd: f64,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use diesel::data_types::Cents;
        let price_usd_cents = Cents((price_usd * 100.0).trunc() as i64);

        Self::update_using_cents(path_ids, report_id, name, price_usd_cents, conn)
    }

    pub fn replace(
        path_ids: (i64, i64),
        new: &NewReportLineItem,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        Self::update_using_cents(
            path_ids,
            new.report_id,
            &new.item_name,
            new.item_price_usd,
            conn,
        )
    }
}
