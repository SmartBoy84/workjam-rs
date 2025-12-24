// this file contains the different parts of the API

use std::marker::PhantomData;

use crate::backend::request::{HasLocationID, HasShiftID};

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
// e.g., default of Company is v4, because more require v4/company than v1/company

request_part!(Shifts, "shifts", Locations, HasShiftID, shift_id);
request_part!(Locations, "locations", Company, HasLocationID, location_id);
request_part!(Users, "users", Company::<V1>, HasEmployeeID, employee_id);
request_part!(Employees, "employees", Company, HasEmployeeID, employee_id);
request_part!(Company, "companies", V4, HasCompanyID, company_id);
request_part!(V1, "v1", ());
request_part!(Auth, "auth", ());
request_part!(V3, "v3", Auth);
request_part!(V4, "v4", ());

impl RequestPart for () {}

impl<C: RequestConfig> SerialiseRequestPart<C> for () {
    const WORD: &str = "";
    type Next = ();

    fn add_str(_s: &mut String, _config: &C) {
        ()
    }
}
