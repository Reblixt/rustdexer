use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Error;
use sui_sdk::{
    rpc_types::{
        ObjectChange, SuiEvent, SuiObjectDataOptions, SuiObjectResponse,
        SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions,
    },
    types::{base_types::ObjectID, digests::TransactionDigest},
};

pub async fn find_potential_nfts(
    app_state: crate::AppState,
    digests: Vec<TransactionDigest>,
    option_transaction: SuiTransactionBlockResponseOptions,
    option_object: SuiObjectDataOptions,
    whitelisted_packages: &HashSet<String>,
    hokko_packages: &HashSet<String>,
) -> Result<Vec<SuiObjectResponse>, Error> {
    // let digests = checkpoint.transactions;
    let objects: Vec<SuiTransactionBlockResponse> = app_state
        .client
        .read_api()
        .multi_get_transactions_with_options(digests, option_transaction)
        .await?;

    let mut tasks = Vec::new();

    for block in &objects {
        if let Some(block_event) = &block.events {
            let vec_event = block_event.data.iter();
            let mut seen_ids = HashSet::new();
            let mut object_ids: Vec<ObjectID> = Vec::new();

            vec_event.for_each(|event| {
                if hokko_packages.contains(&event.package_id.to_string()) {
                    // TODO: Create a Hokko module to handle the logic of processing these events
                    tokio::task::spawn(async move { todo!() });
                };

                let nft_id = extract_id(event);
                if let Some(id) = nft_id {
                    if seen_ids.insert(id) {
                        object_ids.push(id);
                    }
                }
            });

            let id_len = object_ids.len();
            if id_len <= 50 && id_len != 0 {
                let client = app_state.client.clone();
                let options_object = option_object.clone();
                let task = tokio::task::spawn(async move {
                    client
                        .read_api()
                        .multi_get_object_with_options(object_ids, options_object)
                        .await
                });
                tasks.push(task);
            } else if id_len > 50 {
                todo!();
            }
        }

        if let Some(objects) = &block.object_changes {
            // objects[0].to_owned
            // println!("Changed objects: {objects:#?}");

            objects.iter().for_each(|obj| {
                // let object_id = obj.object_ref().0;
                //TODO: if object_id is in the blacklist skip this object
                match obj {
                    ObjectChange::Mutated {
                        // sender,
                        // owner,
                        // object_id,
                        object_type,
                        // digest,
                        ..
                    } => {
                        let whole_type = format!(
                            "{}::{}::{}",
                            object_type.address, object_type.module, object_type.name
                        );
                        if whitelisted_packages.contains(&whole_type) {
                            tokio::task::spawn(async move {
                                //TODO: Add logic when object has been mutaded
                                todo!()
                            });
                        }
                    }
                    ObjectChange::Transferred {
                        // sender,
                        // recipient,
                        // object_id,
                        object_type,
                        // digest,
                        ..
                    } => {
                        let whole_type = format!(
                            "{}::{}::{}",
                            object_type.address, object_type.module, object_type.name
                        );
                        if whitelisted_packages.contains(&whole_type) {
                            tokio::task::spawn(async move {
                                //TODO: Add logic when object has been transfered
                                todo!()
                            });
                        }
                    }
                    _ => {}
                }
            });
        }
    }

    let mut potential_nfts = Vec::new();
    for task in tasks {
        let result = task.await?;
        if let Ok(objects) = result {
            potential_nfts.extend(objects);
        }
    }

    Ok(potential_nfts)
}

// fn partition_transactions_by_limit_of(
//     transactions: &Vec<TransactionDigest>,
//     limit: usize,
// ) -> Vec<Vec<TransactionDigest>> {
//     transactions
//         .chunks(limit)
//         .map(|chunk| chunk.to_vec())
//         .collect()
// }

fn extract_id(event: &SuiEvent) -> Option<ObjectID> {
    let name = event.type_.clone().to_string().to_lowercase();
    if name.contains("token") | name.contains("coin") {
        return None;
    }
    if name.contains("mint") | name.contains("nft") {
        let parsed_json = event.parsed_json.clone();

        let possible_fields = ["id", "object_id", "objectId", "nft_id", "nft", "tokenId"];
        for field in &possible_fields {
            if let Some(value) = parsed_json.get(field) {
                if let Some(value_str) = value.as_str() {
                    let object_id = ObjectID::from_str(&value_str);
                    match object_id {
                        Ok(id) => return Some(id),
                        Err(e) => {
                            eprintln!("Could not parse to ObjectID: {e:#?}");
                            return None;
                        }
                    }
                }
            }
        }
        return None;
    }
    None
}
