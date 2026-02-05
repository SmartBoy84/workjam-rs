pub mod config;
pub mod endpoints;
pub mod parameters;
pub mod parts;
pub mod requests;

pub use restman_rs::{
    client::ApiClient,
    request::{ApiRequest, ApiRequestWithPara, ValidRequest},
};

use restman_rs::{
    ApiBackendError, ApiBackendResult, ApiHttpClient, ConstServer, DynamicServer, Patch, Server,
    client::{AGENT, ApiClientBackend, ApiClientServer},
    request::endpoints::Endpoint,
    ureq::UreqApiHttpClient,
};

use crate::endpoints::Auth;

const TOKEN_COOKIE: &str = "token";

pub struct Workjam {
    pub server: String,
}
impl Server for Workjam {}
impl ConstServer for Workjam {
    const ROOT: &str = "https://api.workjam.com/api";
}

impl DynamicServer for Workjam {
    fn get_root(&self) -> &str {
        &self.server
    }
}

type SelectedHttpClient = UreqApiHttpClient;

// default client impl is ureq
pub struct WorkjamUser<C: ApiHttpClient = SelectedHttpClient> {
    token: String,
    backend: C,
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
        Self::inner_new_with_backend(backend, token)
    }
}

impl<C> WorkjamUser<C>
where
    C: ApiHttpClient,
{
    fn inner_new_with_backend(backend: C, token: &str) -> Self {
        backend.set_cookie(&format!("{TOKEN_COOKIE}={token}"), Workjam::ROOT);
        Self {
            token: token.to_owned(),
            backend,
        }
    }

    pub fn get_auth(&self) -> ApiBackendResult<<Auth as Endpoint>::Res, C>
    where
        C: Patch,
    {
        self.request(&ApiRequest::<Auth>::new(&()))
    }
}

impl<C: ApiHttpClient> ApiClientBackend<C> for WorkjamUser<C> {
    fn token(&self) -> &str {
        &self.token
    }
    fn backend(&self) -> &C {
        &self.backend
    }
}

impl<C: ApiHttpClient> ApiClientServer<Workjam> for WorkjamUser<C> {}
