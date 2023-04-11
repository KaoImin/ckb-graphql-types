mod blockchain;
mod cell;
pub mod error;
mod hex;
mod transaction;

pub use cell::{CellDep, CellInput, CellOutput, OutPoint, Script, ScriptHashType};
pub use transaction::TransactionView;

use ckb_types::{packed, prelude::Unpack};

macro_rules! graphql_primitive {
    ($name: ident, $type_: ty) => {
        #[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
        pub struct $name(pub $type_);

        impl From<ckb_types::packed::$name> for $name {
            fn from(item: ckb_types::packed::$name) -> Self {
                use ckb_types::prelude::Unpack;
                Self::new(item.unpack())
            }
        }

        impl From<$name> for ckb_types::packed::$name {
            fn from(item: $name) -> Self {
                use ckb_types::prelude::Pack;
                item.0.pack()
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = crate::hex::clean_0x(s)?;
                Ok(Self(<$type_>::from_str_radix(&s, 16)?))
            }
        }

        impl $name {
            pub fn new(value: $type_) -> Self {
                Self(value)
            }

            #[cfg(test)]
            pub fn random() -> Self {
                Self::new(rand::random())
            }
        }

        #[async_graphql::Scalar]
        impl async_graphql::ScalarType for $name {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                use std::str::FromStr;

                if let async_graphql::Value::String(value) = &value {
                    return Self::from_str(&value)
                        .map_err(|e| async_graphql::InputValueError::custom(e));
                }
                Err(async_graphql::InputValueError::expected_type(value))
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(crate::hex::hex_uint(self.0))
            }
        }
    };

    ($name: ident, $len: expr) => {
        #[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
        pub struct $name(pub [u8; $len]);

        impl From<ckb_types::$name> for $name {
            fn from(item: ckb_types::$name) -> Self {
                Self(item.0)
            }
        }

        impl From<$name> for ckb_types::$name {
            fn from(item: $name) -> Self {
                Self(item.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let bytes = crate::hex::hex_decode(s)?;

                if bytes.len() != $len {
                    return Err(crate::error::Error::ParseBytes);
                }

                let mut array = [0u8; $len];
                array.copy_from_slice(&bytes);

                Ok(Self(array))
            }
        }

        #[async_graphql::Scalar]
        impl async_graphql::ScalarType for $name {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                use std::str::FromStr;

                if let async_graphql::Value::String(value) = &value {
                    return Self::from_str(&value)
                        .map_err(|e| async_graphql::InputValueError::custom(e));
                }
                Err(async_graphql::InputValueError::expected_type(value))
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(crate::hex::hex_encode(&self.0))
            }
        }

        impl $name {
            pub fn new(array: [u8; $len]) -> Self {
                Self(array)
            }

            #[cfg(test)]
            pub fn random() -> Self {
                let mut array = [0u8; $len];
                array.iter_mut().for_each(|x| *x = rand::random());
                Self(array)
            }
        }
    };

    ($name: ident) => {
        #[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
        pub struct $name(pub bytes::Bytes);

        impl From<Vec<u8>> for $name {
            fn from(item: Vec<u8>) -> Self {
                Self(item.into())
            }
        }

        impl From<$name> for Vec<u8> {
            fn from(item: $name) -> Self {
                item.0.into()
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let bytes = crate::hex::hex_decode(s)?;

                Ok(Self(bytes.into()))
            }
        }

        #[async_graphql::Scalar]
        impl async_graphql::ScalarType for $name {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                use std::str::FromStr;

                if let async_graphql::Value::String(value) = &value {
                    return Self::from_str(&value)
                        .map_err(|e| async_graphql::InputValueError::custom(e));
                }
                Err(async_graphql::InputValueError::expected_type(value))
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(crate::hex::hex_encode(&self.0))
            }
        }

        #[cfg(test)]
        impl $name {
            pub fn random() -> Self {
                (0..128)
                    .map(|_| rand::random::<u8>())
                    .collect::<Vec<_>>()
                    .into()
            }
        }
    };
}

/// Consecutive block number starting from 0.
///
/// This is a 64-bit unsigned integer type encoded as the 0x-prefixed hex
/// string. See examples of [Uint64](type.Uint64.html#examples).
pub type BlockNumber = Uint64;
/// Consecutive epoch number starting from 0.
///
/// This is a 64-bit unsigned integer type encoded as the 0x-prefixed hex
/// string. See examples of [Uint64](type.Uint64.html#examples).
pub type EpochNumber = Uint64;
/// The epoch indicator of a block. It shows which epoch the block is in, and
/// the elapsed epoch fraction after adding this block.
///
/// This is a 64-bit unsigned integer type encoded as the 0x-prefixed hex
/// string. See examples of [Uint64](type.Uint64.html#examples).
///
/// The lower 56 bits of the epoch field are split into 3 parts (listed in the
/// order from higher bits to lower bits):
///
/// * The highest 16 bits represent the epoch length
/// * The next 16 bits represent the current block index in the epoch, starting
///   from 0.
/// * The lowest 24 bits represent the current epoch number.
///
/// Assume there's a block, which number is 11555 and in epoch 50. The epoch 50
/// starts from block 11000 and have 1000 blocks. The epoch field for this
/// particular block will then be 1,099,520,939,130,930, which is calculated in
/// the following way:
///
/// ```text
/// 50 | ((11555 - 11000) << 24) | (1000 << 40)
/// ```
pub type EpochNumberWithFraction = Uint64;
/// The capacity of a cell is the value of the cell in Shannons. It is also the
/// upper limit of the cell occupied storage size where every 100,000,000
/// Shannons give 1-byte storage.
///
/// This is a 64-bit unsigned integer type encoded as the 0x-prefixed hex
/// string. See examples of [Uint64](type.Uint64.html#examples).
pub type Capacity = Uint64;
/// Count of cycles consumed by CKB VM to run scripts.
///
/// This is a 64-bit unsigned integer type encoded as the 0x-prefixed hex
/// string. See examples of [Uint64](type.Uint64.html#examples).
pub type Cycle = Uint64;
/// The Unix timestamp in milliseconds (1 second is 1000 milliseconds).
///
/// For example, 1588233578000 is Thu, 30 Apr 2020 07:59:38 +0000
///
/// This is a 64-bit unsigned integer type encoded as the 0x-prefixed hex
/// string. See examples of [Uint64](type.Uint64.html#examples).
pub type Timestamp = Uint64;
/// The simple increasing integer version.
///
/// This is a 32-bit unsigned integer type encoded as the 0x-prefixed hex
/// string. See examples of [Uint32](type.Uint32.html#examples).
pub type Version = Uint32;

graphql_primitive!(Uint32, u32);
graphql_primitive!(Uint64, u64);
graphql_primitive!(Uint128, u128);
graphql_primitive!(H160, 20);
graphql_primitive!(H256, 32);
graphql_primitive!(GraphqlBytes);

impl From<packed::Byte32> for H256 {
    fn from(value: packed::Byte32) -> Self {
        value.unpack().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_graphql_primitive {
		($($type_: ty)*) => {
			#[test]
			fn test_scalar() {
				use async_graphql::{OutputType, ScalarType};

				$(
					let case = <$type_>::random();
					let value = case.to_value();
					println!("case: {:?}, value: {:?}", case, value);
					assert_eq!(<$type_>::parse(value).expect(case.introspection_type_name().as_ref()), case);
				)*
			}
		};
	}

    test_graphql_primitive!(Uint32 Uint64 Uint128 H160 H256 GraphqlBytes);
}
