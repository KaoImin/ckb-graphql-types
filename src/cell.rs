use std::fmt::{Display, Error, Formatter};

use async_graphql::{Enum, SimpleObject};
use ckb_types::{packed, prelude::*};

use crate::{Capacity, GraphqlBytes, Uint32, Uint64, H256};

/// Specifies how the script `code_hash` is used to match the script code and
/// how to run the code.
///
/// Allowed kinds: "data", "type" and "data1".
///
/// Refer to the section [Code Locating](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0022-transaction-structure/0022-transaction-structure.md#code-locating)
/// and [Upgradable Script](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0022-transaction-structure/0022-transaction-structure.md#upgradable-script)
/// in the RFC *CKB Transaction Structure*.
#[derive(Enum, Default, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[graphql(remote = "ckb_types::core::ScriptHashType")]
pub enum ScriptHashType {
    #[default]
    /// Type "data" matches script code via cell data hash, and run the script
    /// code in v0 CKB VM.
    Data = 0,
    /// Type "type" matches script code via cell type script hash.
    Type = 1,
    /// Type "data1" matches script code via cell data hash, and run the script
    /// code in v1 CKB VM.
    Data1 = 2,
}

impl Display for ScriptHashType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Data => write!(f, "data"),
            Self::Type => write!(f, "type"),
            Self::Data1 => write!(f, "data1"),
        }
    }
}

impl From<packed::Byte> for ScriptHashType {
    fn from(value: packed::Byte) -> Self {
        match value.as_slice()[0] {
            0 => Self::Data,
            1 => Self::Type,
            2 => Self::Data1,
            _ => unreachable!("invalid script hash type"),
        }
    }
}

/// Describes the lock script and type script for a cell.
#[derive(SimpleObject, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Script {
    /// The hash used to match the script code.
    pub code_hash: H256,
    /// Specifies how to use the `code_hash` to match the script code.
    pub hash_type: ScriptHashType,
    /// Arguments for script.
    pub args:      GraphqlBytes,
}

impl From<packed::Script> for Script {
    fn from(value: packed::Script) -> Self {
        Self {
            code_hash: value.code_hash().unpack().into(),
            hash_type: value.hash_type().into(),
            args:      GraphqlBytes(value.args().unpack()),
        }
    }
}

impl From<Script> for packed::Script {
    fn from(value: Script) -> Self {
        Self::new_builder()
            .code_hash(value.code_hash.0.pack())
            .hash_type(packed::Byte::new(value.hash_type as u8))
            .args(value.args.0.pack())
            .build()
    }
}

/// The fields of an output cell except the cell data.
#[derive(SimpleObject, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CellOutput {
    /// The cell capacity.
    ///
    /// The capacity of a cell is the value of the cell in Shannons. It is also
    /// the upper limit of the cell occupied storage size where every
    /// 100,000,000 Shannons give 1-byte storage.
    pub capacity: Capacity,
    /// The lock script.
    pub lock:     Script,
    /// The optional type script.
    pub type_:    Option<Script>,
}

impl From<packed::CellOutput> for CellOutput {
    fn from(value: packed::CellOutput) -> Self {
        Self {
            capacity: Capacity::new(value.capacity().unpack()),
            lock:     value.lock().into(),
            type_:    value.type_().to_opt().map(Into::into),
        }
    }
}

impl From<CellOutput> for packed::CellOutput {
    fn from(value: CellOutput) -> Self {
        Self::new_builder()
            .capacity(value.capacity.0.pack())
            .lock(value.lock.into())
            .type_(value.type_.map(packed::Script::from).pack())
            .build()
    }
}

/// Reference to a cell via transaction hash and output index.
#[derive(SimpleObject, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OutPoint {
    /// Transaction hash in which the cell is an output.
    pub tx_hash: H256,
    /// The output index of the cell in the transaction specified by `tx_hash`.
    pub index:   Uint32,
}

impl From<packed::OutPoint> for OutPoint {
    fn from(value: packed::OutPoint) -> Self {
        Self {
            tx_hash: value.tx_hash().unpack().into(),
            index:   Uint32::new(value.index().unpack()),
        }
    }
}

impl From<OutPoint> for packed::OutPoint {
    fn from(value: OutPoint) -> Self {
        Self::new_builder()
            .tx_hash(value.tx_hash.0.pack())
            .index(value.index.0.pack())
            .build()
    }
}

/// The input cell of a transaction.
#[derive(SimpleObject, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CellInput {
    /// Restrict when the transaction can be committed into the chain.
    ///
    /// See the RFC [Transaction valid since](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0017-tx-valid-since/0017-tx-valid-since.md).
    pub since:           Uint64,
    /// Reference to the input cell.
    pub previous_output: OutPoint,
}

impl From<packed::CellInput> for CellInput {
    fn from(value: packed::CellInput) -> Self {
        Self {
            since:           Uint64::new(value.since().unpack()),
            previous_output: value.previous_output().into(),
        }
    }
}

impl From<CellInput> for packed::CellInput {
    fn from(value: CellInput) -> Self {
        Self::new_builder()
            .since(value.since.0.pack())
            .previous_output(value.previous_output.into())
            .build()
    }
}

/// The dep cell type. Allowed values: "code" and "dep_group".
#[derive(Enum, Default, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[graphql(remote = "ckb_types::core::DepType")]
pub enum DepType {
    /// Type "code".
    ///
    /// Use the cell itself as the dep cell.
    #[default]
    Code,
    /// Type "dep_group".
    ///
    /// The cell is a dep group which members are cells. These members are used
    /// as dep cells instead of the group itself.
    ///
    /// The dep group stores the array of `OutPoint`s serialized via molecule in
    /// the cell data. Each `OutPoint` points to one cell member.
    DepGroup,
}

impl From<packed::Byte> for DepType {
    fn from(value: packed::Byte) -> Self {
        match value.as_slice()[0] {
            0 => Self::Code,
            1 => Self::DepGroup,
            _ => unreachable!("invalid dep type"),
        }
    }
}

/// The cell dependency of a transaction.
#[derive(SimpleObject, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CellDep {
    /// Reference to the cell.
    pub out_point: OutPoint,
    /// Dependency type.
    pub dep_type:  DepType,
}

impl From<packed::CellDep> for CellDep {
    fn from(value: packed::CellDep) -> Self {
        Self {
            out_point: value.out_point().into(),
            dep_type:  value.dep_type().into(),
        }
    }
}

impl From<CellDep> for packed::CellDep {
    fn from(value: CellDep) -> Self {
        Self::new_builder()
            .out_point(value.out_point.into())
            .dep_type(packed::Byte::new(value.dep_type as u8))
            .build()
    }
}
