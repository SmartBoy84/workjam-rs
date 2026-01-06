use restman_rs::{PATCH, POST, PUT, endpoint};

use crate::{
    Workjam, parameters::{ApprovalReqPara, EmployeesDetailsPara, EventsPara, NotifPara}, parts::{ApprovalReq, Companies, Employees, Notif, Shifts, Users, V3, V5}, requests::{AuthRes, OnsiteRes, WorkingStatusRes, approval::ApprovalReqsRes, coworkers::CoworkersRes, employee::EmployeeDetailsRes, events::EventsRes, notifications::NotifRes, shift::ShiftRes}
};

endpoint!(pub ShiftDetail, "", Shifts, ShiftRes, Workjam); // Hella complicated (i.e., deep nesting) - can't be arsed
endpoint!(pub EmployeesDetails, "employees", Companies, EmployeeDetailsRes, EmployeesDetailsPara, Workjam);
endpoint!(pub EmployeeDetails, "", Employees, EmployeeDetailsRes, Workjam);

// approval reqs needs V5, right now it's fine to manually specify it but overtime I will create a separate CompaniesV5 endpoint if a lot are like this
endpoint!(pub ApprovalReqs, "approval_requests", Employees::<Companies::<V5>>, ApprovalReqsRes, ApprovalReqPara, Workjam);

endpoint!(pub WorkingStatus, "working_status", Employees, WorkingStatusRes, Workjam);
endpoint!(pub Onsite, "on_site", Companies, OnsiteRes, Workjam);
endpoint!(pub Coworkers, "coworkers", Shifts, CoworkersRes, Workjam);
endpoint!(pub Notifs, "notifications", Users, NotifRes, NotifPara, Workjam);
endpoint!(pub Events, "events", Employees, EventsRes, EventsPara, Workjam);

// auth uses patch
endpoint!(pub Auth, "", V3, AuthRes, method = PATCH, Workjam);

// notification READ uses put
endpoint!(pub NotifRead, "READ", Notif, (), method = PUT, Workjam); // note notif is a req part that extends from Notifs

// approval request accept uses post
endpoint!(pub AcceptApprovalReq, "accept", ApprovalReq, ShiftRes,  method = POST, Workjam);
