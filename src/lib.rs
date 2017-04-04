pub fn get_faction_list<'a>() -> Result<Vec<&'a str>, &'a str> {
    let factions = vec!["assasins",
         "ninjas",
         "philosophers"];
    Ok(factions)
}

#[cfg(test)]
mod tests;
