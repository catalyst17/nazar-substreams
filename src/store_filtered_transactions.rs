use crate::pb::eth::transaction::v1::{Transaction, Transactions};
use substreams::store::{
    StoreSetIfNotExists, StoreSetIfNotExistsProto, StoreNew
};

#[substreams::handlers::store]
fn store_filtered_transactions(trxs: Transactions, s: StoreSetIfNotExistsProto<Transaction>) {
    for trx in trxs.transactions {
        s.set_if_not_exists(0, &trx.hash, &trx);
    }
}