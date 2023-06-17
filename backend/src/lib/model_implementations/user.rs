#![allow(dead_code)]

use super::traits::*;
use super::{NewUser, User};
use anyhow::Result;
use diesel::PgConnection;

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

    pub fn password(&'a mut self, _password: &'a str) -> &'a mut Self {
        // argon2 here
        todo!()
    }
}

impl<'a> HasBuilder<NewUserBuilder<'a>, Self> for NewUser<'a> {}
impl<'a> NewUser<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<User> {
        todo!()
    }
}

impl<'a> HasBuilder<NewUserBuilder<'a>, NewUser<'a>> for User {}
