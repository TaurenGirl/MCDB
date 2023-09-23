use std::{fs::read_to_string, str::Chars};

pub struct Card {
    pub name: String,
    pub aspect: String,
    pub owner: String
}

pub struct Collection {
    pub contents: Vec<Card>
}

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


pub fn populate(start_tag: &str, end_tag: &str, file_path: String) -> Collection {
    let name_tag_start = "<Name>";
    let name_tag_end = "</Name>";
    let aspect_tag_start = "<Aspect>";
    let aspect_tag_end = "</Aspect>";
    let owner_tag_start = "<Owner>";
    let owner_tag_end = "</Owner>";
    let mut collection: Vec<Card> = Vec::new();
    println!("Attempting to search in {}", file_path);
    let contents = read_to_string(file_path).expect("Should have been able to read file.");
    let mut starts: Vec<_> = contents.match_indices(start_tag).collect();
    let mut ends: Vec<_> = contents.match_indices(end_tag).collect();
    for (i, el) in starts.clone().iter_mut().enumerate() {
        let start_actual = el.0 + el.1.len();
        let mut data_tmp = &contents[start_actual..ends[i].0];
        let card = Card {
            name: find(name_tag_start, name_tag_end, data_tmp),
            aspect: find(aspect_tag_start, aspect_tag_end, data_tmp),
            owner: find(owner_tag_start, owner_tag_end, data_tmp)
        };
        collection.push(card);
    }
    let mut output = Collection {
        contents: collection
    };
    return output;
}


fn find(tag_start: &str, tag_end: &str, data: &str) -> String {
    let x = data.find(tag_start);
    let y = data.find(tag_end);
    if let Some(start) = x {
        if let Some(end) = y {
            let start_actual = start + tag_start.len();
            let output = &data[start_actual..end];
            return output.to_string();
        } else {
            return "Error getting data for ".to_string() + tag_start;
        }
    } else {
        return "Error getting data for ".to_string() + tag_start;
    }
}
