/*
Solution - User creates the config using a builder and type hints from the compiler
So - WorkjamRequest::<Notifications>::new() - error: provide a config
-> create a config: let c = Config::(... hmm only provides a builder ->)
 */

use std::borrow::Cow;

use crate::backend::request::{HasShiftID, endpoints::Notif};

use super::{HasCompanyID, HasEmployeeID, HasLocationID, HasNotificationID, RequestConfig};

// with Cow I can create cheap one-time configs using &str, or configs which own the data (so I don't have to think about the data being in two places)
pub struct CompanyID<'a>(Cow<'a, str>);
pub struct EmployeeID<'a>(Cow<'a, str>);
pub struct LocationID<'a>(Cow<'a, str>);
pub struct ShiftID<'a>(Cow<'a, str>);
pub struct NotificationID<'a>(Cow<'a, str>);

#[derive(Default)]
pub struct Unset;

#[derive(Default)]
pub struct WorkjamRequestConfig<A = Unset, B = Unset, C = Unset, D = Unset, E = Unset> {
    company_id: A,
    employee_id: B,
    location_id: C,
    shift_id: D,
    notification_id: E,
}
impl<A, B, C, D, E> RequestConfig for WorkjamRequestConfig<A, B, C, D, E> {}

impl WorkjamRequestConfig {
    pub fn new() -> Self {
        <Self>::default()
    }
}

impl<B, C, D, E> WorkjamRequestConfig<Unset, B, C, D, E> {
    pub fn company_id<'a>(
        self,
        company_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<CompanyID<'a>, B, C, D, E> {
        let WorkjamRequestConfig {
            employee_id,
            location_id,
            shift_id,
            notification_id,
            ..
        } = self;
        WorkjamRequestConfig::<CompanyID, B, C, D, E> {
            company_id: CompanyID(company_id.into()),
            employee_id,
            location_id,
            shift_id,
            notification_id,
        }
    }
}

impl<B, C, D, E> HasCompanyID for WorkjamRequestConfig<CompanyID<'_>, B, C, D, E> {
    fn company_id(&self) -> &str {
        &self.company_id.0
    }
}

impl<A, C, D, E> WorkjamRequestConfig<A, Unset, C, D, E> {
    pub fn employee_id<'a>(
        self,
        employee_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, EmployeeID<'a>, C, D, E> {
        let WorkjamRequestConfig {
            company_id,
            location_id,
            shift_id,
            notification_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, EmployeeID, C, D, E> {
            employee_id: EmployeeID(employee_id.into()),
            company_id,
            location_id,
            shift_id,
            notification_id,
        }
    }
}

impl<A, C, D, E> HasEmployeeID for WorkjamRequestConfig<A, EmployeeID<'_>, C, D, E> {
    fn employee_id(&self) -> &str {
        &self.employee_id.0
    }
}

impl<A, B, D, E> WorkjamRequestConfig<A, B, Unset, D, E> {
    pub fn location_id<'a>(
        self,
        location_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, LocationID<'a>, D, E> {
        let WorkjamRequestConfig {
            company_id,
            employee_id,
            shift_id,
            notification_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, LocationID, D, E> {
            employee_id,
            company_id,
            location_id: LocationID(location_id.into()),
            shift_id,
            notification_id,
        }
    }
}

impl<A, B, D, E> HasLocationID for WorkjamRequestConfig<A, B, LocationID<'_>, D, E> {
    fn location_id(&self) -> &str {
        &self.location_id.0
    }
}

impl<A, B, C, E> WorkjamRequestConfig<A, B, C, Unset, E> {
    pub fn shift_id<'a>(
        self,
        shift_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, C, ShiftID<'a>, E> {
        let Self {
            company_id,
            employee_id,
            location_id,
            notification_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, C, ShiftID, E> {
            company_id,
            employee_id,
            location_id,
            shift_id: ShiftID(shift_id.into()),
            notification_id,
        }
    }
}

impl<A, B, C, E> HasShiftID for WorkjamRequestConfig<A, B, C, ShiftID<'_>, E> {
    fn shift_id(&self) -> &str {
        &self.shift_id.0
    }
}

impl<A, B, C, D> WorkjamRequestConfig<A, B, C, D, Unset> {
    fn notification_id<'a>(
        self,
        notification_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, C, D, NotificationID<'a>> {
        let Self {
            company_id,
            employee_id,
            location_id,
            shift_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, C, D, NotificationID> {
            company_id,
            employee_id,
            location_id,
            shift_id,
            notification_id: NotificationID(notification_id.into()),
        }
    }
}

impl<A, B, C, D> HasNotificationID for WorkjamRequestConfig<A, B, C, D, NotificationID<'_>> {
    fn notification_id(&self) -> &str {
        &self.notification_id.0
    }
}
