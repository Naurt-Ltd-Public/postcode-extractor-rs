use crate::{PostCodeParser, countries::PtRegex};

#[test]
fn test_portugal() {
    let regex = PtRegex::new();

    let input = "R 3 Br Da Junqueira 13, 4405-872 Vila Nova De Gaia, Portugal";

    let post = regex.evaluate(input, true).unwrap();

    assert_eq!(post.base, "4405-872");
}
