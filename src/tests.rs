use {get_faction_list};

#[test]
fn faction_list_returns_dummies() {
    let faclist = get_faction_list().unwrap();

    assert_eq!(3, faclist.len());
    assert_eq!("assasins", faclist[0]);
}