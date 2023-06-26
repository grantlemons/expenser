#![allow(dead_code)]

use super::traits::*;
use super::{NewReportProof, Report, ReportProof};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportProofBuilder {
    report_id: Option<i64>,
    data: Option<Vec<u8>>,
}

impl Builder<NewReportProof> for NewReportProofBuilder {
    type Output = NewReportProof;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(report_id), Some(data)) = (self.report_id, self.data.clone()) {
            Some(Self::Output { report_id, data })
        } else {
            None
        }
    }
}

impl NewReportProofBuilder {
    pub fn report(&mut self, report: &Report) -> &mut Self {
        self.report_id = Some(report.id);
        self
    }

    pub fn report_id(&mut self, report_id: i64) -> &mut Self {
        self.report_id = Some(report_id);
        self
    }

    pub fn data(&mut self, data: Vec<u8>) -> &mut Self {
        self.data = Some(data);
        self
    }
}

impl HasBuilder<NewReportProofBuilder, Self> for NewReportProof {}
impl NewReportProof {
    pub fn insert(&self, conn: &mut PgConnection) -> Result<ReportProof> {
        use crate::schema::report_proof::dsl;

        let res = diesel::insert_into(dsl::report_proof)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl HasBuilder<NewReportProofBuilder, NewReportProof> for ReportProof {}
impl ReportProof {
    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::report_proof::dsl;

        diesel::delete(dsl::report_proof).execute(conn)?;

        Ok(())
    }

    pub fn clear_by_report(report_id: i64, conn: &mut PgConnection) -> Result<()> {
        use crate::schema::report_proof::dsl;

        diesel::delete(dsl::report_proof.filter(dsl::report_id.eq(report_id))).execute(conn)?;

        Ok(())
    }

    pub fn get_by_id(id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_proof::dsl;

        let res = dsl::report_proof.filter(dsl::id.eq(id)).first(conn)?;

        Ok(res)
    }

    pub fn get_by_report(report_id: i64, conn: &mut PgConnection) -> Result<Vec<Self>> {
        use crate::schema::report_proof::dsl;

        let res = dsl::report_proof
            .filter(dsl::report_id.eq(report_id))
            .select(Self::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn get_by_path(path_ids: (i64, i64), conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_proof::dsl;

        let res = dsl::report_proof
            .filter(dsl::report_id.eq(path_ids.0))
            .filter(dsl::id.eq(path_ids.1))
            .first(conn)?;

        Ok(res)
    }

    pub fn delete(path_ids: (i64, i64), conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_proof::dsl;

        let res = diesel::delete(
            dsl::report_proof
                .filter(dsl::report_id.eq(path_ids.0))
                .filter(dsl::id.eq(path_ids.1)),
        )
        .get_result(conn)?;

        Ok(res)
    }

    pub fn update(
        path_ids: (i64, i64),
        report_id: i64,
        data: &[u8],
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::report_proof::dsl;

        let res = diesel::update(
            dsl::report_proof
                .filter(dsl::report_id.eq(path_ids.0))
                .filter(dsl::id.eq(path_ids.1)),
        )
        .set((dsl::report_id.eq(report_id), dsl::data.eq(data)))
        .get_result(conn)?;

        Ok(res)
    }

    pub fn replace(
        path_ids: (i64, i64),
        new: &NewReportProof,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        Self::update(path_ids, new.report_id, &new.data, conn)
    }
}
