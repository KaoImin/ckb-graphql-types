use async_graphql::SimpleObject;
use ckb_types::{core, packed, prelude::*};

use crate::{CellDep, CellInput, CellOutput, GraphqlBytes, Version, H256};

/// The transaction view.
///
/// Refer to RFC [CKB Transaction Structure](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0022-transaction-structure/0022-transaction-structure.md).
#[derive(SimpleObject, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TransactionView {
    /// Reserved for future usage. It must equal 0 in current version.
    pub version:      Version,
    /// An array of cell deps.
    ///
    /// CKB locates lock script and type script code via cell deps. The script
    /// also can uses syscalls to read the cells here.
    ///
    /// Unlike inputs, the live cells can be used as cell deps in multiple
    /// transactions.
    pub cell_deps:    Vec<CellDep>,
    /// An array of header deps.
    ///
    /// The block must already be in the canonical chain.
    ///
    /// Lock script and type script can read the header information of blocks
    /// listed here.
    pub header_deps:  Vec<H256>,
    /// An array of input cells.
    ///
    /// In the canonical chain, any cell can only appear as an input once.
    pub inputs:       Vec<CellInput>,
    /// An array of output cells.
    pub outputs:      Vec<CellOutput>,
    /// Output cells data.
    ///
    /// This is a parallel array of outputs. The cell capacity, lock, and type
    /// of the output i is `outputs[i]` and its data is `outputs_data[i]`.
    pub outputs_data: Vec<GraphqlBytes>,
    /// An array of variable-length binaries.
    ///
    /// Lock script and type script can read data here to verify the
    /// transaction.
    ///
    /// For example, the bundled secp256k1 lock script requires storing the
    /// signature in `witnesses`.
    pub witnesses:    Vec<GraphqlBytes>,
    /// The transaction hash.
    pub hash:         H256,
}

impl From<packed::Transaction> for TransactionView {
    fn from(value: packed::Transaction) -> Self {
        let raw = value.raw();

        Self {
            version:      raw.version().into(),
            cell_deps:    raw.cell_deps().into_iter().map(Into::into).collect(),
            header_deps:  raw.header_deps().into_iter().map(Into::into).collect(),
            inputs:       raw.inputs().into_iter().map(Into::into).collect(),
            outputs:      raw.outputs().into_iter().map(Into::into).collect(),
            outputs_data: raw
                .outputs_data()
                .into_iter()
                .map(|data| GraphqlBytes(data.unpack()))
                .collect(),
            witnesses:    value
                .witnesses()
                .into_iter()
                .map(|witness| GraphqlBytes(witness.unpack()))
                .collect(),
            hash:         value.calc_tx_hash().into(),
        }
    }
}

impl From<TransactionView> for packed::Transaction {
    fn from(value: TransactionView) -> Self {
        let raw = packed::RawTransaction::new_builder()
            .version(value.version.into())
            .cell_deps(
                value
                    .cell_deps
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .pack(),
            )
            .header_deps(
                value
                    .header_deps
                    .into_iter()
                    .map(|h| h.0.pack())
                    .collect::<Vec<_>>()
                    .pack(),
            )
            .inputs(
                value
                    .inputs
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .pack(),
            )
            .outputs(
                value
                    .outputs
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .pack(),
            )
            .outputs_data(
                value
                    .outputs_data
                    .into_iter()
                    .map(|data| data.0.pack())
                    .collect::<Vec<_>>()
                    .pack(),
            )
            .build();

        packed::Transaction::new_builder()
            .raw(raw)
            .witnesses(
                value
                    .witnesses
                    .into_iter()
                    .map(|witness| witness.0.pack())
                    .collect::<Vec<_>>()
                    .pack(),
            )
            .build()
    }
}

impl From<core::TransactionView> for TransactionView {
    fn from(value: core::TransactionView) -> Self {
        let raw = value.data().raw();

        Self {
            version:      raw.version().into(),
            cell_deps:    raw.cell_deps().into_iter().map(Into::into).collect(),
            header_deps:  raw.header_deps().into_iter().map(Into::into).collect(),
            inputs:       raw.inputs().into_iter().map(Into::into).collect(),
            outputs:      raw.outputs().into_iter().map(Into::into).collect(),
            outputs_data: raw
                .outputs_data()
                .into_iter()
                .map(|data| GraphqlBytes(data.unpack()))
                .collect(),
            witnesses:    value
                .witnesses()
                .into_iter()
                .map(|witness| GraphqlBytes(witness.unpack()))
                .collect(),
            hash:         value.hash().into(),
        }
    }
}
