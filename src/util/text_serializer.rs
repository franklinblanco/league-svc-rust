use league_types::domain::sport::Sport;

pub fn parse_sport_list() -> Vec<Sport> {
    let file = include_str!("../../sport_list.txt");
    file.split(",\n")
        .enumerate()
        .map(|(index, value)| Sport {
            id: index as i32,
            name: String::from(value),
            category_id: 0,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_sport_list_test() {
        assert_eq!(parse_sport_list().len(), 93);
    }
}
