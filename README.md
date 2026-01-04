# Workjam-rs
> Rust bindings for the \(reverse engineered) [Workjam](https://workjam.com) shift management REST API   
# Obtaining token
As this API is reverse-engineered, Workjam does not provide an easy method of obtaining the bearer token needed. However, it is quite easy to find this and it seems to not have an expiry.  
1. Login at [Workjam](https://workjam.com)
2. In the same browser session, visit [`https://api.workjam.com/api/`](https://api.workjam.com/api/)
3. The value of the `token` cookie is the bearer token
> Cookies can be found via the inspect element console of most modern browsers
# Example
```rust
fn main() {
    todo!("gimme an example boi!");
}
```
# Credits/main crates used
> By the way, you should totally look at my method of verifying URL correctness at compile-time using a type-state based linked list! Find it [here](https://github.com/SmartBoy84/workjam-rs/blob/main/src/backend/request/parts.rs).  
- `Serde`; [de]serialisation of JSON payloads
- `Bon`; construction of easy struct builders
- `Chrono`; time parsing (surprisingly difficult!)
- `Ureq`; HTTP handler
- `thiserror`; macros for simplifying library error construction