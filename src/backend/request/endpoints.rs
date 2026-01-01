use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::{GET, HttpMethod, PATCH, PUT, backend::request::parts::Notif};

use super::{
    RequestConfig, RequestPart, SerialiseRequestPart,
    parameters::{EmployeesDetailsPara, EventsPara, NotifPara, QueryParameters},
    parts::{Companies, Employees, Shifts, Users, V3},
    payload::{
        AuthRes, OnsiteRes, WorkingStatusRes,
        coworkers::CoworkersRes,
        employee::{EmployeeDetailsRes, EmployeesDetailsRes},
        events::EventsRes,
        notifications::NotifRes,
    },
};

pub trait Endpoint {
    type Method: HttpMethod;
    type Res: DeserializeOwned;
}

pub trait EndpointWithNoPara: Endpoint {}
pub trait EndpointWithParameters: Endpoint {
    type P: QueryParameters;
}

macro_rules! endpoint_common {
    ($vis:vis $name:ident, $word:literal, $default:ty, $res:ty, $method:ty) => {
        $vis struct $name<T: RequestPart = $default>(PhantomData<T>);

        impl<T: RequestPart> Endpoint for $name<T> {
            type Method = $method;
            type Res = $res;
        }

        impl<T: RequestPart> RequestPart for $name<T> {}

        impl<C: RequestConfig, T: SerialiseRequestPart<C>> SerialiseRequestPart<C> for $name<T> {
            const WORD: &str = $word;
            type Next = T;
        }
    };
}

macro_rules! endpoint {
    // No parameters, default GET
    ($vis:vis $name:ident, $word:literal, $default:ty, $res:ty) => {
        endpoint_common!($vis $name, $word, $default, $res, GET);
        impl<T: RequestPart> EndpointWithNoPara for $name<T> {}
    };
    // No parameters, explicit method
    ($vis:vis $name:ident, $word:literal, $default:ty, $res:ty, method = $method:ty) => {
        endpoint_common!($vis $name, $word, $default, $res, $method);
        impl<T: RequestPart> EndpointWithNoPara for $name<T> {}
    };
    // With parameters, default GET
    ($vis:vis $name:ident, $word:literal, $default:ty, $res:ty, $params:ty) => {
        endpoint_common!($vis $name, $word, $default, $res, GET);
        impl<T: RequestPart> EndpointWithParameters for $name<T> {
            type P = $params;
        }
    };
    // With parameters, explicit method
    ($vis:vis $name:ident, $word:literal, $default:ty, $res:ty, $params:ty, method = $method:ty) => {
        endpoint_common!($vis $name, $word, $default, $res, $method);
        impl<T: RequestPart> EndpointWithParameters for $name<T> {
            type P = $params;
        }
    };
}

// endpoint!(pub ShiftDetail, "", Shifts, ShiftRes, ShiftPara) // Hella complicated (i.e., deep nesting) - can't be arsed
endpoint!(pub EmployeesDetails, "employees", Companies, EmployeesDetailsRes, EmployeesDetailsPara);
endpoint!(pub EmployeeDetails, "", Employees, EmployeeDetailsRes);
endpoint!(pub WorkingStatus, "working_status", Employees, WorkingStatusRes);
endpoint!(pub Onsite, "on_site", Companies, OnsiteRes);
endpoint!(pub Coworkers, "coworkers", Shifts, CoworkersRes);
endpoint!(pub Notifs, "notifications", Users, NotifRes, NotifPara);
endpoint!(pub Events, "events", Employees, EventsRes, EventsPara);

// auth uses patch
endpoint!(pub Auth, "", V3, AuthRes, method = PATCH);

// notification READ uses put
endpoint!(pub NotifRead, "READ", Notif, (), method = PUT); // note notif is a req part that extends from Notifs
