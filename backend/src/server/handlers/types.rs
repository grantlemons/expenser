use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
/// Information on version and other fields set in the cargo manifest
pub struct CrateInfo {
    pub name: &'static str,
    pub authors: Vec<&'static str>,
    pub version: &'static str,
    pub description: &'static str,
    pub license: &'static str,
    pub repository: &'static str,
}

#[derive(Serialize, Debug)]
pub struct ReportLineItemSerde {
    id: i64,
    report_id: i64,
    item_name: String,
    item_price_usd: f64,
}

impl From<ReportLineItemSerde> for expenser::ReportLineItem {
    fn from(value: ReportLineItemSerde) -> Self {
        use diesel::data_types::Cents;
        let price_usd_cents = Cents((value.item_price_usd * 100.0).trunc() as i64);

        Self {
            id: value.id,
            report_id: value.report_id,
            item_name: value.item_name,
            item_price_usd: price_usd_cents,
        }
    }
}

impl From<expenser::ReportLineItem> for ReportLineItemSerde {
    fn from(value: expenser::ReportLineItem) -> Self {
        let price_usd = value.item_price_usd.0 as f64 / 100.0;

        Self {
            id: value.id,
            report_id: value.report_id,
            item_name: value.item_name,
            item_price_usd: price_usd,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct NewReportLineItemSerde {
    report_id: i64,
    item_name: String,
    item_price_usd: f64,
}

impl From<NewReportLineItemSerde> for expenser::NewReportLineItem {
    fn from(value: NewReportLineItemSerde) -> Self {
        use diesel::data_types::Cents;
        let price_usd_cents = Cents((value.item_price_usd * 100.0).trunc() as i64);

        Self {
            report_id: value.report_id,
            item_name: value.item_name,
            item_price_usd: price_usd_cents,
        }
    }
}

impl From<expenser::NewReportLineItem> for NewReportLineItemSerde {
    fn from(value: expenser::NewReportLineItem) -> Self {
        let price_usd = value.item_price_usd.0 as f64 / 100.0;

        Self {
            report_id: value.report_id,
            item_name: value.item_name,
            item_price_usd: price_usd,
        }
    }
}
