//! User whitelist authentication filter.

#![allow(dead_code)]

use std::collections::HashSet;

use crate::domain::messenger::{AuthFilter, UserId};

pub struct UserWhitelist {
    allowed: HashSet<u64>,
}

impl UserWhitelist {
    pub fn new(users: HashSet<u64>) -> Self {
        Self { allowed: users }
    }

    pub fn from_iter(iter: impl IntoIterator<Item = u64>) -> Self {
        Self {
            allowed: iter.into_iter().collect(),
        }
    }
}

impl AuthFilter for UserWhitelist {
    fn is_allowed(&self, user_id: UserId) -> bool {
        self.allowed.contains(&user_id.0)
    }
}
