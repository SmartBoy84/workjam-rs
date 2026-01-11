pub mod config;
pub mod endpoints;
pub mod parameters;
pub mod parts;
pub mod requests;

use std::marker::PhantomData;

pub use restman_rs::request;

use restman_rs::{
    ApiBackendError, ApiBackendResult, ApiHttpClient, MethodMarkerGetter, Server,
    client::{AGENT, ApiClient},
    request::{ApiRequest, endpoints::Endpoint},
    ureq::UreqApiHttpClient,
};

use crate::endpoints::Auth;

const TOKEN_COOKIE: &str = "token";

pub struct Workjam;
impl Server for Workjam {
    const ROOT: &str = "https://api.workjam.com/api";
}

type SelectedHttpClient = UreqApiHttpClient;

#[derive(thiserror::Error, Debug)]
pub enum WorkjamError<C: ApiHttpClient> {
    #[error("backend error")]
    BackendError(#[from] ApiBackendError<C>),
}

// default client impl is ureq
pub struct WorkjamUser<C: ApiHttpClient = SelectedHttpClient, S: Server = Workjam> {
    backend: ApiClient<C>,
    server: PhantomData<S>,
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

impl<C, S> WorkjamUser<C, S>
where
    C: ApiHttpClient,
    S: Server,
{
    fn inner_new_with_backend(backend: C, token: &str) -> Self {
        backend.set_cookie(&format!("{TOKEN_COOKIE}={token}"), S::ROOT);
        let backend = ApiClient::new(backend, token);
        Self {
            backend,
            server: PhantomData,
        }
    }

    fn backend(&self) -> &ApiClient<C> {
        &self.backend
    }

    pub fn get_auth(&self) -> ApiBackendResult<<Auth as Endpoint>::Res, C>
    where
        Auth: Endpoint<Ser = S, Method: MethodMarkerGetter<C>>,
    {
        self.request(&ApiRequest::<Auth>::new(&()))
    }

    // note; have to enforce Ser because token cookie is set to workjam
    pub fn request<P>(&self, r: &ApiRequest<P>) -> ApiBackendResult<P::Res, C>
    where
        P: Endpoint<Ser = S>, // enforce that Server of the request is Workjam since using helper library
        P::Method: MethodMarkerGetter<C>,
    {
        self.backend().request(r)
    }

    pub fn raw_request<P>(&self, r: &ApiRequest<P>) -> ApiBackendResult<String, C>
    where
        P: Endpoint<Ser = S>, // enforce that Server of the request is Workjam since using helper library
        P::Method: MethodMarkerGetter<C>,
    {
        self.backend().raw_request(r)
    }
}
