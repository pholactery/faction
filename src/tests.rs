extern crate redis;

use {get_faction_list, add_faction};

#[test]
fn faction_list_returns_vector() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    redis::cmd("DEL").arg("factions").execute(&con);
    redis::cmd("SADD").arg("factions").arg("ninjas").execute(&con);
    redis::cmd("SADD").arg("factions").arg("pirates").execute(&con);

    match get_faction_list(&con) {
        Ok(v) => {
            assert_eq!(2, v.len());
            assert!(v.contains(&"ninjas".to_string()));
            assert!(v.contains(&"pirates".to_string()));
        },
        Err(_) => {
            assert!(false);
        }
    }

    redis::cmd("DEL").arg("factions").execute(&con);
}

#[test]
fn add_faction_adds_to_set() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    redis::cmd("DEL").arg("factions").execute(&con);
    let _ = add_faction(&con, "ninja");
    let s: Vec<String> = redis::cmd("SMEMBERS").arg("factions").query(&con).unwrap();
    assert_eq!(s.len(), 1);
    assert_eq!(&s, &["ninja"]);

    redis::cmd("DEL").arg("factions").execute(&con);
}