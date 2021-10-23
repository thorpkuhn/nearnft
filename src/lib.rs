
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