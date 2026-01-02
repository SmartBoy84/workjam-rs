pub mod backend;

use backend::{
    AGENT, WorkjamBackendError, WorkjamClient, WorkjamHttpClient,
    request::{
        WorkjamRequest,
        endpoints::{Auth, Endpoint},
    },
    ureq::UreqWorkjamHttpClient,
};
use serde::de::DeserializeOwned;

type SelectedHttpClient = UreqWorkjamHttpClient;

#[derive(thiserror::Error, Debug)]
pub enum WorkjamError<C: WorkjamHttpClient> {
    #[error("backend error")]
    BackendError(#[from] WorkjamBackendError<C::Error>),
}

type WorkjamResult<R, C> = Result<R, WorkjamBackendError<<C as WorkjamHttpClient>::Error>>;

pub trait HttpMethod {
    fn request<H: RequestHandler, T: DeserializeOwned>(handler: &H, uri: &str) -> Result<T, H::E>;
}

// pluggable backend
pub struct PATCH;
pub struct GET;
pub struct PUT;
pub struct POST;

impl HttpMethod for PATCH {
    fn request<H: RequestHandler, T: DeserializeOwned>(handler: &H, uri: &str) -> Result<T, H::E> {
        handler.patch(uri)
    }
}

impl HttpMethod for GET {
    fn request<H: RequestHandler, T: DeserializeOwned>(handler: &H, uri: &str) -> Result<T, H::E> {
        handler.get(uri)
    }
}
impl HttpMethod for PUT {
    fn request<H: RequestHandler, T: DeserializeOwned>(handler: &H, uri: &str) -> Result<T, H::E> {
        handler.put(uri)
    }
}
impl HttpMethod for POST {
    fn request<H: RequestHandler, T: DeserializeOwned>(handler: &H, uri: &str) -> Result<T, H::E> {
        handler.post(uri)
    }
}

pub trait RequestHandler {
    type E: std::error::Error;
    fn get<T>(&self, uri: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned;

    fn patch<T>(&self, uri: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned;

    fn put<T>(&self, uri: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned;
    fn post<T>(&self, uri: &str) -> Result<T, Self::E>
    where
        T: DeserializeOwned;
}

// default client impl is ureq
pub struct WorkjamUser<C: WorkjamHttpClient = SelectedHttpClient> {
    backend: WorkjamClient<C>,
}

impl WorkjamUser {
    pub fn new(token: &str) -> Self {
        Self::new_with_backend(SelectedHttpClient::new(AGENT), token)
    }
}

impl<C> WorkjamUser<C>
where
    C: WorkjamHttpClient,
{
    pub fn new_with_backend(backend: C, token: &str) -> Self {
        let backend = WorkjamClient::new(backend, token);
        Self { backend }
    }

    fn backend(&self) -> &WorkjamClient<C> {
        &self.backend
    }

    pub fn get_auth(&self) -> WorkjamResult<<Auth as Endpoint>::Res, C> {
        self.request(&WorkjamRequest::<Auth>::new(&()))
    }

    pub fn request<P>(&self, r: &WorkjamRequest<P>) -> WorkjamResult<P::Res, C>
    where
        P: Endpoint,
    {
        P::Method::request(self.backend(), &r.uri())
    }

    pub fn request_raw(&self, r: &str) -> WorkjamResult<String, C> {
        self.backend().get_raw(r)
    }
}
