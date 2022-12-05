use std::collections::HashMap;
use std::hash::Hash;

use indexmap::IndexMap;

use crate::block::{
    BlockHash, BlockHeader, BlockNumber, BlockStatus, BlockTimestamp, GasPrice, GlobalRoot,
};
use crate::core::{ClassHash, ContractAddress, EntryPointSelector, Nonce, PatriciaKey};
use crate::hash::{StarkFelt, StarkHash};
use crate::state::{
    ContractClass, ContractClassAbiEntry, EntryPoint, EntryPointOffset, EntryPointType,
    EventAbiEntry, FunctionAbiEntry, FunctionAbiEntryType, FunctionAbiEntryWithType, Program,
    StorageKey, StructAbiEntry, StructMember, TypedParameter,
};
use crate::transaction::{
    CallData, ContractAddressSalt, DeclareTransaction, DeployAccountTransaction, DeployTransaction,
    EthAddress, EventContent, EventData, EventIndexInTransactionOutput, EventKey, Fee,
    InvokeTransaction, L1HandlerTransaction, L1ToL2Payload, L2ToL1Payload, MessageToL1,
    MessageToL2, Transaction, TransactionHash, TransactionOffsetInBlock, TransactionSignature,
    TransactionVersion,
};
use crate::{patky, shash};

pub trait GetTestInstance: Sized {
    fn get_test_instance() -> Self;
}

auto_impl_get_test_instance! {
    pub struct BlockHash(pub StarkHash);
    pub struct BlockHeader {
        pub block_hash: BlockHash,
        pub parent_hash: BlockHash,
        pub block_number: BlockNumber,
        pub gas_price: GasPrice,
        pub state_root: GlobalRoot,
        pub sequencer: ContractAddress,
        pub timestamp: BlockTimestamp,
    }
    pub struct BlockNumber(pub u64);
    pub enum BlockStatus {
        Pending = 0,
        AcceptedOnL2 = 1,
        AcceptedOnL1 = 2,
        Rejected = 3,
    }
    pub struct BlockTimestamp(pub u64);
    pub struct CallData(pub Vec<StarkFelt>);
    pub struct ClassHash(pub StarkHash);
    pub struct ContractAddressSalt(pub StarkHash);
    pub struct ContractClass {
        pub abi: Option<Vec<ContractClassAbiEntry>>,
        pub program: Program,
        pub entry_points_by_type: HashMap<EntryPointType, Vec<EntryPoint>>,
    }
    pub enum ContractClassAbiEntry {
        Event(EventAbiEntry) = 0,
        Function(FunctionAbiEntryWithType) = 1,
        Struct(StructAbiEntry) = 2,
    }
    pub struct DeclareTransaction {
        pub transaction_hash: TransactionHash,
        pub max_fee: Fee,
        pub version: TransactionVersion,
        pub signature: TransactionSignature,
        pub nonce: Nonce,
        pub class_hash: ClassHash,
        pub sender_address: ContractAddress,
    }
    pub struct DeployAccountTransaction {
        pub transaction_hash: TransactionHash,
        pub max_fee: Fee,
        pub version: TransactionVersion,
        pub signature: TransactionSignature,
        pub nonce: Nonce,
        pub class_hash: ClassHash,
        pub contract_address: ContractAddress,
        pub contract_address_salt: ContractAddressSalt,
        pub constructor_calldata: CallData,
    }
    pub struct DeployTransaction {
        pub transaction_hash: TransactionHash,
        pub version: TransactionVersion,
        pub class_hash: ClassHash,
        pub contract_address: ContractAddress,
        pub contract_address_salt: ContractAddressSalt,
        pub constructor_calldata: CallData,
    }
    pub struct EntryPoint {
        pub selector: EntryPointSelector,
        pub offset: EntryPointOffset,
    }
    pub struct EntryPointOffset(pub usize);
    pub struct EntryPointSelector(pub StarkHash);
    pub enum EntryPointType {
        Constructor = 0,
        External = 1,
        L1Handler = 2,
    }
    pub struct EventAbiEntry {
        pub name: String,
        pub keys: Vec<TypedParameter>,
        pub data: Vec<TypedParameter>,
    }
    pub struct EventContent {
        pub keys: Vec<EventKey>,
        pub data: EventData,
    }
    pub struct EventData(pub Vec<StarkFelt>);
    pub struct EventIndexInTransactionOutput(pub usize);
    pub struct EventKey(pub StarkFelt);
    pub struct Fee(pub u128);
    pub struct FunctionAbiEntry {
        pub name: String,
        pub inputs: Vec<TypedParameter>,
        pub outputs: Vec<TypedParameter>,
    }
    pub enum FunctionAbiEntryType {
        Constructor = 0,
        L1Handler = 1,
        Regular = 2,
    }
    pub struct FunctionAbiEntryWithType {
        pub r#type: FunctionAbiEntryType,
        pub entry: FunctionAbiEntry,
    }
    pub struct GasPrice(pub u128);
    pub struct GlobalRoot(pub StarkHash);
    pub struct InvokeTransaction {
        pub transaction_hash: TransactionHash,
        pub max_fee: Fee,
        pub version: TransactionVersion,
        pub signature: TransactionSignature,
        pub nonce: Nonce,
        pub contract_address: ContractAddress,
        pub entry_point_selector: Option<EntryPointSelector>,
        pub calldata: CallData,
    }
    pub struct L1ToL2Payload(pub Vec<StarkFelt>);
    pub struct L2ToL1Payload(pub Vec<StarkFelt>);
    pub struct MessageToL1 {
        pub to_address: EthAddress,
        pub payload: L2ToL1Payload,

    }
    pub struct MessageToL2 {
        pub from_address: EthAddress,
        pub payload: L1ToL2Payload,
    }
    pub struct Nonce(pub StarkFelt);
    pub struct Program {
        pub attributes: serde_json::Value,
        pub builtins: serde_json::Value,
        pub compiler_version: serde_json::Value,
        pub data: serde_json::Value,
        pub debug_info: serde_json::Value,
        pub hints: serde_json::Value,
        pub identifiers: serde_json::Value,
        pub main_scope: serde_json::Value,
        pub prime: serde_json::Value,
        pub reference_manager: serde_json::Value,
    }
    pub struct StructAbiEntry {
        pub name: String,
        pub size: usize,
        pub members: Vec<StructMember>,
    }
    pub struct StructMember {
        pub param: TypedParameter,
        pub offset: usize,
    }
    pub struct TypedParameter {
        pub name: String,
        pub r#type: String,
    }
    pub struct L1HandlerTransaction {
        pub transaction_hash: TransactionHash,
        pub version: TransactionVersion,
        pub nonce: Nonce,
        pub contract_address: ContractAddress,
        pub entry_point_selector: EntryPointSelector,
        pub calldata: CallData,
    }
    pub enum Transaction {
        Declare(DeclareTransaction) = 0,
        Deploy(DeployTransaction) = 1,
        DeployAccount(DeployAccountTransaction) = 2,
        Invoke(InvokeTransaction) = 3,
        L1Handler(L1HandlerTransaction) = 4,
    }
    pub struct TransactionHash(pub StarkHash);
    pub struct TransactionOffsetInBlock(pub usize);
    pub struct TransactionSignature(pub Vec<StarkFelt>);
    pub struct TransactionVersion(pub StarkFelt);
    bool;
    EthAddress;
    u8;
    u32;
    u64;
    u128;
    usize;
}

