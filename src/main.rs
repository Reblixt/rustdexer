use sui_sdk::{
    SuiClient, SuiClientBuilder,
    rpc_types::{Checkpoint, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions},
    types::{digests::TransactionDigest, sui_serde::BigInt},
};

const LIMIT: usize = 10;
const MAX_QUERY_LIMIT: usize = 50;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 193,710,202
    // let mut initial_cursor: BigInt<u64> = BigInt::from(193135143);
    let mut initial_cursor: BigInt<u64> = BigInt::from(193710192);

    let client = SuiClientBuilder::default().build_mainnet().await?;

    loop {
        let checkpoints = get_checkpoints(client.clone(), &mut initial_cursor).await?;

        for checkpoint in checkpoints {
            let digests = checkpoint.transactions;

            // let pages = transacitons.len()
            let options = SuiTransactionBlockResponseOptions::new()
                .with_events()
                .with_effects()
                .with_object_changes();

            if digests.len() < 50 {
                let objects = client
                    .read_api()
                    .multi_get_transactions_with_options(digests, options)
                    .await?;

                // println!("Objects {objects:#?}");
                objects.iter().for_each(|block| {
                    if let Some(event) = &block.events {
                        let data = event.data.iter();

                        data.for_each(|e| {
                            let name = e.type_.clone().to_string().to_lowercase();
                            if name.contains("mint") | name.contains("nft") {
                                println!("Name of event: {name}");
                                let parsed_json = e.parsed_json.clone();
                                // let extract_id = parsed_json.get("nft_id");

                                println!("potential nft: {parsed_json:#?}");
                                // println!("Nft id : {extract_id:?}");
                            };
                        });
                        // let parsed_json = event.data[0].parsed_json.clone();
                        // println!("Events: {event:#?}")
                    }
                });
            } else {
                let partitions = partition_transactions_by_limit_of(&digests, MAX_QUERY_LIMIT);
                let tasks: Vec<_> = partitions
                    .iter()
                    .map(|vec_tx| {
                        let client = client.clone();
                        let options = options.clone();
                        let txs = vec_tx.clone();

                        tokio::task::spawn(async move {
                            client
                                .read_api()
                                .multi_get_transactions_with_options(txs, options)
                                .await
                        })
                    })
                    .collect();

                let results = futures::future::try_join_all(tasks).await?;

                let objects: Vec<SuiTransactionBlockResponse> = results
                    .into_iter()
                    .filter_map(|result| result.ok())
                    .flatten()
                    .collect();
                println!("Objects {objects:#?}");
            }

            // checkpoint.transactions.iter().for_each(|tx| {
            //     println!("transactions: {:?}", tx);
            // });
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
    }
}

async fn get_checkpoints(
    client: SuiClient,
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
