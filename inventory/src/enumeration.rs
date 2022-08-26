use near_sdk::PromiseOrValue;
use crate::*;

#[near_bindgen]
impl InventoryContract {
    //Query for the total supply of NFTs on the contract
    pub fn assets_total_supply(&self) -> U128 {
        //return the length of the token metadata by ID
        U128(self.token_metadata_by_id.len() as u128)
    }

    pub fn _asset_token(&self, token_id: AssetTokenId) -> Option<JsonAssetToken> {
        //if there is some token ID in the tokens_by_id collection
        let token_opt = self.tokens_by_id.get(&token_id);
        if token_opt.is_none() {
            return None
        }
        let token = token_opt.unwrap();
        // we'll get the metadata for that token
        let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
        let licenses = self.token_licenses_by_id.get(&token_id);
        // we return the JsonAssetToken (wrapped by Some since we return an option)
        let asset = JsonAssetToken {
            token_id,
            owner_id: token.owner_id,
            metadata,
            licenses,
            minter_id: token.minter_id,
            available_licenses: None,
        };
        Some(asset)
    }

    //get the information for a specific token ID
    pub fn asset_token(&self, token_id: AssetTokenId, opt: Option<AssetTokenOpt>) -> PromiseOrValue<Option<JsonAssetToken>> {
        let asset = self._asset_token(token_id.clone());
        if asset.is_none() {
            return PromiseOrValue::Value(None)
        }
        if opt.is_some() {
            let asset_opt = opt.unwrap();
            return if asset_opt.list_available.unwrap_or(false) {
                // Populate available licenses list
                PromiseOrValue::Promise(
                    self.get_available_list_for_asset(asset.as_ref().unwrap())
                )
            } else {
                PromiseOrValue::Value(asset)
            }
        }
        return PromiseOrValue::Value(asset)
    }
    
    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn asset_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonAssetToken> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.token_metadata_by_id.keys()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self._asset_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //get the total supply of asset tokens for a given owner
    pub fn asset_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            U128(tokens_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn asset_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonAssetToken> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self._asset_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}
