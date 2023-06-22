#![allow(dead_code)]

use super::traits::*;
use super::{NewReportProof, Report, ReportProof};
use anyhow::Result;
use diesel::prelude::*;
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
    pub fn insert(&self, conn: &mut PgConnection) -> Result<ReportProof> {
        use crate::schema::report_proof::dsl;

        let res = diesel::insert_into(dsl::report_proof)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl<'a> HasBuilder<NewReportProofBuilder<'a>, NewReportProof<'a>> for ReportProof {}
impl<'a> ReportProof {
    pub fn delete(id: i64, conn: &mut PgConnection) -> Result<usize> {
        use crate::schema::report_proof::dsl;

        let res = diesel::delete(dsl::report_proof.filter(dsl::id.eq(id))).execute(conn)?;

        Ok(res)
    }

    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::report_proof::dsl;

        diesel::delete(dsl::report_proof).execute(conn)?;

        Ok(())
    }

    pub fn update(&self, report_id: i64, data: &'a [u8], conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_proof::dsl;

        let res = diesel::update(self)
            .set((dsl::report_id.eq(report_id), dsl::data.eq(data)))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn replace(&self, new: &NewReportProof, conn: &mut PgConnection) -> Result<Self> {
        self.update(new.report_id, new.data, conn)
    }

    pub fn update_report_id(&self, report_id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_proof::dsl;

        let res = diesel::update(self)
            .set(dsl::report_id.eq(report_id))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_report(&self, report: &Report, conn: &mut PgConnection) -> Result<Self> {
        self.update_report_id(report.id, conn)
    }

    pub fn update_data(&self, data: &'a [u8], conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_proof::dsl;

        let res = diesel::update(self)
            .set(dsl::data.eq(data))
            .get_result(conn)?;

        Ok(res)
    }
}
