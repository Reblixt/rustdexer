use std::collections::BTreeMap;
use sui_sdk::rpc_types::SuiObjectData;

use serde_json::Value;
use sui_sdk::rpc_types::SuiParsedData;

#[derive(Debug)]
pub struct NftDataParams {
    pub token_id: String,
    pub collection_type: String,
    pub digest: String,
}
#[derive(Debug, Clone)]
pub struct OwnerType {
    pub address_owner: Option<String>,
    pub object_owner: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AttributeAssembly {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct NftData {
    // Inherited from AssembleFromDisplayData
    pub collection_name: Option<String>,
    pub collection_description: Option<String>,
    pub image_url: Option<String>,
    pub cover_url: Option<String>,
    pub banner_url: Option<String>,
    pub nft_name: Option<String>,
    pub description: Option<String>,
    pub collection_id: Option<String>,
    pub creator: Option<String>,
    pub external_url: Option<String>,
    pub project_url: Option<String>,
    pub rarity: Option<String>,
    pub rarity_score: Option<u32>,

    // Additional fields from NftData
    pub digest: Option<String>,
    pub collection_type: String,
    pub token_id: String,
    pub attributes: Option<Vec<AttributeAssembly>>,
    pub metadata: Option<Vec<AttributeAssembly>>,
    pub item_holder: Option<OwnerType>,
}

impl NftData {
    pub fn new(input: NftDataParams) -> Self {
        NftData {
            nft_name: None,
            description: None,
            creator: None,
            collection_id: None,
            image_url: None,
            cover_url: None,
            banner_url: None,
            external_url: None,
            project_url: None,
            rarity: None,
            rarity_score: None,
            collection_name: None,
            collection_description: None,
            digest: Some(input.digest),
            collection_type: input.collection_type,
            token_id: input.token_id,
            attributes: None,
            metadata: None,
            item_holder: None,
        }
    }

    pub fn from_object_data(data: &SuiObjectData) -> Option<Self> {
        let type_string = data.type_.as_ref()?.to_string();
        let mut nft = Self::new(NftDataParams {
            token_id: data.object_id.to_string(),
            // token_id: object_id.to_string(),
            collection_type: type_string,
            digest: data.digest.to_string(),
        });
        // Extract from display data first
        if let Some(display) = &data.display {
            if let Some(display_data) = &display.data {
                nft.extract_from_display(display_data);
            }
        }

        // Fill missing fields from content
        if let Some(content) = &data.content {
            nft.extract_from_content(content);
        }

        Some(nft)
    }

    fn extract_from_display(&mut self, data: &BTreeMap<String, String>) {
        self.nft_name = Self::extract_nft_name(data);
        self.description = Self::extract_description(data);
        self.creator = Self::extract_creator(data);
        self.collection_id = Self::extract_collection_id(data);
        self.image_url = Self::extract_image_url(data);
        self.cover_url = Self::extract_cover_url(data);
        self.banner_url = Self::extract_banner_url(data);
        self.external_url = Self::extract_external_url(data);
        self.project_url = Self::extract_project_url(data);
        self.rarity = Self::extract_rarity(data);
        self.rarity_score = Self::extract_rarity_score(data);
        self.collection_name = Self::extract_collection_name(data);
        self.collection_description = Self::extract_collection_description(data);
    }

    fn extract_from_content(&mut self, content: &SuiParsedData) {
        match content {
            SuiParsedData::MoveObject(move_obj) => {
                // Convert SuiMoveStruct to serde_json::Value
                if let Ok(json_value) = serde_json::to_value(&move_obj.fields) {
                    self.extract_fields_from_move_object(&json_value);
                }
            }
            SuiParsedData::Package(_) => {
                // Handle package data if needed
            }
        }
    }

    fn extract_fields_from_move_object(&mut self, fields: &Value) {
        if let Value::Object(map) = fields {
            // Extract basic fields
            if self.nft_name.is_none() {
                if let Some(name) = map.get("name") {
                    if let Some(s) = name.as_str() {
                        self.nft_name = Some(s.to_string());
                    }
                }
            }

            if self.description.is_none() {
                if let Some(desc) = map.get("description") {
                    if let Some(s) = desc.as_str() {
                        self.description = Some(s.to_string());
                    }
                }
            }

            if self.image_url.is_none() {
                if let Some(image) = map.get("image_url") {
                    if let Some(s) = image.as_str() {
                        self.image_url = Some(s.to_string());
                    }
                }
            }

            if self.rarity.is_none() {
                if let Some(rarity) = map.get("rarity") {
                    if let Some(s) = rarity.as_str() {
                        self.rarity = Some(s.to_string());
                    }
                }
            }

            // Extract attributes
            if self.attributes.is_none() {
                if let Some(attributes) = map.get("attributes") {
                    // Try to get the fields from the attributes object first
                    if let Some(attrs_fields) = attributes.get("fields") {
                        if let Some(attrs) = Self::extract_attributes(attrs_fields) {
                            self.attributes = Some(attrs);
                            return; // Successfully extracted, no need to try other methods
                        }
                    }
                    // Fallback to direct extraction if fields not found
                    if let Some(attrs) = Self::extract_attributes(attributes) {
                        self.attributes = Some(attrs);
                    }
                }
            }
        }
    }

    fn extract_attributes(value: &Value) -> Option<Vec<AttributeAssembly>> {
        // Try each extraction method in order of likelihood
        Self::extract_attributes_array_format(value)
            .or_else(|| Self::extract_attributes_nested_format(value))
            .or_else(|| Self::extract_attributes_simple_object_format(value))
    }

    // Handle array format: [{"fields": {"key": "name", "value": "Kumo #104"}}, ...]
    fn extract_attributes_array_format(value: &Value) -> Option<Vec<AttributeAssembly>> {
        let Value::Array(vec) = value else {
            return None;
        };

        let mut attrs = Vec::new();
        for item in vec {
            let Value::Object(obj) = item else { continue };
            let Some(fields) = obj.get("fields") else {
                continue;
            };
            let Value::Object(field_map) = fields else {
                continue;
            };

            let key = field_map
                .get("key")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let value = field_map
                .get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if !key.is_empty() {
                attrs.push(AttributeAssembly { key, value });
            }
        }

        if !attrs.is_empty() { Some(attrs) } else { None }
    }

    // Handle nested attributes structure from complex example
    fn extract_attributes_nested_format(value: &Value) -> Option<Vec<AttributeAssembly>> {
        // println!("Extracting nested attributes from value: {:#?}", value);
        //
        let Value::Object(obj) = value else {
            return None;
        };

        let Some(fields) = obj.get("fields") else {
            return None;
        };

        let Value::Object(fields_obj) = fields else {
            return None;
        };
        // Handle the nested structure: look for pos0 first
        let attrs_obj = if let Some(pos0) = fields_obj.get("pos0") {
            // Extract from pos0.fields
            let Value::Object(pos0_obj) = pos0 else {
                return None;
            };
            let Some(pos0_fields) = pos0_obj.get("fields") else {
                return None;
            };
            let Value::Object(pos0_fields_obj) = pos0_fields else {
                return None;
            };
            pos0_fields_obj
        } else {
            // Fallback to direct fields if pos0 doesn't exist
            fields_obj
        };

        let mut attrs = Vec::new();

        // Extract from dynamic, static, and misc attribute collections
        for &attr_type in &["dynamic", "static", "misc"] {
            let Some(attr_collection) = attrs_obj.get(attr_type) else {
                continue;
            };

            let Value::Object(collection_obj) = attr_collection else {
                continue;
            };

            let Value::Object(collection_fields) = collection_obj.get("fields")? else {
                continue;
            };

            let Some(contents) = collection_fields.get("contents") else {
                continue;
            };

            let Value::Array(contents_array) = contents else {
                continue;
            };

            for entry in contents_array {
                let Value::Object(entry_obj) = entry else {
                    continue;
                };

                let Some(entry_fields) = entry_obj.get("fields") else {
                    continue;
                };

                let Value::Object(fields_map) = entry_fields else {
                    continue;
                };

                let key = fields_map
                    .get("key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let value = fields_map
                    .get("value")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                if !key.is_empty() {
                    attrs.push(AttributeAssembly { key, value });
                }
            }
        }

        if !attrs.is_empty() { Some(attrs) } else { None }
    }

    // Handle simple object format: {"name": "Kumo #104", "description": "...", ...}
    fn extract_attributes_simple_object_format(value: &Value) -> Option<Vec<AttributeAssembly>> {
        let Value::Object(obj) = value else {
            return None;
        };

        // If this object has a "fields" key, it's likely the nested format, not simple object format
        if obj.contains_key("fields") {
            return None;
        }

        let mut attrs = Vec::new();
        for (key, val) in obj {
            if let Some(value_str) = val.as_str() {
                attrs.push(AttributeAssembly {
                    key: key.clone(),
                    value: value_str.to_string(),
                });
            }
        }

        if !attrs.is_empty() { Some(attrs) } else { None }
    }

    // ... rest of the coalesce methods remain the same ...

    fn coalesce_string(data: &BTreeMap<String, String>, keys: &[&str]) -> Option<String> {
        keys.iter()
            .find_map(|&key| data.get(key).filter(|value| !value.is_empty()).cloned())
    }

    fn coalesce_u32(data: &BTreeMap<String, String>, keys: &[&str]) -> Option<u32> {
        keys.iter().find_map(|&key| {
            data.get(key)
                .filter(|value| !value.is_empty())
                .and_then(|v| v.parse::<u32>().ok())
        })
    }

    fn extract_nft_name(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &["name", "nftName", "nft_name", "nameNft", "name_nft"];
        Self::coalesce_string(data, keys)
    }

    fn extract_description(data: &BTreeMap<String, String>) -> Option<String> {
        data.get("description").filter(|v| !v.is_empty()).cloned()
    }

    fn extract_creator(data: &BTreeMap<String, String>) -> Option<String> {
        data.get("creator").filter(|v| !v.is_empty()).cloned()
    }

    fn extract_collection_id(data: &BTreeMap<String, String>) -> Option<String> {
        data.get("collection_id").filter(|v| !v.is_empty()).cloned()
    }

    fn extract_image_url(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &[
            "image_url",
            "imageUrl",
            "image_uri",
            "image",
            "image_hash",
            "imageHash",
        ];
        Self::coalesce_string(data, keys)
    }

    fn extract_cover_url(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &[
            "cover_url",
            "coverUrl",
            "cover_image",
            "coverImage",
            "cover",
        ];
        Self::coalesce_string(data, keys)
    }

    fn extract_banner_url(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &[
            "banner_url",
            "bannerUrl",
            "banner",
            "bannerImage",
            "banner_image",
        ];
        Self::coalesce_string(data, keys)
    }

    fn extract_external_url(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &["external_url", "externalUrl"];
        Self::coalesce_string(data, keys)
    }

    fn extract_project_url(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &["project_url", "projectUrl"];
        Self::coalesce_string(data, keys)
    }

    fn extract_rarity(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &["rarity", "rarityType", "rarity_type"];
        Self::coalesce_string(data, keys)
    }

    fn extract_rarity_score(data: &BTreeMap<String, String>) -> Option<u32> {
        let keys = &["rarity_score", "rarityScore"];
        Self::coalesce_u32(data, keys)
    }

    fn extract_collection_name(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &["collection_name", "collectionName"];
        Self::coalesce_string(data, keys)
    }

    fn extract_collection_description(data: &BTreeMap<String, String>) -> Option<String> {
        let keys = &["collection_description", "collectionDescription"];
        Self::coalesce_string(data, keys)
    }
}

#[cfg(test)]
mod tests {

    use move_core_types::{
        account_address::AccountAddress, identifier::Identifier, language_storage::StructTag,
    };
    use std::str::FromStr;

    use sui_sdk::{
        rpc_types::{DisplayFieldsResponse, SuiMoveStruct, SuiParsedMoveObject},
        types::{
            base_types::{ObjectID, ObjectType},
            digests::ObjectDigest,
        },
    };

    use super::*;

    #[test]
    fn test_extract_ica_nft() {
        let object_type = ObjectType::from_str("0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7::ika_chan_nft::IkaChanNft").unwrap();
        let struct_tag = StructTag {
            address: AccountAddress::from_str(
                "0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7",
            )
            .unwrap(),
            module: Identifier::new("ika_chan_nft").unwrap(),
            name: Identifier::new("IkaChanNft").unwrap(),
            type_params: vec![], // No type parameters in this case
        };
        let object_data = SuiObjectData {
            object_id: ObjectID::from_str(
                "0x9b58b1e31a8adfd6c1fc29111f6b308d0fe4a08bd1bac36234a601697d8a9e57",
            )
            .unwrap(),
            version: 664719341.into(),
            digest: ObjectDigest::from_str("9CmDkwxNmjiUM2ca61cQoKFrbbQ2YKyBwdMA82RMfQ12").unwrap(),
            type_: Some(object_type.clone()),
            owner: None,
            previous_transaction: None,
            storage_rebate: Some(100),
            content: Some(SuiParsedData::MoveObject(SuiParsedMoveObject {
                type_: struct_tag,
                has_public_transfer: true,
                fields: SuiMoveStruct::WithFields(
                    serde_json::from_value(serde_json::json!({
                        "attributes": {
                          "type": "0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7::ika_chan_attributes::Attributes",
                          "fields": {
                            "pos0": {
                              "type": "0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7::base_attributes::Attributes",
                              "fields": {
                                "dynamic": {
                                  "type": "0x2::vec_map::VecMap<0x1::string::String, 0x1::string::String>",
                                  "fields": {
                                    "contents": [
                                      {
                                        "type": "0x2::vec_map::Entry<0x1::string::String, 0x1::string::String>",
                                        "fields": {
                                          "key": "Background",
                                          "value": "None"
                                        }
                                      },
                                      {
                                        "type": "0x2::vec_map::Entry<0x1::string::String, 0x1::string::String>",
                                        "fields": {
                                          "key": "Texture overlay",
                                          "value": "Worn"
                                        }
                                      }
                                    ]
                                  }
                                },
                                "hash": "95b9da6f2bebb1e6c980b6f7ebd67e4fb3310b7ae95e972889cc40053a5f004b",
                                "misc": {
                                  "type": "0x2::vec_map::VecMap<0x1::string::String, 0x1::string::String>",
                                  "fields": {
                                    "contents": [
                                      {
                                        "type": "0x2::vec_map::Entry<0x1::string::String, 0x1::string::String>",
                                        "fields": {
                                          "key": "Edition",
                                          "value": "8862"
                                        }
                                      }
                                    ]
                                  }
                                },
                                "static": {
                                  "type": "0x2::vec_map::VecMap<0x1::string::String, 0x1::string::String>",
                                  "fields": {
                                    "contents": [
                                      {
                                        "type": "0x2::vec_map::Entry<0x1::string::String, 0x1::string::String>",
                                        "fields": {
                                          "key": "Rarity",
                                          "value": "Common"
                                        }
                                      },
                                      {
                                        "type": "0x2::vec_map::Entry<0x1::string::String, 0x1::string::String>",
                                        "fields": {
                                          "key": "Cover image",
                                          "value": "Multicolor"
                                        }
                                      },
                                      {
                                        "type": "0x2::vec_map::Entry<0x1::string::String, 0x1::string::String>",
                                        "fields": {
                                          "key": "Foil pattern",
                                          "value": "Gold"
                                        }
                                      }
                                    ]
                                  }
                                }
                              }
                            }
                          }
                        },
                        "id": {
                            "id": "0x9b58b1e31a8adfd6c1fc29111f6b308d0fe4a08bd1bac36234a601697d8a9e57"
                        },
                        "key_wallet": {
                            "type": "0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7::ika_chan_key_wallet::KeyWallet",
                            "fields": {
                                "balance": "0"
                            }
                        },
                        "level": {
                            "type": "0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7::ika_chan_level::Level",
                            "fields": {
                                "current": 10,
                                "evolution_stage": 2
                            }
                        },
                        "metadata": {
                            "type": "0xd2197b1ce2096e96e726c29fa2c138c5c6748da169b81d34927c522b7499f1d7::ika_chan_nft::Metadata",
                            "fields": {
                                "background": "None",
                                "cover_image": "Multicolor",
                                "edition": "8862",
                                "evolution_stage": 2,
                                "foil_pattern": "Gold",
                                "hash": "95b9da6f2bebb1e6c980b6f7ebd67e4fb3310b7ae95e972889cc40053a5f004b-revealed",
                                "key_balance": "0",
                                "level": 10,
                                "quality": "Worn",
                                "rarity": "Common"
                            }
                        }
                    })).unwrap()
                ),
            })),
            display: Some(
                DisplayFieldsResponse {
                    data: serde_json::from_value(serde_json::json!({
                        "background": "None",
                        "claimable_keys": "0",
                        "collection": "MF SQUID MARKET",
                        "cover_image": "Multicolor",
                        "creator": "dWallet Labs / Rhei / Anima Labs / Studio Mirai",
                        "description": "Ika-sama's forbidden tentacle harem. Slimy yet satisfying. Yamete kudasai~!",
                        "edition": "8862",
                        "evolution_stage": "2",
                        "foil_pattern": "Gold",
                        "hash": "95b9da6f2bebb1e6c980b6f7ebd67e4fb3310b7ae95e972889cc40053a5f004b-revealed",
                        "id": "0x9b58b1e31a8adfd6c1fc29111f6b308d0fe4a08bd1bac36234a601697d8a9e57",
                        "image_url": "https://images.mfsquid.market/ika-chan/95b9da6f2bebb1e6c980b6f7ebd67e4fb3310b7ae95e972889cc40053a5f004b-revealed.png",
                        "level": "10",
                        "name": "Ika-chan #8862",
                        "project_url": "https://mfsmnft.wal.app/",
                        "quality": "Worn",
                        "rarity": "Common"
                    })).unwrap(),
                    error: None

                }
            ),
            bcs: None,
        };

        let nft_data = NftData::from_object_data(&object_data).unwrap();
        let attributes = nft_data.attributes.unwrap();
        assert_eq!(attributes.len(), 6);
        assert_eq!(attributes[0].key, "Background");
        assert_eq!(attributes[0].value, "None");
        assert_eq!(attributes[1].key, "Texture overlay");
        assert_eq!(attributes[1].value, "Worn");
        assert_eq!(attributes[2].key, "Rarity");
        assert_eq!(attributes[2].value, "Common");
        assert_eq!(attributes[3].key, "Cover image");
        assert_eq!(attributes[3].value, "Multicolor");
        assert_eq!(attributes[4].key, "Foil pattern");
        assert_eq!(attributes[4].value, "Gold");
        assert_eq!(attributes[5].key, "Edition");
        assert_eq!(attributes[5].value, "8862");
    }

    #[test]
    fn test_extract_simple_nft() {
        let object_type = ObjectType::from_str(
            "0xa760a12f086fbb98687427e2524cf218f752c43fac3ffdc0b918284b23a2765d::nft::Nft",
        )
        .unwrap();
        let struct_tag = StructTag {
            address: AccountAddress::from_str(
                "0xa760a12f086fbb98687427e2524cf218f752c43fac3ffdc0b918284b23a2765d",
            )
            .unwrap(),
            module: Identifier::new("ika_chan_nft").unwrap(),
            name: Identifier::new("IkaChanNft").unwrap(),
            type_params: vec![], // No type parameters in this case
        };
        let object_data = SuiObjectData {
            object_id: ObjectID::from_str(
                "0x81163556c33a6cd6ab3f1b046ed2755d58f62b9234d06da246b7189b196d8f7d",
            )
            .unwrap(),
            version: 664719341.into(),
            digest: ObjectDigest::from_str("GBXs3hHdQRgoGgH2Ee1YKBCjgSWYgvfaY9JkAxaHkqjF").unwrap(),
            type_: Some(object_type.clone()),
            owner: None,
            previous_transaction: None,
            storage_rebate: Some(100),
            content: Some(SuiParsedData::MoveObject(SuiParsedMoveObject {
                type_: struct_tag,
                has_public_transfer: true,
                fields: SuiMoveStruct::WithFields(
                    serde_json::from_value(serde_json::json!({
                        "attributes": [
                          {
                            "type": "0xa760a12f086fbb98687427e2524cf218f752c43fac3ffdc0b918284b23a2765d::nft::Attributes",
                            "fields": {
                              "key": "Eyes",
                              "value": "X-Ray Vision"
                            }
                          },
                          {
                            "type": "0xa760a12f086fbb98687427e2524cf218f752c43fac3ffdc0b918284b23a2765d::nft::Attributes",
                            "fields": {
                              "key": "Headwear",
                              "value": "Fez"
                            }
                          },
                          {
                            "type": "0xa760a12f086fbb98687427e2524cf218f752c43fac3ffdc0b918284b23a2765d::nft::Attributes",
                            "fields": {
                              "key": "Expression",
                              "value": "Grinning"
                            }
                          },
                          {
                            "type": "0xa760a12f086fbb98687427e2524cf218f752c43fac3ffdc0b918284b23a2765d::nft::Attributes",
                            "fields": {
                              "key": "Environment",
                              "value": "Deep Ocean"
                            }
                          }
                        ],
                    "description": "This colletion will rule the world",
                    "id": {
                      "id": "0x81163556c33a6cd6ab3f1b046ed2755d58f62b9234d06da246b7189b196d8f7d"
                    },
                    "image_url": "https://example-image-url.com/nft8.png",
                    "name": "My awesome NFT #8",
                    "rarity": "66"
                    })).unwrap()
                ),
            })),
            display: Some(
                DisplayFieldsResponse {
                    data: serde_json::from_value(serde_json::json!({
                        "banner_image": "https://testers.com/banner.png",
                        "collection_description": "Nft is a collection of unique NFTs representing digital collectibles.",
                        "collection_name": "Nft",
                        "cover_url": "https://testers.com/cover.png",
                        "creator": "Nft Team",
                        "description": "This colletion will rule the world",
                        "image_url": "https://example-image-url.com/nft8.png",
                        "name": "My awesome NFT #8",
                        "project_url": "https://testers.com",
                        "rarity": "66"
                    })).unwrap(),
                    error: None

                }
            ),
            bcs: None,
        };

        let nft_data = NftData::from_object_data(&object_data).unwrap();
        let attributes = nft_data.attributes.unwrap();
        assert_eq!(attributes.len(), 4);
        assert_eq!(attributes[0].key, "Eyes");
        assert_eq!(attributes[0].value, "X-Ray Vision");
        assert_eq!(attributes[1].key, "Headwear");
        assert_eq!(attributes[1].value, "Fez");
        assert_eq!(attributes[2].key, "Expression");
        assert_eq!(attributes[2].value, "Grinning");
        assert_eq!(attributes[3].key, "Environment");
        assert_eq!(attributes[3].value, "Deep Ocean");
    }
}
