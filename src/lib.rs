
/* import the serializing libraries so that we are able to convert our variables into data that will 
be permanently stored on the blockchain AKA turning data into serials to store in the near blockchain*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap; // import lib in order to use UnorderedMap type


near_sdk::setup_alloc!();
//make the data serializable using the following decorators
\#\[near_bindgen\]

\#\[derive(BorshDeserialize, BorshSerialize)\]

//create NFT data structure using struct
pub struct NftOwners {

    owners: UnorderedMap; //UnorderMap is a dictionary recognized by Near blockchain
  
}

//create a default for nftowner struct. Defaults are required for structs in Rust
impl Default for NftOwners {

    fn default() -> Self {
  
      Self { 
          //create a new UnorderedMap we also need to provide an identifier ”o” which will identify this map on the storage space.
  
          owners: UnorderedMap::new(b“o”.to_vec())
  
      }
  
    }
  
} 

\#\[near_bindgen\]

impl NftOwners { //implement functionality to nftowners
//setOwner creates an owner, mutable since owner can be altered
  pub fn setOwner(&mut self, tokenId: String, accountId: String){ 

    self.owners.insert(&tokenId, &accountId); 

  }

  //retrieves the owner of the token
  // return type of getOwner is Option meaning it can be a String or a null
pub fn getOwner(&self, tokenId: String) -> Option {

    return self.owners.get(&tokenId);

  }

}