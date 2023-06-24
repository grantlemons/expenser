#![allow(dead_code)]

use super::traits::*;
use super::{NewReport, Report, User};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Default, Debug)]
pub struct NewReportBuilder<'a> {
    owner_id: Option<i64>,
    title: Option<&'a str>,
    description: Option<&'a str>,
}

impl<'a> Builder<NewReport<'a>> for NewReportBuilder<'a> {
    type Output = NewReport<'a>;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(owner_id), Some(title)) = (self.owner_id, self.title) {
            Some(Self::Output {
                owner_id,
                title,
                description: self.description,
            })
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
    pub fn insert(&self, conn: &mut PgConnection) -> Result<Report> {
        use crate::schema::reports::dsl;

        let res = diesel::insert_into(dsl::reports)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl<'a> HasBuilder<NewReportBuilder<'a>, NewReport<'a>> for Report {}
impl<'a> Report {
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

    pub fn delete(id: i64, conn: &mut PgConnection) -> Result<usize> {
        use crate::schema::reports::dsl;

        let res = diesel::delete(dsl::reports.filter(dsl::id.eq(id))).execute(conn)?;

        Ok(res)
    }

    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::reports::dsl;

        diesel::delete(dsl::reports).execute(conn)?;

        Ok(())
    }

    pub fn update(
        &self,
        owner_id: i64,
        title: &'a str,
        description: Option<&'a str>,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::reports::dsl;

        let res = diesel::update(self)
            .set((
                dsl::owner_id.eq(owner_id),
                dsl::title.eq(title),
                dsl::description.eq(description),
            ))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn replace(&self, new: &NewReport, conn: &mut PgConnection) -> Result<Self> {
        self.update(new.owner_id, new.title, new.description, conn)
    }

    pub fn update_owner_id(&self, owner_id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::reports::dsl;

        let res = diesel::update(self)
            .set(dsl::owner_id.eq(owner_id))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_owner(&self, owner: &User, conn: &mut PgConnection) -> Result<Self> {
        self.update_owner_id(owner.id, conn)
    }

    pub fn update_title(&self, title: &'a str, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::reports::dsl;

        let res = diesel::update(self)
            .set(dsl::title.eq(title))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_description(
        &self,
        description: &'a str,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::reports::dsl;

        let res = diesel::update(self)
            .set(dsl::description.eq(Some(description)))
            .get_result(conn)?;

        Ok(res)
    }
}
