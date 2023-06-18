#![allow(dead_code)]

use super::traits::*;
use super::{NewReportPermissions, Report, ReportPermissions, User};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportPermissionsBuilder {
    borrower_id: Option<i64>,
    report_id: Option<i64>,
    read_access: bool,
    write_access: bool,
}

impl Builder<NewReportPermissions> for NewReportPermissionsBuilder {
    type Output = NewReportPermissions;

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

impl NewReportPermissionsBuilder {
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

impl HasBuilder<NewReportPermissionsBuilder, Self> for NewReportPermissions {}
impl NewReportPermissions {
    pub fn insert(&self, conn: &mut PgConnection) -> Result<ReportPermissions> {
        use crate::schema::report_permissions::dsl;

        let res = diesel::insert_into(dsl::report_permissions)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl HasBuilder<NewReportPermissionsBuilder, NewReportPermissions> for ReportPermissions {}
impl ReportPermissions {
    pub fn delete(id: i64, conn: &mut PgConnection) -> Result<usize> {
        use crate::schema::report_permissions::dsl;

        let res = diesel::delete(dsl::report_permissions.filter(dsl::id.eq(id))).execute(conn)?;

        Ok(res)
    }

    pub fn update(
        &self,
        borrower_id: i64,
        report_id: i64,
        read_access: bool,
        write_access: bool,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::report_permissions::dsl;

        let res = diesel::update(self)
            .set((
                dsl::borrower_id.eq(borrower_id),
                dsl::report_id.eq(report_id),
                dsl::read_access.eq(read_access),
                dsl::write_access.eq(write_access),
            ))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn replace(&self, new: &NewReportPermissions, conn: &mut PgConnection) -> Result<Self> {
        self.update(
            new.borrower_id,
            new.report_id,
            new.read_access,
            new.write_access,
            conn,
        )
    }

    pub fn update_borrower_id(&self, borrower_id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_permissions::dsl;

        let res = diesel::update(self)
            .set(dsl::borrower_id.eq(borrower_id))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_borrower(&self, borrower: &User, conn: &mut PgConnection) -> Result<Self> {
        self.update_borrower_id(borrower.id, conn)
    }

    pub fn update_report_id(&self, report_id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_permissions::dsl;

        let res = diesel::update(self)
            .set(dsl::report_id.eq(report_id))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_report(&self, report: &Report, conn: &mut PgConnection) -> Result<Self> {
        self.update_report_id(report.id, conn)
    }

    pub fn update_read_access(&self, read_access: bool, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_permissions::dsl;

        let res = diesel::update(self)
            .set(dsl::read_access.eq(read_access))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_write_access(&self, write_access: bool, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::report_permissions::dsl;

        let res = diesel::update(self)
            .set(dsl::write_access.eq(write_access))
            .get_result(conn)?;

        Ok(res)
    }
}
