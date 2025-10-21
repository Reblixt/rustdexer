use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Wallet::Table)
                    .if_not_exists()
                    .col(string(Wallet::address).not_null().primary_key())
                    .col(integer(Wallet::personal_fee).null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Collection::Table)
                    .if_not_exists()
                    .col(string(Collection::r#type).not_null().primary_key())
                    .col(string(Collection::name).not_null())
                    .col(string(Collection::description).not_null())
                    .col(string(Collection::image_url).not_null())
                    .col(string(Collection::banner_url).null())
                    .col(boolean(Collection::verified).not_null().default(false))
                    .col(big_integer(Collection::volume).not_null().default(0))
                    .col(date_time(Collection::created_at).not_null())
                    .col(date_time(Collection::updated_at).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Nft::Table)
                    .if_not_exists()
                    .col(string(Nft::token_id).not_null().primary_key())
                    .col(string(Nft::r#type).not_null())
                    .col(string(Nft::name).not_null())
                    .col(string(Nft::image_url).not_null())
                    .col(json_binary(Nft::metadata).not_null())
                    .col(string(Nft::rarity).not_null())
                    .col(string(Nft::description).not_null())
                    .col(string(Nft::kiosk).not_null())
                    .col(string(Nft::holder).not_null())
                    .col(date_time(Nft::created_at).not_null())
                    .col(date_time(Nft::updated_at).not_null())
                    .col(big_integer(Nft::volume).not_null().default(0))
                    .col(string(Nft::wallet_address).null())
                    .col(integer(Nft::rarity_score).null())
                    .col(big_integer(Nft::last_sale).null())
                    .col(string(Nft::small_image_url).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nft_collection")
                            .from(Nft::Table, Nft::r#type)
                            .to(Collection::Table, Collection::r#type)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nft_wallet")
                            .from(Nft::Table, Nft::wallet_address)
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CollectionOffer::Table)
                    .if_not_exists()
                    .col(string(CollectionOffer::collection_type).not_null())
                    .col(string(CollectionOffer::kiosk).not_null())
                    .col(string(CollectionOffer::offer_id).not_null().primary_key())
                    .col(string(CollectionOffer::offer_cap).not_null())
                    .col(string(CollectionOffer::policy_id).not_null())
                    .col(big_integer(CollectionOffer::price).not_null())
                    .col(integer(CollectionOffer::marketplace_fee).not_null())
                    .col(integer(CollectionOffer::royalty_fee).not_null())
                    .col(string(CollectionOffer::owner_wallet_address).not_null())
                    .col(boolean(CollectionOffer::kiosk_standard).not_null())
                    .col(date_time(CollectionOffer::created_at).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_collection_offer_collection")
                            .from(CollectionOffer::Table, CollectionOffer::collection_type)
                            .to(Collection::Table, Collection::r#type)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_collection_offer_wallet")
                            .from(
                                CollectionOffer::Table,
                                CollectionOffer::owner_wallet_address,
                            )
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Listing::Table)
                    .if_not_exists()
                    .col(string(Listing::collection_type).not_null())
                    .col(string(Listing::kiosk).not_null())
                    .col(string(Listing::kiosk_owner_cap).not_null())
                    .col(
                        string(Listing::shared_purchase_cap)
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Listing::token_id).not_null())
                    .col(big_integer(Listing::price).not_null())
                    .col(integer(Listing::marketplace_fee).not_null())
                    .col(integer(Listing::royalty_fee).not_null())
                    .col(string(Listing::owner_wallet_address).not_null())
                    .col(boolean(Listing::kiosk_standard).not_null())
                    .col(date_time(Listing::created_at).not_null())
                    .col(date_time(Listing::updated_at).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_listing_nft")
                            .from(Listing::Table, Listing::token_id)
                            .to(Nft::Table, Nft::token_id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_listing_wallet")
                            .from(Listing::Table, Listing::owner_wallet_address)
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Offer::Table)
                    .if_not_exists()
                    .col(string(Offer::collection_type).not_null())
                    .col(string(Offer::kiosk).not_null())
                    .col(string(Offer::offer_id).not_null().primary_key())
                    .col(string(Offer::offer_cap).not_null())
                    .col(string(Offer::token_id).not_null())
                    .col(big_integer(Offer::price).not_null())
                    .col(integer(Offer::marketplace_fee).not_null())
                    .col(integer(Offer::royalty_fee).not_null())
                    .col(string(Offer::owner_wallet_address).not_null())
                    .col(boolean(Offer::kiosk_standard).not_null())
                    .col(date_time(Offer::created_at).not_null())
                    .col(date_time(Offer::updated_at).not_null())
                    .col(string(Offer::nft_id).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_offer_nft")
                            .from(Offer::Table, Offer::token_id)
                            .to(Nft::Table, Nft::token_id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_offer_wallet")
                            .from(Offer::Table, Offer::owner_wallet_address)
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Kiosk::Table)
                    .if_not_exists()
                    .col(string(Kiosk::kiosk_owner_cap).not_null().primary_key())
                    .col(string(Kiosk::kiosk).not_null())
                    .col(boolean(Kiosk::personal).not_null())
                    .col(string(Kiosk::owner_wallet_address).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_kiosk_wallet")
                            .from(Kiosk::Table, Kiosk::owner_wallet_address)
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LaunchpadCollection::Table)
                    .if_not_exists()
                    .col(
                        string(LaunchpadCollection::collectionType)
                            .not_null()
                            .primary_key(),
                    )
                    .col(boolean(LaunchpadCollection::kiosk_standard).not_null())
                    .col(string(LaunchpadCollection::name).not_null())
                    .col(string(LaunchpadCollection::description).not_null())
                    .col(string(LaunchpadCollection::supply).not_null())
                    .col(big_integer(LaunchpadCollection::price).not_null())
                    .col(string(LaunchpadCollection::max_item_per_address).not_null())
                    .col(boolean(LaunchpadCollection::white_list_enabled).not_null())
                    .col(big_integer(LaunchpadCollection::white_list_price).not_null())
                    .col(string(LaunchpadCollection::white_list_supply).not_null())
                    .col(boolean(LaunchpadCollection::custom_enabled).not_null())
                    .col(string(LaunchpadCollection::custom_name).not_null())
                    .col(big_integer(LaunchpadCollection::custom_price).not_null())
                    .col(string(LaunchpadCollection::custom_supply).not_null())
                    .col(
                        integer(LaunchpadCollection::mint_count)
                            .not_null()
                            .default(0),
                    )
                    .col(date_time(LaunchpadCollection::created_at).not_null())
                    .col(date_time(LaunchpadCollection::updated_at).not_null())
                    .col(string(LaunchpadCollection::creator_cap).not_null())
                    .col(
                        string(LaunchpadCollection::launch_id)
                            .not_null()
                            .unique_key(),
                    )
                    .col(boolean(LaunchpadCollection::native).not_null())
                    .col(string(LaunchpadCollection::state).not_null())
                    .col(string(LaunchpadCollection::cover_image).null())
                    .col(date_time(LaunchpadCollection::start_time).not_null())
                    .col(date_time(LaunchpadCollection::white_list_startTime).not_null())
                    .col(date_time(LaunchpadCollection::custom_start_time).not_null())
                    .col(string(LaunchpadCollection::owner_wallet_address).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_launchpad_collection_wallet")
                            .from(
                                LaunchpadCollection::Table,
                                LaunchpadCollection::owner_wallet_address,
                            )
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WhiteListAddress::Table)
                    .if_not_exists()
                    .col(string(WhiteListAddress::address).not_null().primary_key())
                    .col(string(WhiteListAddress::allocation).not_null())
                    .col(string(WhiteListAddress::launch_id).not_null())
                    .col(string(WhiteListAddress::name).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_whitelist_address_launchpad_collection")
                            .from(WhiteListAddress::Table, WhiteListAddress::launch_id)
                            .to(
                                LaunchpadCollection::Table,
                                LaunchpadCollection::collectionType,
                            )
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_whitelist_address_wallet")
                            .from(WhiteListAddress::Table, WhiteListAddress::address)
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(StaticNftAttribute::Table)
                    .if_not_exists()
                    .col(
                        string(StaticNftAttribute::nft_token_id)
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(StaticNftAttribute::collection_type).not_null())
                    .col(string(StaticNftAttribute::trait_type).not_null())
                    .col(string(StaticNftAttribute::value).not_null())
                    .col(string(StaticNftAttribute::rarity).not_null())
                    .col(date_time(StaticNftAttribute::created_at).not_null())
                    .col(date_time(StaticNftAttribute::updated_at).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_static_nft_attribute_nft")
                            .from(StaticNftAttribute::Table, StaticNftAttribute::nft_token_id)
                            .to(Nft::Table, Nft::token_id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DynamicNftAttribute::Table)
                    .if_not_exists()
                    .col(
                        string(DynamicNftAttribute::token_id)
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(DynamicNftAttribute::nft_token_id).not_null())
                    .col(string(DynamicNftAttribute::collection_type).not_null())
                    .col(string(DynamicNftAttribute::trait_type).not_null())
                    .col(string(DynamicNftAttribute::value).not_null())
                    .col(boolean(DynamicNftAttribute::equipped).not_null())
                    .col(date_time(DynamicNftAttribute::equipped_at).null())
                    .col(string(DynamicNftAttribute::name).not_null())
                    .col(string(DynamicNftAttribute::image_url).not_null())
                    .col(string(DynamicNftAttribute::rarity).null())
                    .col(json_binary(DynamicNftAttribute::metadata).null())
                    .col(date_time(DynamicNftAttribute::created_at).not_null())
                    .col(date_time(DynamicNftAttribute::updated_at).not_null())
                    .col(string(DynamicNftAttribute::attribute_type).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dynamic_nft_attribute_nft")
                            .from(
                                DynamicNftAttribute::Table,
                                DynamicNftAttribute::nft_token_id,
                            )
                            .to(Nft::Table, Nft::token_id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(NftEvent::Table)
                    .if_not_exists()
                    .col(string(NftEvent::collection_type).not_null())
                    .col(string(NftEvent::token_id).not_null())
                    .col(string(NftEvent::digest).not_null().primary_key())
                    .col(big_integer(NftEvent::price).not_null())
                    .col(integer(NftEvent::marketplace_fee).not_null())
                    .col(integer(NftEvent::royalty_fee).not_null())
                    .col(string(NftEvent::description).not_null())
                    .col(string(NftEvent::sender).not_null())
                    .col(string(NftEvent::reciever).not_null())
                    .col(string(NftEvent::r#type).not_null())
                    .col(date_time(NftEvent::created_at).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nft_event_wallet_sender")
                            .from(NftEvent::Table, NftEvent::sender)
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nft_event_wallet_reciever")
                            .from(NftEvent::Table, NftEvent::reciever)
                            .to(Wallet::Table, Wallet::address)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let _ = manager
            .drop_table(Table::drop().table(Wallet::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(Collection::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(Nft::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(CollectionOffer::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(Listing::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(Offer::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(Kiosk::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(LaunchpadCollection::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(WhiteListAddress::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(StaticNftAttribute::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(DynamicNftAttribute::Table).to_owned())
            .await;
        manager
            .drop_table(Table::drop().table(NftEvent::Table).to_owned())
            .await
    }
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum Collection {
    Table,
    r#type,
    name,
    description,
    image_url,
    banner_url,
    verified,
    volume,
    created_at,
    updated_at,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum Nft {
    Table,
    token_id,
    r#type,
    name,
    image_url,
    metadata,
    rarity,
    description,
    kiosk,
    holder,
    created_at,
    updated_at,
    volume,
    wallet_address,
    rarity_score,
    last_sale,
    small_image_url,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum Wallet {
    Table,
    address,
    personal_fee,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum CollectionOffer {
    Table,
    collection_type,
    kiosk,
    offer_id,
    offer_cap,
    policy_id,
    price,
    marketplace_fee,
    royalty_fee,
    owner_wallet_address,
    kiosk_standard,
    created_at,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum Listing {
    Table,
    collection_type,
    kiosk,
    kiosk_owner_cap,
    shared_purchase_cap,
    token_id,
    price,
    marketplace_fee,
    royalty_fee,
    owner_wallet_address,
    kiosk_standard,
    created_at,
    updated_at,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum Offer {
    Table,
    collection_type,
    kiosk,
    offer_id,
    offer_cap,
    token_id,
    price,
    marketplace_fee,
    royalty_fee,
    owner_wallet_address,
    kiosk_standard,
    created_at,
    updated_at,
    nft_id,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum Kiosk {
    Table,
    kiosk_owner_cap,
    kiosk,
    personal,
    owner_wallet_address,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum LaunchpadCollection {
    Table,
    collectionType,
    kiosk_standard,
    name,
    description,
    supply,
    price,
    max_item_per_address,
    white_list_enabled,
    white_list_price,
    white_list_supply,
    custom_enabled,
    custom_name,
    custom_price,
    custom_supply,
    mint_count,
    created_at,
    updated_at,
    creator_cap,
    launch_id,
    native,
    state,
    cover_image,
    start_time,
    white_list_startTime,
    custom_start_time,
    owner_wallet_address,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum WhiteListAddress {
    Table,
    address,
    allocation,
    launch_id,
    name,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum StaticNftAttribute {
    Table,
    nft_token_id,
    collection_type,
    trait_type,
    value,
    rarity,
    created_at,
    updated_at,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum DynamicNftAttribute {
    Table,
    token_id,
    nft_token_id,
    collection_type,
    trait_type,
    value,
    equipped,
    equipped_at,
    name,
    image_url,
    rarity,
    metadata,
    created_at,
    updated_at,
    attribute_type,
}

#[allow(non_camel_case_types)]
#[derive(DeriveIden)]
enum NftEvent {
    Table,
    collection_type,
    token_id,
    digest,
    price,
    marketplace_fee,
    royalty_fee,
    description,
    sender,
    reciever,
    r#type,
    created_at,
}
