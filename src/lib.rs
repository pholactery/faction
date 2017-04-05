extern crate redis;

use redis::{RedisResult};

pub fn get_faction_list(con: &redis::Connection) -> Result<Vec<String>, String> {   
    let res: RedisResult<Vec<String>> = redis::cmd("SMEMBERS").arg("factions").query(con);
    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}", e)),
    }        
}

pub fn add_faction(con: &redis::Connection, name: &str, displayname: &str) -> Result<(), u8> {
    redis::cmd("SADD").arg("factions").arg(name).execute(con);
    redis::cmd("HSET").arg(format!("faction:{}", name)).arg("displayname").arg(displayname).execute(con);
    Ok(())
}

pub fn update_faction(con: &redis::Connection, player:&str, faction: &str, amount: i32) -> Result<i32, String> {
    let res: RedisResult<i32> = 
      redis::cmd("HINCRBY").arg(format!("player:{}:factions", player)).arg(faction).arg(amount).query(con);
    match res {
        Ok(fac) => Ok(fac),
        Err(e) => Err(format!("{}", e)),
    }
}
 
#[cfg(test)]
mod tests;
