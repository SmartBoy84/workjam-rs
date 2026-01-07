use restman_rs::{PATCH, POST, PUT, endpoint};

use crate::{
    Workjam, parameters::{ApprovalReqPara, EmployeesDetailsPara, EventsPara, NotifPara}, parts::{ApprovalReq, Companies, Employees, Notif, Shifts, Users, V3, V5}, requests::{AuthRes, OnsiteRes, WorkingStatusRes, approval::ApprovalReqsRes, coworkers::CoworkersRes, employee::EmployeeDetailsRes, events::EventsRes, notifications::NotifRes, shift::ShiftRes}
};

endpoint!(Workjam, pub ShiftDetail, "", Shifts, ShiftRes); // Hella complicated (i.e., deep nesting) - can't be arsed
endpoint!(Workjam, pub EmployeesDetails, "employees", Companies, EmployeeDetailsRes, EmployeesDetailsPara);
endpoint!(Workjam, pub EmployeeDetails, "", Employees, EmployeeDetailsRes);

// approval reqs needs V5, right now it's fine to manually specify it but overtime I will create a separate CompaniesV5 endpoint if a lot are like this
endpoint!(Workjam, pub ApprovalReqs, "approval_requests", Employees::<Companies::<V5>>, ApprovalReqsRes, ApprovalReqPara);

endpoint!(Workjam, pub WorkingStatus, "working_status", Employees, WorkingStatusRes);
endpoint!(Workjam, pub Onsite, "on_site", Companies, OnsiteRes);
endpoint!(Workjam, pub Coworkers, "coworkers", Shifts, CoworkersRes);
endpoint!(Workjam, pub Notifs, "notifications", Users, NotifRes, NotifPara);
endpoint!(Workjam, pub Events, "events", Employees, EventsRes, EventsPara);

// auth uses patch
endpoint!(Workjam, pub Auth, "", V3, AuthRes, method = PATCH);

// notification READ uses put
endpoint!(Workjam, pub NotifRead, "READ", Notif, (), method = PUT); // note notif is a req part that extends from Notifs

// approval request accept uses post
endpoint!(Workjam, pub AcceptApprovalReq, "accept", ApprovalReq, ShiftRes,  method = POST);
