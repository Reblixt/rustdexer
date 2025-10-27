use sea_orm::DatabaseConnection;
use sui_sdk::{
    SuiClient, SuiClientBuilder,
    rpc_types::{Checkpoint, SuiObjectDataOptions, SuiTransactionBlockResponseOptions},
    types::{digests::TransactionDigest, sui_serde::BigInt},
};

use std::{collections::HashSet, sync::Arc};

use crate::constants::contract::{HokkoPackageId, WHITE_LISTED_PACKAGES};

const LIMIT: usize = 50;
// const MAX_QUERY_LIMIT: usize = 50;
// const API_URL: &str =
//     "https://hokko-sui-mainnet.n.dwellir.com/c0c81069-123b-42e2-83d9-75ee1137e544";
const API_URL: &str = "https://weathered-soft-frost.sui-mainnet.quiknode.pro/c39d95c17cf68776ca7e369b9c4539681d7729c8/";

mod constants;
mod entity;
mod processor;
mod store;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub client: Arc<SuiClient>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Database connection
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file or environment variable");

    let db = Arc::new(sea_orm::Database::connect(&database_url).await?);

    // 193,710,202
    // let mut initial_cursor: BigInt<u64> = BigInt::from(193135143);
    let mut initial_cursor: BigInt<u64> = BigInt::from(193710192);
    let mut whitelisted_packages: HashSet<String> = HashSet::new();
    let hokko_packages: HashSet<String> = HokkoPackageId::hokko_hash_set();

    for package in WHITE_LISTED_PACKAGES {
        println!("packge: {package:?}");
        whitelisted_packages.insert(package.to_string());
    }

    // let client = SuiClientBuilder::default().build_mainnet().await?;
    let client = Arc::new(SuiClientBuilder::default().build(API_URL).await?);

    let app_state = AppState {
        db: db.clone(),
        client: client.clone(),
    };

    let option_transaction = SuiTransactionBlockResponseOptions::new()
        .with_events()
        .with_effects()
        .with_object_changes();

    let option_object = SuiObjectDataOptions::new()
        .with_content()
        .with_type()
        .with_display()
        .with_owner();

    loop {
        let checkpoints = get_checkpoints(app_state.client.clone(), &mut initial_cursor).await?;
        println!("Processed Checkpoint {initial_cursor}");

        for checkpoint in checkpoints {
            let digests = checkpoint.transactions;

            if digests.len() < 50 {
                let potential_nfts = processor::procesor::find_potential_nfts(
                    app_state.clone(),
                    digests,
                    option_transaction.clone(),
                    option_object.clone(),
                    &whitelisted_packages,
                    &hokko_packages,
                )
                .await?;
                if potential_nfts.len() == 0 {
                    continue;
                }
                //
                // println!("Potential Nfts: {potential_nfts:#?}");
                let app_state = app_state.clone();

                tokio::task::spawn(async move {
                    processor::assemble::process_object(app_state, potential_nfts).await
                });
            } else {
                // let partitions = partition_transactions_by_limit_of(&digests, MAX_QUERY_LIMIT);
                // let tasks: Vec<_> = partitions
                //     .iter()
                //     .map(|vec_tx| {
                //         let options = options_transaction.clone();
                //         let client = client.clone();
                //         // let txs = vec_tx.clone();
                //
                //         tokio::task::spawn(async move {
                //             client
                //                 .read_api()
                //                 .multi_get_transactions_with_options(vec_tx.to_vec(), options)
                //                 .await
                //         })
                //     })
                //     .collect();
                //
                // let results = futures::future::try_join_all(tasks).await?;
                //
                // let objects: Vec<SuiTransactionBlockResponse> = results
                //     .into_iter()
                //     .filter_map(|result| result.ok())
                //     .flatten()
                //     .collect();
                // println!("Objects {objects:#?}");
            }

            // checkpoint.transactions.iter().for_each(|tx| {
            //     println!("transactions: {:?}", tx);
            // });
        }

        // tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

async fn get_checkpoints(
    client: Arc<SuiClient>,
    next_cursor: &mut BigInt<u64>,
) -> Result<Vec<Checkpoint>, anyhow::Error> {
    let response = client
        .read_api()
        .get_checkpoints(Some(*next_cursor), Some(LIMIT), false)
        .await?;

    *next_cursor = response.next_cursor.unwrap();

    Ok(response.data)
}

fn partition_transactions_by_limit_of(
    transactions: &Vec<TransactionDigest>,
    limit: usize,
) -> Vec<Vec<TransactionDigest>> {
    transactions
        .chunks(limit)
        .map(|chunk| chunk.to_vec())
        .collect()
}
