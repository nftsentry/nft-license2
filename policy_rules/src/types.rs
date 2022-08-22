use crate::*;

pub type TokenId = String;
pub type AssetId = String;
//defines the payout type we'll be returning as a part of the royalty standards.

pub trait LicenseGeneral {
    fn is_exclusive(&self) -> bool;
    fn is_personal(&self) -> bool;
    fn is_commercial(&self) -> bool;
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenExtra {
    asset_id_path: String
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

impl TokenMetadata {
    pub fn inventory_asset_license(&self) -> (String, String, String) {
        let extra: TokenExtra = serde_json::from_str(
            self.extra.as_ref().unwrap().as_str()
        ).expect("Failed parse token_metadata.extra");
        let splitted: Vec<_> = extra.asset_id_path.split("/").collect();
        return if splitted.len() >= 3 {
            (splitted[0].to_string(), splitted[1].to_string(), splitted[2].to_string())
        } else {
            ("".to_string(), "".to_string(), "".to_string())
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct LicenseData {
    pub i_agree: bool,
    pub perpetuity: bool,
    pub exclusivity: bool,
    pub personal_use: bool,
    pub commercial_use: bool,
    pub limited_display_sublicensee: bool,
    pub template: Option<String>,
    pub pdf_url: Option<String>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[derive(PartialEq, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenLicense {
    // pub test: u8,
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub issuer_id: Option<AccountId>, // AccountId of the license issuer
    pub uri: Option<String>, // URL to associated pdf, preferably to decentralized, content-addressed storage
    pub metadata: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(Debug)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    // token_id: TokenId,
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //owner of the token
    pub asset_id: AssetId,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //the next approval ID to give out.
    pub next_approval_id: u64,
    //keep track of the royalty percentages for the token in a hash map
    pub royalty: HashMap<AccountId, u32>,
}

//The Json token is what will be returned from view calls.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct LicenseToken {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //asset id of the token
    pub asset_id: AssetId,
    //token metadata
    pub metadata: TokenMetadata,
    // license metadata
    pub license: Option<TokenLicense>,
    // proposed license
    // pub proposed_license: TokenLicense,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    pub royalty: HashMap<AccountId, u32>,
}

impl LicenseToken {
    pub fn as_inventory_license(&self, price: Option<String>) -> Option<InventoryLicense> {
        if self.license.is_none() {
            return None
        }
        let license_data: LicenseData = serde_json::from_str(
            self.license.as_ref().unwrap().metadata.as_ref().unwrap().as_str()
        ).expect("Failed parse license metadata");
        let (_, _, lic_id) = self.metadata.inventory_asset_license();
        let res = InventoryLicense {
            license_id: lic_id,
            title: self.license.as_ref().unwrap().title.as_ref().unwrap().to_string(),
            license: license_data,
            price: if price.is_some() {price.unwrap()} else {"".to_string()},
        };
        Some(res)
    }
}

impl LicenseGeneral for LicenseToken {
    fn is_exclusive(&self) -> bool {
        let lic = self.as_inventory_license(None);
        lic.unwrap().license.exclusivity
    }

    fn is_personal(&self) -> bool {
        let lic = self.as_inventory_license(None);
        lic.unwrap().license.personal_use
    }

    fn is_commercial(&self) -> bool {
        let lic = self.as_inventory_license(None);
        !lic.unwrap().license.personal_use
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTokenLicense {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub license: TokenLicense,
    // proposed license
    // pub proposed_license: TokenLicense,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
}

pub type AssetTokenId = String;
pub type AssetMinterContractId = String;
pub type AssetLicenses = Vec<AssetLicense>;
pub type InventoryLicenses = Vec<InventoryLicense>;

/// This spec can be treated like a version of the standard.
pub const INVENTORY_METADATA_SPEC: &str = "inventory-1.0.0";

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryLicense {
    pub license_id: String,
    pub title: String,
    pub price: String,
    pub license: LicenseData,
}

impl LicenseGeneral for InventoryLicense {
    fn is_exclusive(&self) -> bool {
        self.license.exclusivity
    }

    fn is_personal(&self) -> bool {
        self.license.personal_use
    }

    fn is_commercial(&self) -> bool {
        !self.license.personal_use
    }
}

impl InventoryLicense {
    pub fn as_license_token(&self, token_id: String) -> Result<LicenseToken, String> {
        let res = serde_json::to_string(&self.license);
        if res.is_err() {
            return Err(res.err().unwrap().to_string())
        }
        let meta = res.unwrap();
        let mut token_metadata = TokenMetadata::default();
        token_metadata.title = Some(self.title.clone());
        token_metadata.extra = Some(extra_reference_to_id(
            "".to_string(),
            "asset_id_path".to_string(),
            format!("{}/{}/{}", "inv", "asset", self.license_id),
        ));
        let res = LicenseToken{
            token_id,
            license: Some(TokenLicense{
                title: Some(self.title.clone()),
                metadata: Some(meta),
                description: None,
                expires_at: None,
                issued_at: None,
                issuer_id: None,
                reference: None,
                reference_hash: None,
                starts_at: None,
                updated_at: None,
                uri: None,
            }),
            approved_account_ids: Default::default(),
            royalty: Default::default(),
            owner_id: AccountId::new_unchecked("alice".to_string()),
            asset_id: "".to_string(),
            metadata: token_metadata,
        };
        Ok(res)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetLicense {
    pub license_id: String,
    pub title: String,
    pub price: Option<String>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub licenses: InventoryLicenses,            // required, ex. "MOSIAC"
    pub default_minter_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Deserialize, Serialize)]
#[derive(PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetTokenMetadata {
    // The title for this token
    pub title: Option<String>,
    // The free-form description for this token
    pub description: Option<String>,
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>,
    // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub media_hash: Option<Base64VecU8>,
    // number of copies of this set of metadata in existence when token was minted.
    pub copies: Option<u64>,
    // When token was issued or minted, Unix epoch in milliseconds
    pub issued_at: Option<u64>,
    // When token expires, Unix epoch in milliseconds
    pub expires_at: Option<u64>,
    // When token starts being valid, Unix epoch in milliseconds
    pub starts_at: Option<u64>,
    // When token was last updated, Unix epoch in milliseconds
    pub updated_at: Option<u64>,
    // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub extra: Option<String>,
    // URL to an off-chain JSON file with more info.
    pub reference: Option<String>,
    // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
//#[derive(Debug)]
pub struct AssetToken {
    // token_id: AssetTokenId,
    pub token_id: AssetTokenId,
    //owner of the token
    pub owner_id: AccountId,
    //minter of the token
    pub minter_id: Option<AccountId>,
    //token metadata
    pub metadata: AssetTokenMetadata,
    // list of approved licenses available for this token
    pub licenses: Option<AssetLicenses>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InventoryLicenseAvailability {
    pub inventory_license:    InventoryLicense,
    pub available:            bool,
    pub reason_not_available: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullInventory {
    pub inventory_licenses: Vec<InventoryLicense>,
    pub issued_licenses:    Vec<LicenseToken>,
}

pub fn extra_reference_to_id(extra_orig: String, key: String, value: String) -> String {
    // 1. Parse extra as JSON and save to map
    let mut extra_new: HashMap<String, serde_json::Value> = HashMap::default();

    let result = serde_json::from_str(extra_orig.as_str());
    if result.is_err() {
        // 2. Invalid
        if extra_orig.len() > 1 {
            // Not empty, not JSON, push orig as "extra"
            extra_new.insert("extra".to_string(), extra_orig.parse().unwrap());
        }
    } else {
        // let parsed: serde_json::Value = result.unwrap();
        // extra_new = parsed.as_object().unwrap().clone();
        extra_new = result.unwrap();
    }
    extra_new.insert(key, serde_json::Value::String(value));
    let extra_raw = serde_json::to_string(&extra_new).expect("Failed serialize to JSON");
    extra_raw
}

pub fn  extra_reference_for_asset_path(extra_orig: String, inv_id: String, asset_id: String, license_id: String) -> String {
    return extra_reference_to_id(
        extra_orig,
        "asset_id_path".to_string(),
        format!("{}/{}/{}", inv_id, asset_id, license_id),
    )
}