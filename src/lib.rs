extern crate redis;

use redis::{RedisResult};

pub fn get_faction_list<'a>(con: &redis::Connection) -> Result<Vec<String>, String> {   
    let res: RedisResult<Vec<String>> = redis::cmd("SMEMBERS").arg("factions").query(con);
    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}", e)),
    }        
}

pub fn add_faction(con: &redis::Connection, name: &str) -> Result<(), u8> {
    redis::cmd("SADD").arg("factions").arg(name).execute(con);
    Ok(())
}
 

#[cfg(test)]
mod tests;
