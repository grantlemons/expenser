#![allow(dead_code)]

use super::traits::*;
use super::{NewReport, Report, User};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportBuilder {
    owner_id: Option<i64>,
    title: Option<String>,
    description: Option<String>,
}

impl Builder<NewReport> for NewReportBuilder {
    type Output = NewReport;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(owner_id), Some(title)) = (self.owner_id, self.title.clone()) {
            Some(Self::Output {
                owner_id,
                title,
                description: self.description.clone(),
            })
        } else {
            None
        }
    }
}

impl NewReportBuilder {
    pub fn owner(&mut self, owner: &User) -> &mut Self {
        self.owner_id = Some(owner.id);
        self
    }

    pub fn owner_id(&mut self, owner_id: i64) -> &mut Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }
}

impl HasBuilder<NewReportBuilder, Self> for NewReport {}
impl NewReport {
    pub fn insert(&self, conn: &mut PgConnection) -> Result<Report> {
        use crate::schema::reports::dsl;

        let res = diesel::insert_into(dsl::reports)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl HasBuilder<NewReportBuilder, NewReport> for Report {}
impl Report {
    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::reports::dsl;

        diesel::delete(dsl::reports).execute(conn)?;

        Ok(())
    }

    pub fn get_by_id(id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::reports::dsl;

        let res = dsl::reports.filter(dsl::id.eq(id)).first(conn)?;

        Ok(res)
    }

    pub fn get_by_owner(owner_id: i64, conn: &mut PgConnection) -> Result<Vec<Self>> {
        use crate::schema::reports::dsl;

        let res = dsl::reports
            .filter(dsl::owner_id.eq(owner_id))
            .select(Self::as_select())
            .load(conn)?;

        Ok(res)
    }

    pub fn delete(id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::reports::dsl;

        let res = diesel::delete(dsl::reports.filter(dsl::id.eq(id))).get_result(conn)?;

        Ok(res)
    }

    pub fn update(
        id: i64,
        owner_id: i64,
        title: String,
        description: Option<String>,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::reports::dsl;

        let res = diesel::update(dsl::reports.filter(dsl::id.eq(id)))
            .set((
                dsl::owner_id.eq(owner_id),
                dsl::title.eq(title),
                dsl::description.eq(description),
            ))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn replace(id: i64, new: &NewReport, conn: &mut PgConnection) -> Result<Self> {
        Self::update(
            id,
            new.owner_id,
            new.title.clone(),
            new.description.clone(),
            conn,
        )
    }
}
