use std::{cell::RefCell, collections::HashMap};

use anyhow::Result;
use url::Url;

pub type Cookies = HashMap<String, String>;
pub type DomainMap = HashMap<Url, Cookies>;

#[derive(Debug)]
pub struct CookieJar {
    cookies: RefCell<DomainMap>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self {
            cookies: Default::default(),
        }
    }
}

pub trait CookieStore {
    fn get_all(&self, domain: &str) -> Result<Option<Cookies>>;
    fn set(&self, domain: &str, name: &str, value: &str) -> Result<()>;
}

impl CookieStore for CookieJar {
    fn get_all(&self, domain: &str) -> Result<Option<Cookies>> {
        let domain_url = Url::parse(domain)?;
        Ok(self.cookies.borrow().get(&domain_url).cloned())
    }

    fn set(&self, domain: &str, name: &str, value: &str) -> Result<()> {
        let domain_url = Url::parse(domain)?;
        let mut cookies = self.cookies.borrow_mut();

        if let Some(cookies) = cookies.get_mut(&domain_url) {
            cookies.insert(name.into(), value.into());
        }

        Ok(())
    }
}
