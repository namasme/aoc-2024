pub type LocationID = i32;

pub fn parse_input(input: &str) -> (Vec<LocationID>, Vec<LocationID>) {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once("   ").unwrap();
            (
                parts.0.parse::<i32>().unwrap(),
                parts.1.parse::<i32>().unwrap(),
            )
        })
        .unzip()
}
