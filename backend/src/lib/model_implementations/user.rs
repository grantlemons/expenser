#![allow(dead_code)]

use super::traits::*;
use super::{NewUser, User};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

fn hash_password(_password: &str) -> &str {
    todo!()
}

#[derive(Default, Debug)]
pub struct NewUserBuilder<'a> {
    username: Option<&'a str>,
    email: Option<&'a str>,
    profile_picture: Option<&'a [u8]>,
    password_hash: Option<&'a str>,
}

impl<'a> Builder<NewUser<'a>> for NewUserBuilder<'a> {
    type Output = NewUser<'a>;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(username), Some(email), Some(password_hash)) =
            (self.username, self.email, self.password_hash)
        {
            Some(Self::Output {
                username,
                email,
                profile_picture: self.profile_picture,
                password_hash,
            })
        } else {
            None
        }
    }
}
impl<'a> NewUserBuilder<'a> {
    pub fn username(&'a mut self, username: &'a str) -> &'a mut Self {
        self.username = Some(username);
        self
    }

    pub fn email(&'a mut self, email: &'a str) -> &'a mut Self {
        self.email = Some(email);
        self
    }

    pub fn profile_picture(&'a mut self, profile_picture: &'a [u8]) -> &'a mut Self {
        self.profile_picture = Some(profile_picture);
        self
    }

    pub fn password(&'a mut self, password: &'a str) -> &'a mut Self {
        self.password_hash = Some(hash_password(password));
        self
    }
}

impl<'a> HasBuilder<NewUserBuilder<'a>, Self> for NewUser<'a> {}
impl<'a> NewUser<'a> {
    pub fn insert(&self, conn: &mut PgConnection) -> Result<User> {
        use crate::schema::users::dsl;

        let res = diesel::insert_into(dsl::users)
            .values(self)
            .get_result(conn)?;

        Ok(res)
    }
}

impl<'a> HasBuilder<NewUserBuilder<'a>, NewUser<'a>> for User {}
impl<'a> User {
    pub fn get_by_id(id: i64, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = dsl::users.filter(dsl::id.eq(id)).first(conn)?;

        Ok(res)
    }

    pub fn delete(id: i64, conn: &mut PgConnection) -> Result<usize> {
        use crate::schema::users::dsl;

        let res = diesel::delete(dsl::users.filter(dsl::id.eq(id))).execute(conn)?;

        Ok(res)
    }

    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::users::dsl;

        diesel::delete(dsl::users).execute(conn)?;

        Ok(())
    }

    fn update_hash(
        &self,
        username: &'a str,
        email: &'a str,
        profile_picture: Option<&'a [u8]>,
        password_hash: &'a str,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = diesel::update(self)
            .set((
                dsl::username.eq(username),
                dsl::email.eq(email),
                dsl::profile_picture.eq(profile_picture),
                dsl::password_hash.eq(password_hash),
            ))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update(
        &self,
        username: &'a str,
        email: &'a str,
        profile_picture: Option<&'a [u8]>,
        password: &'a str,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        let password_hash = hash_password(password);
        self.update_hash(username, email, profile_picture, password_hash, conn)
    }

    pub fn replace(&self, new: &NewUser, conn: &mut PgConnection) -> Result<Self> {
        self.update_hash(
            new.username,
            new.email,
            new.profile_picture,
            new.password_hash,
            conn,
        )
    }

    pub fn update_username(&self, username: &'a str, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = diesel::update(self)
            .set(dsl::username.eq(username))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_email(&self, email: &'a str, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = diesel::update(self)
            .set(dsl::email.eq(email))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_profile_picture(
        &self,
        profile_picture: &'a [u8],
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = diesel::update(self)
            .set(dsl::profile_picture.eq(Some(profile_picture)))
            .get_result(conn)?;

        Ok(res)
    }

    fn update_password_hash(
        &self,
        password_hash: &'a str,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = diesel::update(self)
            .set(dsl::password_hash.eq(password_hash))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_password(&self, password: &'a str, conn: &mut PgConnection) -> Result<Self> {
        let password_hash = hash_password(password);
        self.update_password_hash(password_hash, conn)
    }
}
