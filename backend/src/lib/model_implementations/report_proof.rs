#![allow(dead_code)]

use super::traits::*;
use super::{NewReportProof, Report, ReportProof};
use anyhow::Result;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportProofBuilder<'a> {
    report_id: Option<i64>,
    data: Option<&'a [u8]>,
}

impl<'a> Builder<NewReportProof<'a>> for NewReportProofBuilder<'a> {
    type Output = NewReportProof<'a>;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(report_id), Some(data)) = (self.report_id, self.data) {
            Some(Self::Output { report_id, data })
        } else {
            None
        }
    }
}

impl<'a> NewReportProofBuilder<'a> {
    pub fn report(&'a mut self, report: &Report) -> &'a mut Self {
        self.report_id = Some(report.id);
        self
    }

    pub fn report_id(&'a mut self, report_id: i64) -> &'a mut Self {
        self.report_id = Some(report_id);
        self
    }

    pub fn data(&'a mut self, data: &'a [u8]) -> &'a mut Self {
        self.data = Some(data);
        self
    }
}

impl<'a> HasBuilder<NewReportProofBuilder<'a>, Self> for NewReportProof<'a> {}
impl<'a> NewReportProof<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<ReportProof> {
        todo!()
    }
}

impl<'a> HasBuilder<NewReportProofBuilder<'a>, NewReportProof<'a>> for ReportProof {}
