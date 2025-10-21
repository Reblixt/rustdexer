use std::collections::HashSet;

pub struct HokkoPackageId;

impl HokkoPackageId {
    pub const MARKETPLACE_V2: &'static str =
        "0x84fe9cc034eb3d4bf4e50acd79cbe99974a21f6988a60e4ad1935d3964de4984";
    pub const MARKETPLACE_V1: &'static str =
        "0x66a422f94f320db5f541fa1b7fa0c097fabbd9024e9496b3b0f3e45d5558ec51";
    pub const LAUNCHPAD: &'static str =
        "0x071296655b5bf15a4115573ccc283e1b75974a338e639a125adffcaf7560b16f";
    pub const LAUNCHPAD_SDK: &'static str =
        "0x884d89784bcc3ae443eb402b2e8af8891cfd5a0340cd2e4aa1aba7e2c20a5057";
    pub const MARKETPLACE_V3: &'static str =
        "0x392ebf946f80f1a93ba1a171ff286a88eb76f7c65d02e3639121b11f7e65be3";
    pub fn hokko_hash_set() -> HashSet<String> {
        let mut set = HashSet::new();
        set.insert(Self::MARKETPLACE_V1.to_string());
        set.insert(Self::MARKETPLACE_V2.to_string());
        set.insert(Self::MARKETPLACE_V3.to_string());
        set.insert(Self::LAUNCHPAD.to_string());
        set.insert(Self::LAUNCHPAD_SDK.to_string());
        set
    }
}

