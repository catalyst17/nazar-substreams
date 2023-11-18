use crate::pb::eth::transaction::v1::{Transaction, Transactions};
use crate::abi;
use crate::util;
use anyhow::anyhow;
use serde::Deserialize;
use substreams::{log, Hex};
use substreams_ethereum::block_view::CallView;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace};

#[derive(Deserialize)]
struct TransactionFilterParams {
    to: Option<String>,
    from: Option<String>,
}

#[substreams::handlers::map]
fn map_filter_transactions(params: String, blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let filters = parse_filters_from_params(params)?;
    let header = blk.header.unwrap();

    let transactions: Vec<Transaction> = blk
        .transaction_traces.iter()
        .filter(|trans| apply_filter(&trans, &filters))
        .map(|trans| Transaction {
            from: Hex::encode(&trans.from),
            to: Hex::encode(&trans.to),
            hash: Hex::encode(&trans.hash),
            chain: "ethereum".to_owned(),
            account_abstraction_type: "erc4337".to_owned(),
            status: trans.status().as_str_name().to_owned(),
            timestamp: Some(header.timestamp.as_ref().unwrap().clone())
        })
        .collect();

    Ok(Transactions { transactions })
}

fn parse_filters_from_params(params: String) -> Result<TransactionFilterParams, Vec<substreams::errors::Error>> {
    let parsed_result = serde_qs::from_str(&params);
    if parsed_result.is_err() {
        return Err(Vec::from([anyhow!("Unexpected error while parsing parameters")]));
    }

    let filters = parsed_result.unwrap();
    verify_filters(&filters)?;

    Ok(filters)
}

fn verify_filters(params: &TransactionFilterParams) -> Result<(), Vec<substreams::errors::Error>> {
    let mut errors: Vec<substreams::errors::Error> = Vec::new();

    if params.from.is_some() && !util::is_address_valid(&params.from.as_ref().unwrap()) {
        let from = params.from.as_ref().unwrap();

        if !util::is_address_valid(from) {
            errors.push(anyhow!("'from' address ({}) is not valid", from));
        }
    }

    if params.to.is_some() && !util::is_address_valid(&params.to.as_ref().unwrap()) {
        let to = params.to.as_ref().unwrap();

        if !util::is_address_valid(to) {
            errors.push(anyhow!("'to' address ({}) is not valid", to));
        }
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(())
}

fn apply_filter(transaction: &TransactionTrace, filters: &TransactionFilterParams) -> bool {
    if !filter_by_parameter(&filters.from, &transaction.from)
        || !filter_by_parameter(&filters.to, &transaction.to)
    {
        return false;
    }

    return transaction.calls().any(|call| call_signature_filter(&call))
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

fn filter_by_parameter(parameter: &Option<String>, transaction_field: &Vec<u8>) -> bool {
    if parameter.is_none() {
        return true;
    }

    let parameter_as_vec = &Hex::decode(parameter.as_ref().unwrap()).expect("already verified");
    if transaction_field == parameter_as_vec {
        return true;
    }

    false
}