// this file contains the different parts of the API

use crate::{
    config::{
        HasApprovalReqID, HasCompanyID, HasEmployeeID, HasLocationID, HasNotificationID, HasShiftID,
    },
    endpoints::{ApprovalReqs, Notifs},
};
use restman_rs::request_part;

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

// // standalone parts
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
