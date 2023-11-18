use crate::pb::eth::transaction::v1::{Transactions};

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

#[substreams::handlers::map]
fn db_out(
    trxs: Transactions,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    for trx in trxs.transactions {
        push_create(&mut database_changes, &trx.hash, 0, &trx.chain, &trx.account_abstraction_type);
    }

    Ok(database_changes)
}


fn push_create(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    chain: &str,
    aa_type: &str,
) {
    changes
        .push_change("transactions", key, ordinal, Operation::Create)
        .change("chain", (None, chain))
        .change("aaType", (None, aa_type));
}