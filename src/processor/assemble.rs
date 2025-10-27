use sui_sdk::rpc_types::SuiObjectResponse;
use sui_sdk::types::object::Owner;

use crate::AppState;
use crate::processor::model::{NftData, OwnerType};
use crate::store::store::store_nft;

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
            let owner = object.owner.clone();
            if let Some(mut nft) = NftData::from_object_data(&object) {
                let app_state = app_state.clone();
                let owner = tokio::runtime::Handle::current().block_on(extract_ownership(
                    &app_state,
                    owner.as_ref(),
                    &object.object_id.to_string(),
                ));

                nft.item_holder = Some(owner);
                println!("Assembled NFT: {nft:#?}");
                tokio::task::spawn(async move {
                    // Send nft for storing into the database
                    store_nft(app_state, nft).await;
                });
            }
        });
}

async fn extract_ownership(
    app_state: &crate::AppState,
    owner: Option<&Owner>,
    object_id: &str,
) -> OwnerType {
    // Handle null/None owners
    let Some(owner) = owner else {
        eprintln!("Invalid owner type for {}: None", object_id);
        return OwnerType {
            address_owner: Some("error".to_string()),
            object_owner: Some("error".to_string()),
        };
    };

    match owner {
        // Handle shared owners
        Owner::Shared { .. } => OwnerType {
            address_owner: Some("Shared".to_string()),
            object_owner: Some("Shared".to_string()),
        },
        // Handle immutable owners
        Owner::Immutable => OwnerType {
            address_owner: Some("Immutable".to_string()),
            object_owner: Some("Immutable".to_string()),
        },
        // Handle address owners
        Owner::AddressOwner(address) => OwnerType {
            address_owner: Some(address.to_string()),
            object_owner: None,
        },
        // Handle object owners (likely in kiosk)
        Owner::ObjectOwner(object_id) => rpc_owner(app_state.clone(), object_id.to_string()).await,
        // Handle consensus address owners
        Owner::ConsensusAddressOwner { owner, .. } => OwnerType {
            address_owner: Some(owner.to_string()),
            object_owner: None,
        },
    }
}

async fn rpc_owner(app_state: AppState, object_owner_id: String) -> OwnerType {
    match app_state
        .client
        .read_api()
        .get_object_with_options(
            object_owner_id.parse().expect("Valid ObjectID"),
            sui_sdk::rpc_types::SuiObjectDataOptions::default()
                .with_owner()
                .with_type(),
        )
        .await
    {
        Ok(object_response) => {
            if let Some(data) = object_response.data {
                if let Some(owner) = data.owner {
                    match owner {
                        // Handle shared owners
                        Owner::Shared { .. } => OwnerType {
                            address_owner: Some("Shared".to_string()),
                            object_owner: Some("Shared".to_string()),
                        },
                        // Handle immutable owners
                        Owner::Immutable => OwnerType {
                            address_owner: Some("Immutable".to_string()),
                            object_owner: Some("Immutable".to_string()),
                        },
                        // Handle address owners
                        Owner::AddressOwner(address) => OwnerType {
                            address_owner: Some(address.to_string()),
                            object_owner: None,
                        },
                        // Handle object owners (likely in kiosk)
                        Owner::ObjectOwner(object_id) => OwnerType {
                            address_owner: None,
                            object_owner: Some(object_id.to_string()),
                        },
                        // Handle consensus address owners
                        Owner::ConsensusAddressOwner { owner, .. } => OwnerType {
                            address_owner: Some(owner.to_string()),
                            object_owner: None,
                        },
                    }
                } else {
                    OwnerType {
                        address_owner: None,
                        object_owner: Some(object_owner_id),
                    }
                }
            } else {
                OwnerType {
                    address_owner: Some("".to_string()),
                    object_owner: Some(object_owner_id),
                }
            }
        }
        Err(error) => {
            eprintln!("Error fetching owner: {}", error);
            OwnerType {
                address_owner: Some("error".to_string()),
                object_owner: Some(object_owner_id),
            }
        }
    }
}