#[macro_export]
macro_rules! auto_impl_get_test_instance {
    () => {};
    // Tuple structs (no names associated with fields) - one field.
    ($(pub)? struct $name:ident($(pub)? $ty:ty); $($rest:tt)*) => {
        impl_get_test_instance!(struct $name ($ty));
        auto_impl_get_test_instance!($($rest)*);
    };
    // Tuple structs (no names associated with fields) - two fields.
    ($(pub)? struct $name:ident($(pub)? $ty0:ty, $(pub)? $ty1:ty) ; $($rest:tt)*) => {
        impl_get_test_instance!(struct $name ($ty0, $ty1));
        auto_impl_get_test_instance!($($rest)*);
    };
    // Structs with public fields.
    ($(pub)? struct $name:ident { $(pub $field:ident : $ty:ty ,)* } $($rest:tt)*) => {
        impl_get_test_instance!(struct $name { $(pub $field : $ty ,)* });
        auto_impl_get_test_instance!($($rest)*);
    };
    // Tuples - two elements.
    (($ty0:ty, $ty1:ty) ; $($rest:tt)*) => {
        impl_get_test_instance!(($ty0, $ty1));
        auto_impl_get_test_instance!($($rest)*);
    };
    // Tuples - three elements.
    (($ty0:ty, $ty1:ty, $ty2:ty) ; $($rest:tt)*) => {
        impl_get_test_instance!(($ty0, $ty1, $ty2));
        auto_impl_get_test_instance!($($rest)*);
    };
    // Enums.
    ($(pub)? enum $name:ident { $($variant:ident $( ($ty:ty) )? = $num:expr ,)* } $($rest:tt)*) => {
        impl_get_test_instance!(enum $name { $($variant $( ($ty) )? = $num ,)* });
        auto_impl_get_test_instance!($($rest)*);
    };
    // Primitive types.
    ($name:ident; $($rest:tt)*) => {
        impl_get_test_instance!($name);
        auto_impl_get_test_instance!($($rest)*);
    }
}
pub use auto_impl_get_test_instance;

