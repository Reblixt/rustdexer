use sui_sdk::rpc_types::SuiObjectResponse;

use crate::processor::model::NftData;

pub async fn process_object(app_state: crate::AppState, objects: Vec<SuiObjectResponse>) {
    objects
        .into_iter()
        .filter(|obj| {
            obj.data
                .as_ref()
                .map_or(false, |data| data.display.is_some())
        })
        .for_each(|obj| {
            let object = obj.data.unwrap(); // Safe to unwrap

            if let Some(nft) = NftData::from_object_data(object.object_id, &object) {
                // tokio::task::spawn(async move {
                //     // Send nft for storing into the database
                //     todo!()
                // });
                println!("Assembled NFT: {nft:#?}");
            }
        });
}
