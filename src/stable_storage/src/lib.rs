mod error;
mod storage;

use crate::error::Error;

use candid::Principal;
use ic_cdk::export_candid;
use storage::{Asset, AssetArgs, STORE};

#[ic_cdk::update]
fn insert_doc(new_asset: AssetArgs) -> Result<u8, Error> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(Error::AnonymousCaller);
        // ic_cdk::trap("Anonymous Caller")
    }

    STORE.with(|store| {
        let mut store = store.borrow_mut();
        let len: u8 = store.len().try_into().unwrap();
        let id = len + 1;
        let asset = Asset::from((new_asset, caller));

        store.insert(id.clone(), asset);
        Ok(id)
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