////////////////////////////////////////////////////////////////////////
// Implements the [`GetTestInstance`] trait for the types that the
// [`auto_impl_get_test_instance`] macro is called with.
////////////////////////////////////////////////////////////////////////
#[macro_export]
macro_rules! impl_get_test_instance {
    // Tuple structs (no names associated with fields) - one field.
    (struct $name:ident($ty:ty)) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self(<$ty>::get_test_instance())
            }
        }
    };
    // Tuple structs (no names associated with fields) - two fields.
    (struct $name:ident($ty0:ty, $ty1:ty)) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self(<$ty0>::get_test_instance(), <$ty1>::get_test_instance())
            }
        }
    };
    // Structs with public fields.
    (struct $name:ident { $(pub $field:ident : $ty:ty ,)* }) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self {
                    $(
                        $field: <$ty>::get_test_instance(),
                    )*
                }
            }
        }
    };
    // Tuples - two elements.
    (($ty0:ty, $ty1:ty)) => {
        impl GetTestInstance for ($ty0, $ty1) {
            fn get_test_instance() -> Self {
                (
                    <$ty0>::get_test_instance(),
                    <$ty1>::get_test_instance(),
                )
            }
        }
    };
    // Tuples - three elements.
    (($ty0:ty, $ty1:ty, $ty2:ty)) => {
        impl GetTestInstance for ($ty0, $ty1, $ty2) {
            fn get_test_instance() -> Self {
                (
                    <$ty0>::get_test_instance(),
                    <$ty1>::get_test_instance(),
                    <$ty2>::get_test_instance(),
                )
            }
        }
    };
    // Enums with no inner struct.
    (enum $name:ident { $variant:ident = $num:expr , $($rest:tt)* }) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self::$variant
            }
        }
    };
    // Enums with inner struct.
    (enum $name:ident { $variant:ident ($ty:ty) = $num:expr , $($rest:tt)* }) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self::$variant(<$ty>::get_test_instance())
            }
        }
    };
    // Primitive types.
    ($name:ident) => {
        impl GetTestInstance for $name {
            fn get_test_instance() -> Self {
                Self::default()
            }
        }
    }
}
pub use impl_get_test_instance;

////////////////////////////////////////////////////////////////////////
// Implements the [`GetTestInstance`] trait for primitive types.
////////////////////////////////////////////////////////////////////////
impl GetTestInstance for serde_json::Value {
    fn get_test_instance() -> Self {
        serde_json::from_str(r#""0x1""#).unwrap()
    }
}
impl GetTestInstance for String {
    fn get_test_instance() -> Self {
        "a".to_string()
    }
}
impl<T: GetTestInstance> GetTestInstance for Option<T> {
    fn get_test_instance() -> Self {
        Some(T::get_test_instance())
    }
}
impl<T: GetTestInstance> GetTestInstance for Vec<T> {
    fn get_test_instance() -> Self {
        vec![T::get_test_instance()]
    }
}
impl<K: GetTestInstance + Eq + Hash, V: GetTestInstance> GetTestInstance for HashMap<K, V> {
    fn get_test_instance() -> Self {
        let mut res = HashMap::with_capacity(1);
        let k = K::get_test_instance();
        let v = V::get_test_instance();
        res.insert(k, v);
        res
    }
}
impl<K: GetTestInstance + Eq + Hash, V: GetTestInstance> GetTestInstance for IndexMap<K, V> {
    fn get_test_instance() -> Self {
        let mut res = IndexMap::with_capacity(1);
        let k = K::get_test_instance();
        let v = V::get_test_instance();
        res.insert(k, v);
        res
    }
}
impl<T: GetTestInstance + Default + Copy, const N: usize> GetTestInstance for [T; N] {
    fn get_test_instance() -> Self {
        [T::get_test_instance(); N]
    }
}

////////////////////////////////////////////////////////////////////////
// Implements the [`GetTestInstance`] trait for types not supported
// by the macro [`impl_get_test_instance`].
////////////////////////////////////////////////////////////////////////
impl GetTestInstance for StarkHash {
    fn get_test_instance() -> Self {
        shash!("0x1")
    }
}

impl GetTestInstance for ContractAddress {
    fn get_test_instance() -> Self {
        Self(patky!("0x1"))
    }
}

impl GetTestInstance for StorageKey {
    fn get_test_instance() -> Self {
        Self(patky!("0x1"))
    }
}