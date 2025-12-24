/*
Solution - User creates the config using a builder and type hints from the compiler
So - WorkjamRequest::<Notifications>::new() - error: provide a config
-> create a config: let c = Config::(... hmm only provides a builder ->)
 */

use std::{borrow::Cow, panic::Location};

use crate::backend::request::{HasShiftID, payload::events::Shift};

use super::{HasCompanyID, HasEmployeeID, HasLocationID, RequestConfig};

// with Cow I can create cheap one-time configs using &str, or configs which own the data (so I don't have to think about the data being in two places)
pub struct CompanyID<'a>(Cow<'a, str>);
pub struct EmployeeID<'a>(Cow<'a, str>);
pub struct LocationID<'a>(Cow<'a, str>);
pub struct ShiftID<'a>(Cow<'a, str>);

#[derive(Default)]
pub struct Unset;

#[derive(Default)]
pub struct WorkjamRequestConfig<A = Unset, B = Unset, C = Unset, D = Unset> {
    company_id: A,
    employee_id: B,
    location_id: C,
    shift_id: D,
}
impl<A, B, C, D> RequestConfig for WorkjamRequestConfig<A, B, C, D> {}

impl WorkjamRequestConfig {
    pub fn new() -> Self {
        <Self>::default()
    }
}

impl<B, C, D> WorkjamRequestConfig<Unset, B, C, D> {
    pub fn company_id<'a>(
        self,
        company_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<CompanyID<'a>, B, C, D> {
        let WorkjamRequestConfig {
            employee_id,
            location_id,
            shift_id,
            ..
        } = self;
        WorkjamRequestConfig::<CompanyID, B, C, D> {
            company_id: CompanyID(company_id.into()),
            employee_id,
            location_id,
            shift_id,
        }
    }
}

impl<B, C, D> HasCompanyID for WorkjamRequestConfig<CompanyID<'_>, B, C, D> {
    fn company_id(&self) -> &str {
        &self.company_id.0
    }
}

impl<A, C, D> WorkjamRequestConfig<A, Unset, C, D> {
    pub fn employee_id<'a>(
        self,
        employee_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, EmployeeID<'a>, C, D> {
        let WorkjamRequestConfig {
            company_id,
            location_id,
            shift_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, EmployeeID, C, D> {
            employee_id: EmployeeID(employee_id.into()),
            company_id,
            location_id,
            shift_id,
        }
    }
}

impl<A, C, D> HasEmployeeID for WorkjamRequestConfig<A, EmployeeID<'_>, C, D> {
    fn employee_id(&self) -> &str {
        &self.employee_id.0
    }
}

impl<A, B, D> WorkjamRequestConfig<A, B, Unset, D> {
    pub fn location_id<'a>(
        self,
        location_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, LocationID<'a>, D> {
        let WorkjamRequestConfig {
            company_id,
            employee_id,
            shift_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, LocationID, D> {
            employee_id,
            company_id,
            location_id: LocationID(location_id.into()),
            shift_id,
        }
    }
}

impl<A, B, D> HasLocationID for WorkjamRequestConfig<A, B, LocationID<'_>, D> {
    fn location_id(&self) -> &str {
        &self.location_id.0
    }
}

impl<A, B, C> WorkjamRequestConfig<A, B, C, Unset> {
    pub fn shift_id<'a>(
        self,
        shift_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, C, ShiftID<'a>> {
        let Self {
            company_id,
            employee_id,
            location_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, C, ShiftID> {
            company_id,
            employee_id,
            location_id,
            shift_id: ShiftID(shift_id.into()),
        }
    }
}

impl<A, B, C> HasShiftID for WorkjamRequestConfig<A, B, C, ShiftID<'_>> {
    fn shift_id(&self) -> &str {
        &self.shift_id.0
    }
}
