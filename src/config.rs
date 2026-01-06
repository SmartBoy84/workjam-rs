/*
Solution - User creates the config using a builder and type hints from the compiler
So - WorkjamRequest::<Notifications>::new() - error: provide a config
-> create a config: let c = Config::(... hmm only provides a builder ->)
 */

use std::borrow::Cow;

use restman_rs::request::RequestConfig;

pub trait HasEmployeeID: RequestConfig {
    fn employee_id(&self) -> &str;
}
pub trait HasCompanyID: RequestConfig {
    fn company_id(&self) -> &str;
}
pub trait HasLocationID: RequestConfig {
    fn location_id(&self) -> &str;
}

// endpoint specific parameters
pub trait HasShiftID: RequestConfig {
    fn shift_id(&self) -> &str;
}
pub trait HasNotificationID: RequestConfig {
    fn notification_id(&self) -> &str;
}
pub trait HasApprovalReqID: RequestConfig {
    fn approval_req_id(&self) -> &str;
}

// with Cow I can create cheap one-time configs using &str, or configs which own the data (so I don't have to think about the data being in two places)
pub struct CompanyID<'a>(Cow<'a, str>);
pub struct EmployeeID<'a>(Cow<'a, str>);
pub struct LocationID<'a>(Cow<'a, str>);
pub struct ShiftID<'a>(Cow<'a, str>);
pub struct NotificationID<'a>(Cow<'a, str>);
pub struct ApprovalReqID<'a>(Cow<'a, str>);

#[derive(Default)]
pub struct Unset;

#[derive(Default, Clone, Copy)]
pub struct WorkjamRequestConfig<A = Unset, B = Unset, C = Unset, D = Unset, E = Unset, F = Unset> {
    company_id: A,
    employee_id: B,
    location_id: C,
    shift_id: D,
    notification_id: E,
    approval_req_id: F,
}
impl<A, B, C, D, E, F> RequestConfig for WorkjamRequestConfig<A, B, C, D, E, F> {}

impl WorkjamRequestConfig {
    pub fn new() -> Self {
        <Self>::default()
    }
}

impl<B, C, D, E, F> WorkjamRequestConfig<Unset, B, C, D, E, F> {
    pub fn company_id<'a>(
        self,
        company_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<CompanyID<'a>, B, C, D, E, F> {
        let WorkjamRequestConfig {
            employee_id,
            location_id,
            shift_id,
            notification_id,
            approval_req_id,
            ..
        } = self;
        WorkjamRequestConfig::<CompanyID, B, C, D, E, F> {
            company_id: CompanyID(company_id.into()),
            employee_id,
            location_id,
            shift_id,
            notification_id,
            approval_req_id,
        }
    }
}

impl<B, C, D, E, F> HasCompanyID for WorkjamRequestConfig<CompanyID<'_>, B, C, D, E, F> {
    fn company_id(&self) -> &str {
        &self.company_id.0
    }
}

impl<A, C, D, E, F> WorkjamRequestConfig<A, Unset, C, D, E, F> {
    pub fn employee_id<'a>(
        self,
        employee_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, EmployeeID<'a>, C, D, E, F> {
        let WorkjamRequestConfig {
            company_id,
            location_id,
            shift_id,
            notification_id,
            approval_req_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, EmployeeID, C, D, E, F> {
            employee_id: EmployeeID(employee_id.into()),
            company_id,
            location_id,
            shift_id,
            notification_id,
            approval_req_id,
        }
    }
}

impl<A, C, D, E, F> HasEmployeeID for WorkjamRequestConfig<A, EmployeeID<'_>, C, D, E, F> {
    fn employee_id(&self) -> &str {
        &self.employee_id.0
    }
}

impl<A, B, D, E, F> WorkjamRequestConfig<A, B, Unset, D, E, F> {
    pub fn location_id<'a>(
        self,
        location_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, LocationID<'a>, D, E, F> {
        let WorkjamRequestConfig {
            company_id,
            employee_id,
            shift_id,
            notification_id,
            approval_req_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, LocationID, D, E, F> {
            employee_id,
            company_id,
            location_id: LocationID(location_id.into()),
            shift_id,
            notification_id,
            approval_req_id,
        }
    }
}

impl<A, B, D, E, F> HasLocationID for WorkjamRequestConfig<A, B, LocationID<'_>, D, E, F> {
    fn location_id(&self) -> &str {
        &self.location_id.0
    }
}

impl<A, B, C, E, F> WorkjamRequestConfig<A, B, C, Unset, E, F> {
    pub fn shift_id<'a>(
        self,
        shift_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, C, ShiftID<'a>, E, F> {
        let Self {
            company_id,
            employee_id,
            location_id,
            notification_id,
            approval_req_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, C, ShiftID, E, F> {
            company_id,
            employee_id,
            location_id,
            shift_id: ShiftID(shift_id.into()),
            notification_id,
            approval_req_id,
        }
    }
}

impl<A, B, C, E, F> HasShiftID for WorkjamRequestConfig<A, B, C, ShiftID<'_>, E, F> {
    fn shift_id(&self) -> &str {
        &self.shift_id.0
    }
}

impl<A, B, C, D, F> WorkjamRequestConfig<A, B, C, D, Unset, F> {
    pub fn notification_id<'a>(
        self,
        notification_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, C, D, NotificationID<'a>, F> {
        let Self {
            company_id,
            employee_id,
            location_id,
            shift_id,
            approval_req_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, C, D, NotificationID, F> {
            company_id,
            employee_id,
            location_id,
            shift_id,
            approval_req_id,
            notification_id: NotificationID(notification_id.into()),
        }
    }
}

impl<A, B, C, D, F> HasNotificationID for WorkjamRequestConfig<A, B, C, D, NotificationID<'_>, F> {
    fn notification_id(&self) -> &str {
        &self.notification_id.0
    }
}

impl<A, B, C, D, E> WorkjamRequestConfig<A, B, C, D, E, Unset> {
    pub fn approval_req_id<'a>(
        self,
        approval_req_id: impl Into<Cow<'a, str>>,
    ) -> WorkjamRequestConfig<A, B, C, D, E, ApprovalReqID<'a>> {
        let Self {
            company_id,
            employee_id,
            location_id,
            shift_id,
            notification_id,
            ..
        } = self;
        WorkjamRequestConfig::<A, B, C, D, E, ApprovalReqID<'a>> {
            company_id,
            employee_id,
            location_id,
            shift_id,
            notification_id,
            approval_req_id: ApprovalReqID(approval_req_id.into()),
        }
    }
}

impl<A, B, C, D, E> HasApprovalReqID for WorkjamRequestConfig<A, B, C, D, E, ApprovalReqID<'_>> {
    fn approval_req_id(&self) -> &str {
        &self.approval_req_id.0
    }
}
