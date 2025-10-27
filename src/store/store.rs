use crate::{
    AppState,
    entity::{collection, nft},
    processor::model::{NftData, OwnerType},
};
use chrono::Utc;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::OnConflict;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{EntityTrait, sea_query::Func};
use sea_orm::{NotSet, Set};

pub async fn store_nft(app_state: AppState, nft: NftData) {
    let db = app_state.db;

    let collection_entity = collection::ActiveModel {
        r#type: Set(nft.collection_type.clone()),
        verified: Set(false),
        banner_url: Set(nft.banner_url),
        image_url: Set(nft.cover_url.unwrap_or_else(|| "".to_string())),
        name: Set(nft.collection_name.unwrap_or_else(|| {
            nft.collection_type
                .clone()
                .split("::")
                .last()
                .unwrap_or(&nft.collection_type)
                .to_string()
        })),
        volume: Set(0),
        description: Set(nft.collection_description.unwrap_or_else(|| "".to_string())),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let _res = collection::Entity::insert(collection_entity)
        .on_conflict(
            OnConflict::column(collection::Column::Type)
                .do_nothing()
                .to_owned(),
        )
        .exec(db.as_ref())
        .await;

    let item_holder = nft.item_holder.unwrap_or_else(|| OwnerType {
        address_owner: Some("0x000".to_string()),
        object_owner: Some("0x000".to_string()),
    });

    let nft_entity = nft::ActiveModel {
        token_id: Set(nft.token_id.clone()),
        r#type: Set(nft.collection_type),
        name: Set(nft.nft_name.unwrap_or_else(|| nft.token_id)),
        image_url: Set(nft.image_url.unwrap_or_else(|| "".to_string())),
        description: Set(nft.description.unwrap_or_else(|| "".to_string())),
        rarity: Set(nft.rarity.unwrap_or_else(|| "".to_string())),
        kiosk: Set(item_holder.object_owner.unwrap_or_else(|| "".to_string())),
        holder: Set(item_holder
            .address_owner
            .clone()
            .unwrap_or_else(|| "".to_string())),
        digests: Set(Some(vec![
            nft.digest.clone().unwrap_or_else(|| "".to_string()),
        ])),
        volume: Set(0),
        wallet_address: Set(Some(
            item_holder.address_owner.unwrap_or_else(|| "".to_string()),
        )),
        rarity_score: nft
            .rarity_score
            .map_or_else(|| NotSet, |score| Set(Some(score as i32))),
        last_sale: NotSet,
        small_image_url: NotSet,
        metadata: NotSet,
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let _res = nft::Entity::insert(nft_entity)
        .on_conflict(
            OnConflict::column(nft::Column::TokenId)
                .update_columns([
                    nft::Column::Type,
                    nft::Column::Name,
                    nft::Column::ImageUrl,
                    nft::Column::Description,
                    nft::Column::Rarity,
                    nft::Column::Kiosk,
                    nft::Column::Holder,
                    nft::Column::WalletAddress,
                ])
                .value(
                    nft::Column::Digests,
                    Expr::col((nft::Entity, nft::Column::Digests))
                        .concat(Expr::val(vec![nft.digest.unwrap_or_default()])),
                )
                .to_owned(),
        )
        .exec(db.as_ref())
        .await;
}
