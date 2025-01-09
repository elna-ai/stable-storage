mod error;
mod storage;
// mod tests;

use crate::error::Error;

use candid::Principal;
use ic_cdk::export_candid;
use storage::{Asset, AssetArgs, STATE, STORE};

#[ic_cdk::init]
fn init() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let result = state.set(0);
        ic_cdk::println!("{:?}", result);
    });
}

// #[ic_cdk::update]
fn set_id(val: u8) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let result = state.set(val);
        ic_cdk::println!("set_id: {:?}", result)
    });
}

#[ic_cdk::query]
fn get_id() -> u8 {
    STATE.with(|state| {
        let state = state.borrow();
        let result = state.get();
        *result
    })
}

#[ic_cdk::update]
fn insert_doc(new_asset: AssetArgs) -> Result<u8, Error> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(Error::AnonymousCaller);
        // ic_cdk::trap("Anonymous Caller")
    }

    STORE.with(|store| {
        let mut store = store.borrow_mut();
        let last_id: u8 = get_id();
        let new_id = last_id + 1;
        let asset = Asset::from((new_asset, caller));
        store.insert(new_id.clone(), asset);
        set_id(new_id);
        Ok(new_id)
    })
}

#[ic_cdk::update]
fn update_doc(new_asset: AssetArgs, id: u8) -> Result<u8, Error> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(Error::AnonymousCaller);
        // ic_cdk::trap("Anonymous Caller")
    }

    STORE.with(|store| {
        let mut store = store.borrow_mut();
        let asset = Asset::from((new_asset, caller));

        store.insert(id.clone(), asset);
        Ok(id)
    })
}

#[ic_cdk::update]
fn delete_doc(id: u8) -> Result<u8, Error> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(Error::AnonymousCaller);
        // ic_cdk::trap("Anonymous Caller")
    }

    STORE.with(|store| {
        let mut store = store.borrow_mut();
        let asset = store.get(&id);
        match asset {
            Some(asset) => {
                if caller == asset.owner {
                    store.remove(&id);
                    Ok(id)
                } else {
                    Err(Error::Unauthorized)
                }
            }
            None => Err(Error::NotFound),
        }
    })
}

#[ic_cdk::query]
fn get_doc(id: u8) -> Result<AssetArgs, Error> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(Error::AnonymousCaller);
        // ic_cdk::trap("Anonymous Caller")
    }

    STORE.with(|store| {
        let store = store.borrow();
        let asset = store.get(&id);
        match asset {
            Some(asset) => {
                if caller == asset.owner {
                    Ok(AssetArgs::from(asset))
                } else {
                    Err(Error::Unauthorized)
                }
            }
            None => Err(Error::NotFound),
        }
    })
}

export_candid!();
