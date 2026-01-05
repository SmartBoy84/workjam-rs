pub mod request;
pub(super) mod ureq;

use std::io::{self, Read};

use http::header::ACCEPT_LANGUAGE;
use thiserror::Error;

pub const AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36 Edg/136.0.0.0";

const ROOT: &str = "https://api.workjam.com/api";
const TOKEN_COOKIE: &str = "token";

#[derive(Error, Debug)]
pub enum WorkjamBackendError<E: std::error::Error> {
    #[error("http error: {0:?}")]
    HttpError(E),

    #[error("json error")]
    ParseError(#[from] serde_json::Error),

    #[error("io err")]
    ReadError(#[from] io::Error),
}

type WorkjamBackendResult<O, C> = Result<O, WorkjamBackendError<<C as WorkjamHttpClient>::Error>>;

use serde::de::DeserializeOwned;

use crate::RequestHandler;

pub(super) struct WorkjamClient<T: WorkjamHttpClient> {
    inner: T,
    token: String, // token's not thaaat long to bother with lifetimes infesting code
}

impl<C: WorkjamHttpClient> RequestHandler for WorkjamClient<C> {
    type E = WorkjamBackendError<C::Error>;

    fn get<T, P>(&self, r: &request::WorkjamRequest<P>) -> Result<T, Self::E>
    where
        T: DeserializeOwned,
        P: request::endpoints::Endpoint,
    {
        Ok(serde_json::from_reader(
            self.inner
                .get(r.uri(), (ACCEPT_LANGUAGE.as_str(), "*"))
                .map_err(|e| WorkjamBackendError::HttpError(e))?,
        )?)
    }

    fn patch<T, P>(&self, r: &request::WorkjamRequest<P>) -> Result<T, Self::E>
    where
        T: DeserializeOwned,
        P: request::endpoints::Endpoint,
    {
        Ok(serde_json::from_reader(
            self.inner
                .patch(r.uri(), &self.token)
                .map_err(|e| WorkjamBackendError::HttpError(e))?,
        )?)
    }

    fn put<T, P>(&self, r: &request::WorkjamRequest<P>) -> Result<T, Self::E>
    where
        T: DeserializeOwned,
        P: request::endpoints::Endpoint,
    {
        // needed to set READ status on notification
        Ok(serde_json::from_reader(
            self.inner
                .put(r.uri(), &self.token)
                .map_err(|e| WorkjamBackendError::HttpError(e))?,
        )?)
    }

    fn post<T, P>(&self, r: &request::WorkjamRequest<P>) -> Result<T, Self::E>
    where
        T: DeserializeOwned,
        P: request::endpoints::Endpoint,
    {
        // needed to set READ status on notification
        Ok(serde_json::from_reader(
            self.inner
                .post(r.uri(), &self.token)
                .map_err(|e| WorkjamBackendError::HttpError(e))?,
        )?)
    }
}

impl<C: WorkjamHttpClient> WorkjamClient<C> {
    pub(super) fn new(backend: C, token: &str) -> Self {
        backend.set_cookie(&format!("{TOKEN_COOKIE}={token}"), ROOT);
        Self {
            inner: backend,
            token: token.into(),
        }
    }

    pub(super) fn get_raw(&self, uri: &str) -> WorkjamBackendResult<String, C> {
        let mut s = String::new();
        self.inner
            .get(uri, (ACCEPT_LANGUAGE.as_str(), "*"))
            .map_err(|e| WorkjamBackendError::HttpError(e))?
            .read_to_string(&mut s)?;
        Ok(s)
    }
}

// trait can be public - inner methods will never be leaked because encapsulated in struct
pub trait WorkjamHttpClient {
    // impl Trait in trait method return types is permitted, however I want to enforce that *all* methods return the same type for consistency
    type Reader: std::io::Read; // no need for GAT because not doing massive downloads
    type Error: std::error::Error;

    fn set_cookie(&self, cookie: &str, uri: &'static str); // must be able to set a single persistent cookie once
    fn patch(&self, uri: &str, bearer_token: &str) -> Result<Self::Reader, Self::Error>; // this is all we need for patch, nothing more
    fn get(&self, uri: &str, header: (&str, &str)) -> Result<Self::Reader, Self::Error>;
    fn put(&self, uri: &str, bearer_token: &str) -> Result<Self::Reader, Self::Error>;
    fn post(&self, uri: &str, bearer_token: &str) -> Result<Self::Reader, Self::Error>;
}
