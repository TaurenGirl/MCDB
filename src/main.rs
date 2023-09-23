mod xml_utilities;
fn main() {
    let test = xml_utilities::search("Cyclops".to_string(), "Name".to_string(), "./src/info.xml".to_string());
    println!("{}", test);
}
