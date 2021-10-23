
/* import the serializing libraries so that we are able to convert our variables into data that will 
be permanently stored on the blockchain AKA turning data into serials to store in the near blockchain*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

pub struct NftOwners {

    owners: UnorderedMap;
  
}