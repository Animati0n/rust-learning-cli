#[test]
fn find_a_match() {
    let mut result= Vec::new();
   let _= grrs::find_matches("lorem ipsum\ndolor sit amet","lorem",&mut result);
    assert_eq!(result,b"lorem ipsum\n");
}
