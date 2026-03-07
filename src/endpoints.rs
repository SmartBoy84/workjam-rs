use restman_rs::{GET, PATCH, POST, PUT, endpoint};

use crate::{
    Workjam,
    parameters::{ApprovalReqsPara, EmployeesDetailsPara, EventsPara, NotifPara},
    parts::{ApprovalReq, Availability, Companies, Employees, Notif, Shifts, Users, V3, V5},
    requests::{
        AuthRes, OnsiteRes, WorkingStatusRes,
        approval::ApprovalReqsRes,
        availability::{AvailabilityPayload, AvailabilityRes},
        coworkers::CoworkersRes,
        employee::{EmployeeDetailsRes, EmployeesDetailsRes},
        events::EventsRes,
        notifications::NotifRes,
        shift::ShiftRes,
    },
};

endpoint!(Workjam, pub ShiftDetail, "", Shifts, ShiftRes, (), (), GET);
endpoint!(Workjam, pub EmployeesDetails, "employees", Companies, EmployeesDetailsRes, EmployeesDetailsPara, (), GET); // not using Employees part, because that would require an employee id to be appended as well
endpoint!(Workjam, pub EmployeeDetails, "", Employees, EmployeeDetailsRes, (), (), GET);

// approval reqs needs V5, right now it's fine to manually specify it but overtime I will create a separate CompaniesV5 endpoint if a lot are like this
endpoint!(Workjam, pub ApprovalReqs, "approval_requests", Employees::<Companies::<V5>>, ApprovalReqsRes, ApprovalReqsPara, (), GET);

endpoint!(Workjam, pub WorkingStatus, "working_status", Employees, WorkingStatusRes, (), (), GET);
endpoint!(Workjam, pub Onsite, "on_site", Companies, OnsiteRes, (), (), GET);
endpoint!(Workjam, pub Coworkers, "coworkers", Shifts, CoworkersRes, (), (), GET);
endpoint!(Workjam, pub Notifs, "notifications", Users, NotifRes, NotifPara, (), GET);
endpoint!(Workjam, pub Events, "events", Employees, EventsRes, EventsPara, (), GET);

// auth uses patch
endpoint!(Workjam, pub Auth, "", V3, AuthRes, (), (), PATCH);

// notification READ uses put
endpoint!(Workjam, pub NotifRead, "READ", Notif, (), (), (), PUT); // note notif is a req part that extends from Notifs

// approval request accept uses post (no payload though)
endpoint!(Workjam, pub AcceptApprovalReq, "accept", ApprovalReq, ShiftRes, (), (),  POST);

// availability updates use POST with JSON payload
// fully specified 'url' because it doesn't take the employees detail
endpoint!(Workjam, pub SubmitAvailability, "submit", Availability::<ApprovalReqs::<Companies::<V5>>>, AvailabilityRes, (), AvailabilityPayload, POST);
