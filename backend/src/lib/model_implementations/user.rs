#![allow(dead_code)]

use super::traits::*;
use super::{NewUser, User};
use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

fn hash_password(_password: &str) -> String {
    todo!()
}

#[derive(Default, Debug)]
pub struct NewUserBuilder {
    username: Option<String>,
    email: Option<String>,
    profile_picture: Option<Vec<u8>>,
    password_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    email: String,
}

impl From<User> for UserInfo {
    fn from(value: User) -> Self {
        Self {
            username: value.username,
            email: value.email,
        }
    }
}

impl Builder<NewUser> for NewUserBuilder {
    type Output = NewUser;

    fn build(&self) -> Option<Self::Output> {
        if let (Some(username), Some(email), Some(password_hash)) = (
            self.username.clone(),
            self.email.clone(),
            self.password_hash.clone(),
        ) {
            Some(Self::Output {
                username,
                email,
                profile_picture: self.profile_picture.clone(),
                password_hash,
            })
        } else {
            None
        }
    }
}
impl NewUserBuilder {
    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    pub fn profile_picture(&mut self, profile_picture: Vec<u8>) -> &mut Self {
        self.profile_picture = Some(profile_picture);
        self
    }

    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password_hash = Some(hash_password(password));
        self
    }
}

impl HasBuilder<NewUserBuilder, Self> for NewUser {}
impl NewUser {
    pub fn insert(&self, conn: &mut PgConnection) -> Result<UserInfo> {
        use crate::schema::users::dsl;

        let res = diesel::insert_into(dsl::users)
            .values(self)
            .get_result::<User>(conn)?;

        Ok(res.into())
    }
}

impl HasBuilder<NewUserBuilder, NewUser> for User {}
impl User {
    pub fn get_by_id(id: i64, conn: &mut PgConnection) -> Result<UserInfo> {
        use crate::schema::users::dsl;

        let res = dsl::users.filter(dsl::id.eq(id)).first::<User>(conn)?;

        Ok(res.into())
    }

    pub fn get_profile_picture(id: i64, conn: &mut PgConnection) -> Result<axum::body::Bytes> {
        use crate::schema::users::dsl;

        let res: User = dsl::users.filter(dsl::id.eq(id)).first::<User>(conn)?;
        let profile_picture = res.profile_picture.unwrap_or_default();
        let bytes = axum::body::Bytes::copy_from_slice(&profile_picture);

        Ok(bytes)
    }

    pub fn delete(id: i64, conn: &mut PgConnection) -> Result<UserInfo> {
        use crate::schema::users::dsl;

        let res = diesel::delete(dsl::users.filter(dsl::id.eq(id))).get_result::<User>(conn)?;

        Ok(res.into())
    }

    pub fn clear(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::users::dsl;

        diesel::delete(dsl::users).execute(conn)?;

        Ok(())
    }

    fn update_hash(
        id: i64,
        username: String,
        email: String,
        profile_picture: Option<Vec<u8>>,
        password_hash: String,
        conn: &mut PgConnection,
    ) -> Result<UserInfo> {
        use crate::schema::users::dsl;

        let res = diesel::update(dsl::users.filter(dsl::id.eq(id)))
            .set((
                dsl::username.eq(username),
                dsl::email.eq(email),
                dsl::profile_picture.eq(profile_picture),
                dsl::password_hash.eq(password_hash),
            ))
            .get_result::<User>(conn)?;

        Ok(res.into())
    }

    pub fn update_info(
        id: i64,
        username: &str,
        email: &str,
        conn: &mut PgConnection,
    ) -> Result<UserInfo> {
        use crate::schema::users::dsl;

        let res = diesel::update(dsl::users.filter(dsl::id.eq(id)))
            .set((dsl::username.eq(username), dsl::email.eq(email)))
            .get_result::<User>(conn)?;

        Ok(res.into())
    }

    pub fn update(
        id: i64,
        username: String,
        email: String,
        profile_picture: Option<Vec<u8>>,
        password: String,
        conn: &mut PgConnection,
    ) -> Result<UserInfo> {
        let password_hash = hash_password(&password);

        Self::update_hash(id, username, email, profile_picture, password_hash, conn)
    }

    pub fn replace(id: i64, new: &NewUser, conn: &mut PgConnection) -> Result<UserInfo> {
        Self::update_hash(
            id,
            new.username.clone(),
            new.email.clone(),
            new.profile_picture.clone(),
            new.password_hash.clone(),
            conn,
        )
    }

    pub fn update_profile_picture(
        id: i64,
        profile_picture: &[u8],
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = diesel::update(dsl::users.filter(dsl::id.eq(id)))
            .set(dsl::profile_picture.eq(Some(profile_picture)))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_password_hash(
        id: i64,
        password_hash: &str,
        conn: &mut PgConnection,
    ) -> Result<Self> {
        use crate::schema::users::dsl;

        let res = diesel::update(dsl::users.filter(dsl::id.eq(id)))
            .set(dsl::password_hash.eq(password_hash))
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update_password(id: i64, password: String, conn: &mut PgConnection) -> Result<Self> {
        let password_hash = hash_password(&password);
        Self::update_password_hash(id, &password_hash, conn)
    }
}
