use ic_cdk_macros::*;
use ic_cdk::{api, storage};
use candid::{candid_method, Principal, Nat};

struct Counter(Principal);
struct Owner(Principal);

impl Default for Counter {
    fn default() -> Self {
        Counter(Principal::anonymous())
    }
}

impl Default for Owner {
    fn default() -> Self {
        Owner(Principal::anonymous())
    }
}

#[init]
#[candid_method(init)]
fn init() {
    let caller = ic_cdk::caller();
    let owner = storage::get_mut::<Owner>();
    *owner = Owner(caller);
}

#[update(name = "setCounter")]
#[candid_method(update, rename = "setCounter")]
fn set_counter(counter: Principal) {
    let caller = ic_cdk::caller();
    let owner = storage::get::<Owner>();
    assert_eq!(caller, owner.0);

    let _counter = storage::get_mut::<Counter>();
    _counter.0 = counter;
}


#[update]
#[candid_method]
async fn get(counter: Principal) -> Nat {
    let result: Result<(Nat,), _> = api::call::call(counter, "get", ()).await;
    result.unwrap().0
}

#[update]
#[candid_method]
async fn add(n: Nat) -> Nat {
    let counter = storage::get::<Counter>();
    let result: Result<(Nat,), _> = api::call::call(counter.0, "add", (n, )).await;
    result.unwrap().0
}

#[update(name = "msgCaller")]
#[candid_method(update, rename = "msgCaller")]
async fn msg_caller() -> Principal {
    let counter = storage::get::<Counter>();
    let result: Result<(Principal,), _> = api::call::call(counter.0, "msgCaller", ()).await;
    result.unwrap().0
}

#[update]
#[candid_method]
async fn set(n: Nat) {
    let counter = storage::get::<Counter>();
    let _: Result<(), _> = api::call::call(counter.0, "set", (n, )).await;
}

#[update]
#[candid_method]
async fn inc() {
    let counter = storage::get::<Counter>();
    let _: Result<(), _> = api::call::call(counter.0, "inc", ()).await;
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}