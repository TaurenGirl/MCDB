use std::fs::read_to_string;
  
pub fn search(text: String, keyword: String, file_path: String) -> String {
    println!("Attempting to search in {}", file_path);
    let contents = read_to_string(file_path).expect("Should have been able to read file.");
    let contents_mut = contents.to_string();
    let match_start_tag = "<".to_string() + &keyword + "=" + &text + ">";
    let match_end_tag = "</".to_string() + &keyword + ">";
    let match_start_index = contents_mut.find(&match_start_tag);
    let match_end_index = contents_mut.find(&match_end_tag);
    if let Some(start_value) = match_start_index {
        if let Some(end_value) = match_end_index {
            let start_new = start_value + text.len() + keyword.len() + 3;
            let mut slice = &contents_mut[start_new..end_value];
            slice = slice.trim();
            return slice.to_string();
        } else {
            return "Error".to_string();
        }
      } else {
            return "Error".to_string();
      }
    }
    
pub fn write_to_xml(_name: &str, _aspect: &str, _owner: &str, _amount: u32) {
    todo!();
}


