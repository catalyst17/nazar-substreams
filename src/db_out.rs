use crate::pb::eth::transaction::v1::{Transactions, Transaction};

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

#[substreams::handlers::map]
fn db_out(
    trxs: Transactions,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    for trx in trxs.transactions {
        push_create(&mut database_changes, 0, &trx);
    }

    Ok(database_changes)
}


fn push_create(
    changes: &mut DatabaseChanges,
    ordinal: u64,
    trx: &Transaction
) {
    changes
        .push_change("transactions", &trx.hash, ordinal, Operation::Create)
        .change("chain", (None, &trx.chain))
        .change("aaType", (None, &trx.account_abstraction_type))
        .change("status", (None, &trx.status))
        .change("timestamp", (None, &trx.timestamp.as_ref().unwrap().clone()));
}