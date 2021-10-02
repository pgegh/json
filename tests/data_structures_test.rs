use json::data_structures;

#[test]
fn test_ws() {
    let mut white_space = String::new();
    assert_eq!(Err("The string is empty"), data_structures::Ws::new(white_space.clone()));

    white_space.push(0x20 as char);
    white_space.push(0x0A as char);
    white_space.push(0x0D as char);
    white_space.push(0x09 as char);
    let ws = data_structures::Ws::new(white_space.clone()).unwrap();
    assert_eq!(white_space, ws.to_string());
    // Testing with an illegal string
    white_space.push(0x0B as char);
    assert_eq!(Err("The string contains illegal characters"), data_structures::Ws::new(white_space));
}