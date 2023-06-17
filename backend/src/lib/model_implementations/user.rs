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

impl<'a> NewUserBuilder<'a> {
    pub fn build(&self) -> Option<NewUser<'a>> {
        if let (Some(username), Some(email), Some(password_hash)) =
            (self.username, self.email, self.password_hash)
        {
            Some(NewUser {
                username,
                email,
                profile_picture: self.profile_picture,
                password_hash,
            })
        } else {
            None
        }
    }

    pub fn username(&'a mut self, username: &'a str) -> &'a mut Self {
        self.username = Some(username);
        self
    }

    pub fn email(&'a mut self, email: &'a str) -> &'a mut Self {
        self.email = Some(email);
        self
    }

    pub fn profile_picture(&'a mut self) -> &'a mut Self {
        todo!()
    }

    pub fn password(&'a mut self, _password: &'a str) -> &'a mut Self {
        // argon2 here
        todo!()
    }
}

impl<'a> NewUser<'a> {
    pub fn insert(&self, _conn: &mut PgConnection) -> Result<User> {
        todo!()
    }
}

impl User {
    pub fn builder<'a>() -> NewUserBuilder<'a> {
        NewUserBuilder::default()
    }
}
