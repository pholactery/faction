extern crate redis;

use {get_faction_list, add_faction, update_faction};

#[test]
fn faction_list_returns_vector() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    redis::cmd("DEL").arg("factions").execute(&con);
    redis::cmd("SADD")
        .arg("factions")
        .arg("ninjas")
        .execute(&con);
    redis::cmd("SADD")
        .arg("factions")
        .arg("pirates")
        .execute(&con);

    match get_faction_list(&con) {
        Ok(v) => {
            assert_eq!(2, v.len());
            assert!(v.contains(&"ninjas".to_string()));
            assert!(v.contains(&"pirates".to_string()));
        }
        Err(_) => {
            assert!(false);
        }
    }

    redis::cmd("DEL").arg("factions").execute(&con);
}

#[test]
fn add_faction_adds_to_set_and_hash() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    redis::cmd("DEL").arg("factions").execute(&con);
    redis::cmd("HDEL")
        .arg("faction:ninjas")
        .arg("displayname")
        .execute(&con);
    let _ = add_faction(&con, "ninjas", "Stealth Killers");
    let s: Vec<String> = redis::cmd("SMEMBERS")
        .arg("factions")
        .query(&con)
        .unwrap();
    assert_eq!(s.len(), 1);
    assert_eq!(&s, &["ninjas"]);

    let displayname: String = redis::cmd("HGET")
        .arg("faction:ninjas")
        .arg("displayname")
        .query(&con)
        .unwrap();
    assert_eq!(displayname, "Stealth Killers");

    redis::cmd("DEL").arg("factions").execute(&con);
    redis::cmd("HDEL")
        .arg("faction:ninjas")
        .arg("displayname")
        .execute(&con);
}

#[test]
fn update_faction_increments_hash_key() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    redis::cmd("HDEL")
        .arg("player:kevin:factions")
        .arg("ninjas")
        .execute(&con);
    match update_faction(&con, "kevin", "ninjas", 51) {
        Ok(count) => {
            assert_eq!(count, 51);
        }
        Err(_) => {
            assert!(false);
        }
    };
    match update_faction(&con, "kevin", "ninjas", -12) {
        Ok(count) => assert_eq!(count, 51 - 12),
        Err(_) => assert!(false),
    };
}