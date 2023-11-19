use crate::pb::eth::transaction::v1::{Transaction, Transactions};
use crate::abi;
use substreams::{log, Hex};
use substreams_ethereum::block_view::{CallView, LogView};
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace, CallType};

// #[derive(Deserialize)]
struct TransactionFilters {
    filters: Vec<TransactionFilter>
}
struct TransactionFilter {
    to: Vec<String>,
    account_abstraction_type: String
}

#[substreams::handlers::map]
fn map_filter_transactions(blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    // let filters = parse_filters_from_params(params)?;
    let filters = compose_filters();
    let header = blk.header.unwrap();

    let transactions: Vec<Transaction> = blk
        .transaction_traces.iter()
        .filter_map(|trans| {
            let aa_trans_type = apply_filter(&trans, &filters);
            if aa_trans_type.is_some() {
                Some(Transaction {
                    from: Hex::encode(&trans.from),
                    to: Hex::encode(&trans.to),
                    hash: Hex::encode(&trans.hash),
                    chain: "ethereum".to_owned(),
                    account_abstraction_type: aa_trans_type.unwrap(),
                    status: trans.status().as_str_name().to_owned(),
                    timestamp: Some(header.timestamp.as_ref().unwrap().clone())
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Transactions { transactions })
}

fn compose_filters() -> TransactionFilters {
    let erc4337_filter = TransactionFilter {
        to: vec!["0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789".to_string()],
        account_abstraction_type: "erc4337".to_string(),
    };

    let safe_filter = TransactionFilter {
        to: vec![
            "0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A".to_string(),   // v1.0.0
            "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F".to_string(),   // v1.1.1
            "0x6851D6fDFAfD08c0295C392436245E5bc78B0185".to_string(),   // v1.2.0
            "0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552".to_string(),   // v1.3.0
            "0x41675C099F32341bf84BFc5382aF534df5C7461a".to_string(),   // v1.4.1
        ],
        account_abstraction_type: "safe".to_string(),
    };

    let filters = TransactionFilters {
        filters: vec![erc4337_filter, safe_filter]
    };
    
    return filters;
}

fn apply_filter(transaction: &TransactionTrace, filters: &TransactionFilters) -> Option<String> {
    let hex_transaction_to = format!("0x{}", Hex::encode(&transaction.to));
    let mut pass = false;
    let mut account_abstraction_type: Option<String> = None;

    for filter in &filters.filters {
        if filter.to.iter().any(|address| address.to_lowercase() == hex_transaction_to) {
            account_abstraction_type = Some(filter.account_abstraction_type.to_owned());

            if account_abstraction_type.as_ref().unwrap().eq("erc4337") {
                pass = transaction.calls().any(|call| call_signature_filter(&call));
            } else if account_abstraction_type.as_ref().unwrap().eq("safe") {
                // shouldn't actually happen
                pass = transaction.receipt().logs().any(|log| event_data_filter(&log, &hex_transaction_to));
            }
        }

        if transaction.calls().any(|call| call_to_implementation_filter(&call, &filter.to)) {
            account_abstraction_type = Some(filter.account_abstraction_type.to_owned());

            if account_abstraction_type.as_ref().unwrap().eq("erc4337") {
                // shouldn't actually happen
                pass = transaction.calls().any(|call| call_signature_filter(&call))
            } else if account_abstraction_type.as_ref().unwrap().eq("safe") {
                pass =  transaction.receipt().logs().any(|log| event_data_filter(&log, &hex_transaction_to) )
            }
        }
    }
    if pass {
        return account_abstraction_type;
    } else {
        return None;
    }
}

fn call_signature_filter(call: &CallView) -> bool {
    match abi::entrypoint::functions::HandleOps::decode(&call.call) {
        Ok(decoded) => {
            log::info!("handleOps found, with beneficiary address: {}", Hex ::encode(decoded.beneficiary));
            return true;
        }
        Err(_e) => {
            return false;
        }
    }
}

fn call_to_implementation_filter(call: &CallView, implementation_addresses: &Vec<String>) -> bool {
    if call.call.call_type == CallType::Delegate as i32 {
        let hex_call_target_address = format!("0x{}", Hex::encode(&call.call.address));
        return implementation_addresses.iter().any(|address| address.to_lowercase() == hex_call_target_address);
    }
    false
}

fn event_data_filter(log: &LogView, trx_to_address: &str) -> bool {
    let hex_log_emmiter = format!("0x{}", Hex::encode(log.address()));
    if trx_to_address.eq(&hex_log_emmiter) {
        if let Ok(decoded) = abi::safe_v1_0_0::events::ExecutionFailed::decode(&log.log) {
            log::info!("ExecutionFailed (v1.0.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Ok(decoded) = abi::safe_v1_1_1::events::ExecutionFailure::decode(&log.log) {
            log::info!("ExecutionFailed (v1.1.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Ok(decoded) = abi::safe_v1_1_1::events::ExecutionSuccess::decode(&log.log) {
            log::info!("ExecutionSuccess (v1.1.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Ok(decoded) = abi::safe_v1_2_0::events::ExecutionFailure::decode(&log.log) {
            log::info!("ExecutionFailed (v1.2.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Ok(decoded) = abi::safe_v1_2_0::events::ExecutionSuccess::decode(&log.log) {
            log::info!("ExecutionSuccess (v1.2.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Ok(decoded) = abi::safe_v1_3_0::events::ExecutionFailure::decode(&log.log) {
            log::info!("ExecutionFailed (v1.3.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Ok(decoded) = abi::safe_v1_2_0::events::ExecutionSuccess::decode(&log.log) {
            log::info!("ExecutionSuccess (v1.3.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Ok(decoded) = abi::safe_v1_4_1::events::ExecutionFailure::decode(&log.log) {
            log::info!("ExecutionFailed (v1.4.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Ok(decoded) = abi::safe_v1_4_1::events::ExecutionSuccess::decode(&log.log) {
            log::info!("ExecutionSuccess (v1.4.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
    }
    false
}