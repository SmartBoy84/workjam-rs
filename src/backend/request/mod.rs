// // /v1/companies/{company id}/users/{user id}/notifications

pub mod config;
pub mod endpoints;
pub mod parameters;
mod parts;
pub mod payload;

use std::marker::PhantomData;

use endpoints::{Endpoint, EndpointWithParameters};
use parameters::QueryParameters;

use crate::backend::request::{
    endpoints::EndpointWithNoPara, parts::Shifts, payload::events::EventData,
};

use super::ROOT;

pub trait SerialiseRequestPart<C: RequestConfig>: RequestPart {
    const WORD: &str;
    type Next: SerialiseRequestPart<C>;

    fn get_val(_config: &C) -> Option<&str> {
        None
    }

    fn add_str(s: &mut String, config: &C) {
        <<Self as SerialiseRequestPart<C>>::Next>::add_str(s, config);
        s.push('/');
        s.push_str(<Self as SerialiseRequestPart<C>>::WORD);

        // should get optimised away?
        if let Some(v) = Self::get_val(config) {
            s.push('/');
            s.push_str(v);
        }
    }
}

pub trait RequestPart {
    /*
    Marker trait prevents creation of RequestParts with invalid inner types
    */
}

pub trait RequestConfig {}
pub trait HasEmployeeID: RequestConfig {
    fn employee_id(&self) -> &str;
}
pub trait HasCompanyID: RequestConfig {
    fn company_id(&self) -> &str;
}
pub trait HasLocationID: RequestConfig {
    fn location_id(&self) -> &str;
}
pub trait HasShiftID: RequestConfig {
    fn shift_id(&self) -> &str;
}
pub trait HasNotificationID: RequestConfig {
    fn notification_id(&self) -> &str;
}

impl RequestConfig for () {}

#[derive(Default)]
// use the more general Endpoint here to avoid leaking implementation detail `Config`
pub struct WorkjamRequest<P: Endpoint> {
    uri: String,
    uri_len: usize,
    inner: PhantomData<P>,
}

impl<E: Endpoint> WorkjamRequest<E> {
    fn new_inner<C: RequestConfig>(c: &C) -> Self
    where
        E: SerialiseRequestPart<C>, // guaranteed, since I do SerialiseEndpoint: Endpoint
    {
        let mut uri = ROOT.to_string();
        E::add_str(&mut uri, c);
        let uri_len = uri.len();
        Self {
            uri,
            uri_len,
            inner: PhantomData,
        }
    }
}

impl<E: Endpoint + EndpointWithNoPara> WorkjamRequest<E> {
    pub fn new<C: RequestConfig>(c: &C) -> Self
    where
        E: SerialiseRequestPart<C>, // guaranteed, since I do SerialiseEndpoint: Endpoint
    {
        Self::new_inner(c)
    }
}

impl<E: Endpoint + EndpointWithParameters> WorkjamRequest<E> {
    pub fn new_with_para<C>(c: &C, p: E::P) -> Self
    where
        C: RequestConfig,
        E: SerialiseRequestPart<C>,
    {
        let mut s = Self::new_inner(c);
        p.add_str(s.uri_mut());
        s
    }
}

impl<T: Endpoint> WorkjamRequest<T> {
    pub fn change_para(&mut self, p: T::P)
    where
        T: EndpointWithParameters,
    {
        self.uri.truncate(self.uri_len);
        p.add_str(&mut self.uri);
    }

    fn uri_mut(&mut self) -> &mut String {
        &mut self.uri
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}