pub struct HokkoEventTypes;
impl HokkoEventTypes {
    pub fn kiosk_created() -> String {
        format!(
            "{}::marketplace::KioskCreatedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn listing_created() -> String {
        format!("{}::trade::ItemListedEvent", HokkoPackageId::MARKETPLACE_V1)
    }

    pub fn listing_updated() -> String {
        format!(
            "{}::trade::ItemUpdatedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn delisted() -> String {
        format!(
            "{}::trade::ItemDelistedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn purchased() -> String {
        format!("{}::trade::ItemBoughtEvent", HokkoPackageId::MARKETPLACE_V1)
    }

    pub fn offer_created() -> String {
        format!("{}::escrow::OfferEvent", HokkoPackageId::MARKETPLACE_V1)
    }

    pub fn offer_accepted() -> String {
        format!(
            "{}::escrow::AcceptOfferEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn offer_declined() -> String {
        format!(
            "{}::escrow::DeclineOfferEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn offer_revoked() -> String {
        format!(
            "{}::escrow::RevokeOfferEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn collection_offer_created() -> String {
        format!(
            "{}::collection_escrow::NewOfferEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn collection_offer_accepted() -> String {
        format!(
            "{}::collection_escrow::OfferAcceptedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn collection_offer_revoked() -> String {
        format!(
            "{}::collection_escrow::OfferRevokedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn personal_fee_updated() -> String {
        format!(
            "{}::marketplace::PersonalFeeSetEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn recipient_created() -> String {
        format!(
            "{}::escrow::ReceiptCreatedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn recipient_destroyed() -> String {
        format!(
            "{}::escrow::ReceiptDestroyedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn recipient_created_collection() -> String {
        format!(
            "{}::collection_escrow::ReceiptCreatedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    pub fn recipient_destroyed_collection() -> String {
        format!(
            "{}::collection_escrow::ReceiptDestroyedEvent",
            HokkoPackageId::MARKETPLACE_V1
        )
    }

    // Launchpad events initiated by admins
    // pub fn new_register() -> String {
    //     format!("{}::launchpad::LaunchpadCollectionPendingEvent", HokkoPackageId::LAUNCHPAD)
    // }

    pub fn approve_collection() -> String {
        format!(
            "{}::launchpad::LaunchpadApprovedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    pub fn rejected_collection() -> String {
        format!(
            "{}::launchpad::LaunchpadRejectedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    pub fn paused_collection() -> String {
        format!(
            "{}::launchpad::LaunchpadPausedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    pub fn resumed_collection() -> String {
        format!(
            "{}::launchpad::LaunchpadResumedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    // Collection manager events
    pub fn creator_timestamp_updated() -> String {
        format!(
            "{}::launch_manager::LaunchTimestampsUpdatedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    pub fn creator_whitelist_updated() -> String {
        format!(
            "{}::launch_manager::LaunchWhitelistUpdatedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    pub fn creator_paused() -> String {
        format!(
            "{}::launch_manager::LaunchPausedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    pub fn creator_resumed() -> String {
        format!(
            "{}::launch_manager::LaunchResumedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    // These events insert new collections into the database
    pub fn creator_initialized() -> String {
        format!(
            "{}::launch_manager::LaunchInitializedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }

    pub fn item_minted() -> String {
        format!(
            "{}::launch_manager::ItemMintedEvent",
            HokkoPackageId::LAUNCHPAD
        )
    }
}

pub const WHITE_LISTED_PACKAGES: &[&str] = &[
    "0x57191e5e5c41166b90a4b7811ad3ec7963708aa537a8438c1761a5d33e2155fd::kumo::Kumo",
    "0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7::ika_chan_nft::IkaChanNft",
    "0x8f74a7d632191e29956df3843404f22d27bd84d92cca1b1abde621d033098769::rootlet::Rootlet",
    "0xc4f793bda2ce1db8a0626b5d3e189680bf7b17559bfe8389cd9db10d4e4d61dc::nft::KillaClubNFT",
    "0x034c162f6b594cb5a1805264dd01ca5d80ce3eca6522e6ee37fd9ebfb9d3ddca::factory::PrimeMachin",
    "0x75888defd3f392d276643932ae204cd85337a5b8f04335f9f912b6291149f423::nft::Tally",
    "0x9f48e186b1527bd164960a03f392c14669acfd1ef560fb6138ad0918e6e712a3::doonies::NFT",
    "0x862810efecf0296db2e9df3e075a7af8034ba374e73ff1098e88cc4bb7c15437::doubleup_citizens::DoubleUpCitizen",
    "0x00a1d5e3f98eb588b245a87c02363652436450aedb62ef1a7b018f16e6423059::delorean::DeloreanNFT",
    "0xb07b09b016d28f989b6adda8069096da0c0a0ff6490f6e0866858c023b061bee::mystic_yeti::MysticYeti",
    "0xd22b24490e0bae52676651b4f56660a5ff8022a2576e0089f79b3c88d44e08f0::suins_registration::SuinsRegistration",
    "0x141d8a2333f9369452fe075331924bb98d2abf0ee98de941db85aaf809c4ef54::aeon::Aeon",
    "0x75cab45b9cba2d0b06a91d1f5fa51a4569da07374cf42c1bd2802846a61efe33::cosmetic::Cosmetic",
    "0x835515170ee826c646fafd5c41602edf9474a42649983472119cd8e98c7318c3::vram::VramNFT",
    "0xee496a0cc04d06a345982ba6697c90c619020de9e274408c7819f787ff66e1a1::suifrens::SuiFren<0xee496a0cc04d06a345982ba6697c90c619020de9e274408c7819f787ff66e1a1::capy::Capy>",
    "0x4125c462e4dc35631e7b31dc0c443930bd96fbd24858d8e772ff5b225c55a792::avatars::Avatar",
    "0xbaac739939538e93167c6063b3f0b9318d52b66677070676815c8266d328a340::nft::Tako",
    "0x57191e5e5c41166b90a4b7811ad3ec7963708aa537a8438c1761a5d33e2155fd::attribute::KumoFurColour",
    "0x57191e5e5c41166b90a4b7811ad3ec7963708aa537a8438c1761a5d33e2155fd::attribute::KumoMouth",
    "0x57191e5e5c41166b90a4b7811ad3ec7963708aa537a8438c1761a5d33e2155fd::attribute::KumoBackground",
    "0x57191e5e5c41166b90a4b7811ad3ec7963708aa537a8438c1761a5d33e2155fd::attribute::KumoEyes",
    "0x57191e5e5c41166b90a4b7811ad3ec7963708aa537a8438c1761a5d33e2155fd::attribute::KumoTail",
    "0x57191e5e5c41166b90a4b7811ad3ec7963708aa537a8438c1761a5d33e2155fd::attribute::KumoAccessory",
    "0xd22b24490e0bae52676651b4f56660a5ff8022a2576e0089f79b3c88d44e08f0::suins_registration::SuinsRegistration",
];
