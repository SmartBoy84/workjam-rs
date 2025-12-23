pub mod backend;

use backend::{
    AGENT, WorkjamBackendError, WorkjamClient, WorkjamHttpClient,
    request::{
        WorkjamRequest,
        endpoints::{Auth, Endpoint},
    },
    ureq::UreqWorkjamHttpClient,
};

type SelectedHttpClient = UreqWorkjamHttpClient;

#[derive(thiserror::Error, Debug)]
pub enum WorkjamError<C: WorkjamHttpClient> {
    #[error("backend error")]
    BackendError(#[from] WorkjamBackendError<C::Error>),
}

type WorkjamResult<R, C> = Result<R, WorkjamBackendError<<C as WorkjamHttpClient>::Error>>;

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
        self.backend()
            .auth_patch(WorkjamRequest::<Auth>::new(&()).uri())
    }

    pub fn get_request<P: Endpoint>(&self, r: WorkjamRequest<P>) -> WorkjamResult<P::Res, C> {
        self.backend().get(r.uri())
    }

    pub fn get_request_raw<P: Endpoint>(&self, r: WorkjamRequest<P>) -> WorkjamResult<String, C> {
        self.backend().get_raw(r.uri())
    }
}
