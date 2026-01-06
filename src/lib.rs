pub mod config;
pub mod endpoints;
pub mod parameters;
pub mod parts;
pub mod requests;

pub use restman_rs::request as request;

use restman_rs::{
    HttpMethod, Server,
    client::{AGENT, ApiBackendError, ApiClient, ApiHttpClient},
    request::{ApiRequest, endpoints::Endpoint},
    ureq::UreqApiHttpClient,
};

use crate::endpoints::Auth;

pub struct Workjam;
impl Server for Workjam {
    const ROOT: &str = "https://api.workjam.com/api/";
}

type SelectedHttpClient = UreqApiHttpClient;
type WorkjamBackendResult<T, C> = Result<T, ApiBackendError<<C as ApiHttpClient>::Error>>;

#[derive(thiserror::Error, Debug)]
pub enum WorkjamError<C: ApiHttpClient> {
    #[error("backend error")]
    BackendError(#[from] ApiBackendError<C::Error>),
}

// default client impl is ureq
pub struct WorkjamUser<C: ApiHttpClient = SelectedHttpClient> {
    backend: ApiClient<C>,
}

impl WorkjamUser {
    pub fn new(token: &str) -> Self {
        Self::new_with_backend(SelectedHttpClient::new(AGENT), token)
    }
}

impl<C> WorkjamUser<C>
where
    C: ApiHttpClient,
{
    pub fn new_with_backend(backend: C, token: &str) -> Self {
        let backend = ApiClient::new(backend, token);
        Self { backend }
    }

    fn backend(&self) -> &ApiClient<C> {
        &self.backend
    }

    pub fn get_auth(&self) -> WorkjamBackendResult<<Auth as Endpoint>::Res, C> {
        self.request(&ApiRequest::<Auth>::new(&()))
    }

    pub fn request<P>(&self, r: &ApiRequest<P>) -> WorkjamBackendResult<P::Res, C>
    where
        P: Endpoint<Ser = Workjam>, // enforce that Server of the request is Workjam since using helper library
    {
        self.backend().request(r)
    }

    pub fn request_raw(&self, r: &str) -> WorkjamBackendResult<String, C> {
        self.backend().get_raw(r)
    }
}
