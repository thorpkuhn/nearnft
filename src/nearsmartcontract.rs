/* import
A near account as 4 things:
1. Account Ident
2. Balance in near tokens
3. max of 1 contract 
4. storage

-when contracts are deployed, need create an account and put the contract inside 
-every contract automatically can handle and store money
- storage is independent of the contract space
- must store information in the contract that is standardize and usable outside the contract
- to this, information must be serializable ex. serializable struct and stored serializably 
- so when alterations are needed, data can be pulled from the storage, deserialize it, modify, serialize it back and stored again
*/