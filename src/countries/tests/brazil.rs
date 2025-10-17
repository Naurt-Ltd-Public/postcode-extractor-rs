use crate::{PostCodeParser, countries::BrRegex};

#[test]
fn test_brazil() {
    let regex = BrRegex::new();

    let input =
        "Fulano da Silva, Rua Governador Roberto Silveira, 108, Centro, Macaé - RJ, 27910-050";

    let post = regex.evaluate(input, true).unwrap();

    assert_eq!(post.base, "27910-050");
}
