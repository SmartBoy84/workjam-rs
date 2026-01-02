// this file contains the different parts of the API

use std::marker::PhantomData;

use crate::backend::request::{
    HasApprovalReqID, HasLocationID, HasNotificationID, HasShiftID,
    endpoints::{ApprovalReqs, Notifs},
};

use super::{HasCompanyID, HasEmployeeID, RequestConfig, RequestPart, SerialiseRequestPart};

macro_rules! request_part {
    ($name: ident, $word: literal, $default: ty) => {
        pub struct $name<T: RequestPart = $default>(PhantomData<T>);
        impl<T: RequestPart> RequestPart for $name<T> {}

        impl<C: RequestConfig, T: SerialiseRequestPart<C>> SerialiseRequestPart<C> for $name<T> {
            const WORD: &str = $word;
            type Next = T;
        }
    };

    ($name: ident, $word: literal, $default: ty, $config: path, $getter: ident) => {
        pub struct $name<T: RequestPart = $default>(PhantomData<T>);
        impl<T: RequestPart> RequestPart for $name<T> {}

        impl<C: RequestConfig + $config, T: SerialiseRequestPart<C>> SerialiseRequestPart<C>
            for $name<T>
        {
            const WORD: &str = $word;
            type Next = T;

            fn get_val(config: &C) -> Option<&str> {
                Some(config.$getter())
            }
        }
    };
}

// the defaults as set such that they include the most points possible
// e.g., default of Companies is v4, because more require v4/Companies than v1/Companies

// parts after endpoints - specific actions
request_part!(Notif, "", Notifs, HasNotificationID, notification_id); // to handle things about a specific notification
request_part!(
    ApprovalReq,
    "",
    ApprovalReqs,
    HasApprovalReqID,
    approval_req_id
); // to handle things about a specific approval req

// standalone parts
request_part!(Shifts, "shifts", Locations, HasShiftID, shift_id);
request_part!(
    Locations,
    "locations",
    Companies,
    HasLocationID,
    location_id
);
request_part!(Users, "users", Companies::<V1>, HasEmployeeID, employee_id);
request_part!(
    Employees,
    "employees",
    Companies,
    HasEmployeeID,
    employee_id
);
request_part!(Companies, "companies", V4, HasCompanyID, company_id);
request_part!(Auth, "auth", ());
request_part!(V1, "v1", ());
request_part!(V3, "v3", Auth);
request_part!(V4, "v4", ());
request_part!(V5, "v5", ());

impl RequestPart for () {}

impl<C: RequestConfig> SerialiseRequestPart<C> for () {
    const WORD: &str = "";
    type Next = ();

    fn add_str(_s: &mut String, _config: &C) {
        ()
    }
}
