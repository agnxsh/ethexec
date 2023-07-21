use crate::storage::spec::Database;
use ethereum_types::U256;
use std::collections::HashMap;

pub struct StateDB {
    pub db: HashMap<U256, U256>
}
//initialising the HashMap for StateDB
impl StateDB {
    pub fn new() -> Self{
        Self{ db: HashMap::new()}
    }
}

//Adding Database (set, get) trait for StateDB
impl Database for StateDB {
    fn get(&self, key: U256) -> U256 {
        self.db.get(&key).cloned().unwrap_or_default()
        //makes a clone of the db entry or returns a default U256 value if not found 
    }

    fn set(&mut self, key:U256) -> U256 {
        if value == U256::default() {
            self.db.remove(&key);
        }
        else {
            self.db.insert(key, value);
        }
    }
}

