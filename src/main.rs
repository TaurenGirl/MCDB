mod xml_utilities;
fn main() {
    let x = xml_utilities::populate("<Card>", "</Card>", "./src/info.xml".to_string());
    let y: &xml_utilities::Card = &x.contents[0];
    println!("{}", y.owner);
}
