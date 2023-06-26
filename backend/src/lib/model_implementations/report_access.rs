#![allow(dead_code)]

use super::traits::*;
use super::{NewReportAccess, Report, ReportAccess, User};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportAccessBuilder {
    borrower_id: Option<i64>,
    report_id: Option<i64>,
    read_access: bool,
    write_access: bool,
}

impl Builder<NewReportAccess> for NewReportAccessBuilder {
    type Output = NewReportAccess;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(borrower_id), Some(report_id)) = (self.borrower_id, self.report_id) {
            Some(Self::Output {
                borrower_id,
                report_id,
                read_access: self.read_access,
                write_access: self.read_access,
            })
        } else {
            None
        }
    }
}

impl NewReportAccessBuilder {
    pub fn borrower(&mut self, borrower: &User) -> &mut Self {
        self.borrower_id = Some(borrower.id);
        self
    }

    pub fn borrower_id(&mut self, borrower_id: i64) -> &mut Self {
        self.borrower_id = Some(borrower_id);
        self
    }

    pub fn report(&mut self, report: &Report) -> &mut Self {
        self.report_id = Some(report.id);
        self
    }

    pub fn report_id(&mut self, report_id: i64) -> &mut Self {
        self.report_id = Some(report_id);
        self
    }

    pub fn read_access(&mut self, access: bool) -> &mut Self {
        self.read_access = access;
        self
    }

    pub fn write_access(&mut self, access: bool) -> &mut Self {
        self.write_access = access;
        self
    }
}

impl HasBuilder<NewReportAccessBuilder, Self> for NewReportAccess {}
impl NewReportAccess {
    pub fn insert(&self, conn: &mut PgConnection) -> Result<ReportAccess> {
        use crate::schema::report_access::dsl;

        let res = diesel::insert_into(dsl::report_access)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl HasBuilder<NewReportAccessBuilder, NewReportAccess> for ReportAccess {}
impl ReportAccess {
    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::report_access::dsl;

        diesel::delete(dsl::report_access).execute(conn)?;

        Ok(())
    }

    pub fn clear_by_report(report_id: i64, conn: &mut PgConnection) -> Result<()> {
        use crate::schema::report_access::dsl;

        diesel::delete(dsl::report_access.filter(dsl::report_id.eq(report_id))).execute(conn)?;

        Ok(())
    }

    pub fn get_by_path(path_ids: (i64, i64), conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_access::dsl;

        let res = dsl::report_access
            .filter(dsl::report_id.eq(path_ids.0))
            .filter(dsl::id.eq(path_ids.1))
            .first(conn)?;

        Ok(res)
    }

    pub fn get_by_id(id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_access::dsl;

        let res = dsl::report_access.filter(dsl::id.eq(id)).first(conn)?;

        Ok(res)
    }

    pub fn get_by_report(report_id: i64, conn: &mut PgConnection) -> Result<Vec<Self>> {
        use crate::schema::report_access::dsl;

        let res = dsl::report_access
            .filter(dsl::report_id.eq(report_id))
            .select(Self::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn get_by_borrower(borrower_id: i64, conn: &mut PgConnection) -> Result<Vec<Self>> {
        use crate::schema::report_access::dsl;

        let res = dsl::report_access
            .filter(dsl::borrower_id.eq(borrower_id))
            .select(Self::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn get_by_view_access(borrower_id: i64, conn: &mut PgConnection) -> Result<Vec<Self>> {
        use crate::schema::report_access::dsl;

        let res = dsl::report_access
            .filter(dsl::borrower_id.eq(borrower_id))
            .filter(dsl::read_access.eq(true))
            .select(Self::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn get_report_by_borrower(
        borrower_id: i64,
        conn: &mut PgConnection,
    ) -> Result<Vec<Report>> {
        use crate::schema::reports::dsl;

        let report_ids = Self::get_by_borrower(borrower_id, conn)?
            .into_iter()
            .map(|a| a.report_id);

        let res = dsl::reports
            .filter(dsl::id.eq_any(report_ids))
            .select(Report::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn get_report_by_read_access(
        borrower_id: i64,
        conn: &mut PgConnection,
    ) -> Result<Vec<Report>> {
        use crate::schema::reports::dsl;

        let report_ids = Self::get_by_view_access(borrower_id, conn)?
            .into_iter()
            .map(|a| a.report_id);

        let res = dsl::reports
            .filter(dsl::id.eq_any(report_ids))
            .select(Report::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn delete(path_ids: (i64, i64), conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_access::dsl;

        let res = diesel::delete(
            dsl::report_access
                .filter(dsl::report_id.eq(path_ids.0))
                .filter(dsl::id.eq(path_ids.1)),
        )
        .get_result(conn)?;

        Ok(res)
    }

    pub fn update(
        path_ids: (i64, i64),
        borrower_id: i64,
        report_id: i64,
        read_access: bool,
        write_access: bool,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::report_access::dsl;

        let res = diesel::update(
            dsl::report_access
                .filter(dsl::report_id.eq(path_ids.0))
                .filter(dsl::id.eq(path_ids.1)),
        )
        .set((
            dsl::borrower_id.eq(borrower_id),
            dsl::report_id.eq(report_id),
            dsl::read_access.eq(read_access),
            dsl::write_access.eq(write_access),
        ))
        .get_result(conn)?;

        Ok(res)
    }

    pub fn replace(
        path_ids: (i64, i64),
        new: &NewReportAccess,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        Self::update(
            path_ids,
            new.borrower_id,
            new.report_id,
            new.read_access,
            new.write_access,
            conn,
        )
    }
}
