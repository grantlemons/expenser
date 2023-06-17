#![allow(dead_code)]

use super::traits::*;
use super::{NewReportPermissions, Report, ReportPermissions, User};
use anyhow::Result;
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
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<ReportPermissions> {
        todo!()
    }
}

impl HasBuilder<NewReportPermissionsBuilder, NewReportPermissions> for ReportPermissions {}
