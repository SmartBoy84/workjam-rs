# Workjam-rs
> Rust bindings for the \(reverse engineered) [Workjam](https://workjam.com) shift management REST API   
# Obtaining token
As this API is reverse-engineered, Workjam does not provide an easy method of obtaining the bearer token needed. However, it is quite easy to find this and it seems to not have an expiry.  
1. Login at [Workjam](https://workjam.com)
2. In the same browser session, visit [`https://api.workjam.com/api/`](https://api.workjam.com/api/)
3. The value of the `token` cookie is the bearer token
> Cookies can be found via the inspect element console of most modern browsers
# Using the crate
```rust
use workjam_rs::{
    WorkjamUser,
    backend::request::{
        WorkjamRequest,
        config::WorkjamRequestConfig,
        endpoints,
        parameters::NotifPara,
        payload::{AuthRes, notifications::NotifRes},
    },
};

const TOKEN: &str = "{bearer token}";

fn main() {
    let client = WorkjamUser::new(TOKEN);
    // let client = WorkjamUser::new_with_backend(<custom backend>, TOKEN)

    let AuthRes { employers, user_id } = client.get_auth().unwrap();

    let my_employer = employers.into_iter().next().unwrap();
    let my_id = user_id.to_string();

    let my_config = WorkjamRequestConfig::new()
        .company_id(my_employer)
        .employee_id(&my_id); // Uses `Cow<'a, str>` under the hood so can borrow or own

    let notifs_req = WorkjamRequest::<endpoints::Notifs>::new_with_para(
        &my_config,
        NotifPara::builder().offset(0).size(100).build(),
    );

    let NotifRes { notifications } = client.request(&notifs_req).unwrap();

    println!(
        "{:?}",
        notifications
            .iter()
            .map(|notif| &notif.body)
            .collect::<Vec<_>>()
    );

    // set all notifications to read
    notifications
        .iter()
        .for_each(|notif| client.request(&notif.set_read(&my_config)).unwrap());
}
```
## Notes
As per above, the user-facing interface for the crate is purposely simple:
- Instantiate a `WorkjamUser` using your token
- Get employee details via `get_auth()`
- Create a `WorkjamRequestConfig`
- Create an API request using `WorkjamRequest::<{endpoint}>::new()`
- Fire away!
### Static correctness
With how I've constructed this crate, at compile time every request is statically guaranteed to be correct.  

If an endpoint requires specific parameters then the user is forced to instantiate with `new_with_para` and provide the correct paramter struct..  

The Workjam API encodes certain config parameters in the API URL itself (e.g., `/api/v4/location/123-456-789/...)`. These details are provided in the `WorkjamRequestConfig` and using type states the user must provide an adequately qualified struct *at compile time*.  
### Pluggable thingamajigies!
- In the crate I have provided the user with a backend powered by the `ureq` crate - this is the default, used when the user uses `.new()`
- However, you can write your own backend, adhering to the `WorkjamHttpClient` trait and use `.new_with_backend()`

# Credits/main crates used
> By the way, you should totally look at my method of verifying URL correctness at compile-time using a type-state based linked list! Find it [here](https://github.com/SmartBoy84/workjam-rs/blob/main/src/backend/request/parts.rs).  
- `Serde`; [de]serialisation of JSON payloads
- `Bon`; construction of easy struct builders
- `Chrono`; time parsing (surprisingly difficult!)
- `Ureq`; HTTP handler
- `thiserror`; macros for simplifying library error construction