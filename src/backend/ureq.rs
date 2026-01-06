// backend is fully pluggable

use ureq::{
    self, BodyReader, Cookie,
    config::Config,
    http::{Uri, header::AUTHORIZATION},
};

use super::WorkjamHttpClient;

#[derive(Debug)]
pub struct UreqWorkjamHttpClient {
    a: ureq::Agent,
}

impl UreqWorkjamHttpClient {
    // apprently, bad practise to enforce constructors with traits...
    pub fn new(agent: &str) -> Self {
        let a = ureq::Agent::new_with_config(Config::builder().user_agent(agent).build());
        Self { a }
    }
}

impl WorkjamHttpClient for UreqWorkjamHttpClient {
    type Reader = BodyReader<'static>; // not streaming, so 'static is fine
    type Error = ureq::Error;

    // uri is const - 'static enforces that
    fn set_cookie(&self, cookie: &str, uri: &'static str) {
        let mut c = self.a.cookie_jar_lock();

        let uri = Uri::from_static(uri);
        let cookie = Cookie::parse(cookie.to_owned(), &uri).unwrap();

        c.insert(cookie, &uri).unwrap();
        c.release();
    }

    fn patch(&self, uri: &str, bearer_token: &str) -> Result<Self::Reader, Self::Error> {
        Ok(self
            .a
            .patch(uri)
            .header(AUTHORIZATION, bearer_token)
            .content_type("application/json")
            .send(&[])?
            .into_body()
            .into_reader())
    }

    fn put(&self, uri: &str, bearer_token: &str) -> Result<Self::Reader, Self::Error> {
        Ok(self
            .a
            .put(uri)
            .header(AUTHORIZATION, bearer_token)
            .content_type("application/json")
            .send(&[])?
            .into_body()
            .into_reader())
    }
    fn post(&self, uri: &str, bearer_token: &str) -> Result<Self::Reader, Self::Error> {
        Ok(self
            .a
            .post(uri)
            .header(AUTHORIZATION, bearer_token)
            .content_type("application/json")
            .send("{}")? // needed else 404
            .into_body()
            .into_reader())
    }

    fn get(
        &self,
        uri: &str,
        bearer_token: &str,
        (header_name, header_val): (&str, &str),
    ) -> Result<Self::Reader, Self::Error> {
        Ok(self
            .a
            .get(uri)
            .header(header_name, header_val)
            .header(AUTHORIZATION, bearer_token)
            .call()?
            .into_body()
            .into_reader())
    }
}
