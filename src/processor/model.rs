use std::collections::BTreeMap;

use serde_json::Value;
use sui_sdk::{rpc_types::SuiParsedData, types::base_types::ObjectID};

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
    // pub fn new(token_id: String, collection_type: String, digest: String) -> Self {
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

    pub fn from_object_data(
        object_id: ObjectID,
        data: &sui_sdk::rpc_types::SuiObjectData,
    ) -> Option<Self> {
        let type_string = data.type_.as_ref()?.to_string();
        let mut nft = Self::new(NftDataParams {
            token_id: object_id.to_string(),
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
                    if let Some(attrs) = Self::extract_attributes(attributes) {
                        self.attributes = Some(attrs);
                    }
                }
            }
        }
    }

    fn extract_attributes(value: &Value) -> Option<Vec<AttributeAssembly>> {
        if let Value::Array(vec) = value {
            let mut attrs = Vec::new();
            for item in vec {
                if let Value::Object(obj) = item {
                    if let Some(fields) = obj.get("fields") {
                        if let Value::Object(field_map) = fields {
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
                    }
                }
            }
            if !attrs.is_empty() {
                return Some(attrs);
            }
        }
        None
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
