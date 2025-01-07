// use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::StableBTreeMap;
use ic_stable_structures::{storable::Bound, Storable};
use serde::Serialize;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// // A memory for upgrades, where data from the heap can be serialized/deserialized.
const STORE_MEM: MemoryId = MemoryId::new(0);
// const STATE_MEM: MemoryId = MemoryId::new(1);

fn get_store_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STORE_MEM))
}

// fn get_state_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(STATE_MEM))
// }

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Document {
    pub page_content: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub owner: Principal,
    document: Document,
    file_name: String,
    uploaded_at: u64,
}

// impl Default for Asset {
//     fn default() -> Self {
//         Asset {
//             owner: Principal::anonymous(),
//             document: Document::default(),
//             file_name: String::from("untitled.txt"),
//         }
//     }
// }

#[derive(CandidType, Deserialize)]
pub struct AssetArgs {
    document: Document,
    file_name: String,
}

impl From<(AssetArgs, Principal)> for Asset {
    fn from((arg, owned_by): (AssetArgs, Principal)) -> Self {
        Self {
            owner: owned_by,
            document: arg.document,
            file_name: arg.file_name,
            uploaded_at: ic_cdk::api::time(),
        }
    }
}

impl From<Asset> for AssetArgs {
    fn from(arg: Asset) -> Self {
        Self {
            document: arg.document,
            file_name: arg.file_name,
        }
    }
}

impl Storable for Asset {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static STORE: RefCell<StableBTreeMap<u8,Asset,Memory>> = RefCell::new(
        StableBTreeMap::init(get_store_memory())
    );


    // pub static USERS: RefCell<StableBTreeMap<String,Principal,Memory>> =  RefCell::new(
    //     StableBTreeMap::init(get_state_memory())
    // );

}
