#![allow(dead_code)]

use super::traits::*;
use super::{NewReport, Report, User};
use anyhow::Result;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportBuilder<'a> {
    owner_id: Option<i64>,
    title: Option<&'a str>,
}

impl<'a> Builder<NewReport<'a>> for NewReportBuilder<'a> {
    type Output = NewReport<'a>;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(owner_id), Some(title)) = (self.owner_id, self.title) {
            Some(Self::Output { owner_id, title })
        } else {
            None
        }
    }
}

impl<'a> NewReportBuilder<'a> {
    pub fn owner(&'a mut self, owner: &User) -> &'a mut Self {
        self.owner_id = Some(owner.id);
        self
    }

    pub fn owner_id(&'a mut self, owner_id: i64) -> &'a mut Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn title(&'a mut self, title: &'a str) -> &'a mut Self {
        self.title = Some(title);
        self
    }
}

impl<'a> HasBuilder<NewReportBuilder<'a>, Self> for NewReport<'a> {}
impl<'a> NewReport<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<Report> {
        todo!()
    }
}

impl<'a> HasBuilder<NewReportBuilder<'a>, NewReport<'a>> for Report {}
