use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use super::{
    RequestConfig, RequestPart, SerialiseRequestPart,
    parameters::{EventsPara, NotifPara, QueryParameters},
    parts::{Employees, Shifts, Users, V3},
    payload::{AuthRes, coworkers::CoworkersRes, events::EventsRes, notifications::NotifRes},
};

pub trait Endpoint {
    type Res: DeserializeOwned;
}
pub trait EndpointWithParameters: Endpoint {
    type P: QueryParameters;
}

macro_rules! endpoint {
    // Simple endpoint: no parameters
    ($vis:vis $name:ident, $word:literal, $default:ty, $res:ty) => {
        $vis struct $name<T: RequestPart = $default>(PhantomData<T>);

        impl<T: RequestPart> Endpoint for $name<T> {
            type Res = $res;
        }

        impl<T: RequestPart> RequestPart for $name<T> {}

        impl<C: RequestConfig, T: SerialiseRequestPart<C>> SerialiseRequestPart<C> for $name<T> {
            const WORD: &str = $word;
            type Next = T;
        }
    };

    // Endpoint with query parameters
    ($vis:vis $name:ident, $word:literal, $default:ty, $res:ty, $params:ty) => {
        endpoint!($vis $name, $word, $default, $res);

        impl<T: RequestPart> EndpointWithParameters for $name<T> {
            type P = $params;
        }
    };
}

// endpoint!(pub ShiftDetail, "", Shifts, ShiftRes, ShiftPara) // Hella complicated (i.e., deep nesting) - can't be arsed
endpoint!(pub Coworkers, "coworkers", Shifts, CoworkersRes);
endpoint!(pub Notif, "notifications", Users, NotifRes, NotifPara);
endpoint!(pub Auth, "", V3, AuthRes);
endpoint!(pub Events, "events", Employees, EventsRes, EventsPara);
