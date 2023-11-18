use crate::pb::eth::transaction::v1::{Transactions};

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

#[substreams::handlers::map]
fn db_out(
    trxs: Transactions,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    for trx in trxs.transactions {
        push_create(&mut database_changes, &trx.hash, 0);
    }

    Ok(database_changes)
}


fn push_create(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64
) {
    changes
        .push_change("transactions", key, ordinal, Operation::Create);
}