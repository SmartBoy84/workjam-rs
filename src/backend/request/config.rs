/*
Solution - User creates the config using a builder and type hints from the compiler
So - WorkjamRequest::<Notifications>::new() - error: provide a config
-> create a config: let c = Config::(... hmm only provides a builder ->)
 */

use std::borrow::Cow;

use crate::client::backend::request::{HasCompanyID, HasEmployeeID, HasLocationID, RequestConfig};

// with Cow I can create cheap one-time configs using &str, or configs which own the data (so I don't have to think about the data being in two places)
pub struct CompanyID<'a>(Cow<'a, str>);
pub struct EmployeeID<'a>(Cow<'a, str>);
pub struct LocationID<'a>(Cow<'a, str>);

#[derive(Default)]
pub struct Unset;

#[derive(Default)]
pub struct WorkjamRequestConfig<A = Unset, B = Unset, C = Unset> {
    company_id: A,
    employee_id: B,
    location_id: C,
}
impl<A, B, C> RequestConfig for WorkjamRequestConfig<A, B, C> {}

impl WorkjamRequestConfig {
    pub fn new() -> Self {
        <Self>::default()
    }
}

impl<B, C> WorkjamRequestConfig<Unset, B, C> {
    pub fn company_id<'a>(
        self,
        company_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<CompanyID<'a>, B, C> {
        let WorkjamRequestConfig {
            employee_id,
            location_id,
            ..
        } = self;
        WorkjamRequestConfig::<CompanyID, B, C> {
            company_id: CompanyID(company_id.into()),
            employee_id,
            location_id,
        }
    }
}

impl<B, C> HasCompanyID for WorkjamRequestConfig<CompanyID<'_>, B, C> {
    fn company_id(&self) -> &str {
        &self.company_id.0
    }
}

impl<A, C> WorkjamRequestConfig<A, Unset, C> {
    pub fn employee_id<'a>(
        self,
        employee_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, EmployeeID<'a>, C> {
        let WorkjamRequestConfig {
            company_id,
            location_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, EmployeeID, C> {
            employee_id: EmployeeID(employee_id.into()),
            company_id,
            location_id,
        }
    }
}

impl<A, C> HasEmployeeID for WorkjamRequestConfig<A, EmployeeID<'_>, C> {
    fn employee_id(&self) -> &str {
        &self.employee_id.0
    }
}

impl<A, B> WorkjamRequestConfig<A, B, Unset> {
    pub fn location_id<'a>(
        self,
        location_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, LocationID<'a>> {
        let WorkjamRequestConfig {
            company_id,
            employee_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, LocationID> {
            employee_id,
            company_id,
            location_id: LocationID(location_id.into()),
        }
    }
}

impl<A, B> HasLocationID for WorkjamRequestConfig<A, B, LocationID<'_>> {
    fn location_id(&self) -> &str {
        &self.location_id.0
    }
}
