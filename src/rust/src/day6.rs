use wasm_bindgen::prelude::*;

fn get_marker(message: String, distinct_characters_amount: i32) -> i32 {
    // make it a vector
    let message_arr: Vec<&str> = message.split("").collect();
    let mut current_pos = 0;
    // iterate over each item in the array
    for (i, _char) in message_arr.iter().enumerate() {
        current_pos += 1;
        if i <= (distinct_characters_amount - 1).try_into().unwrap() {
            continue;
        };

        // get current + last x items
        let mut items: Vec<&str> = vec![];
        for j in 0..distinct_characters_amount {
            items.push(message_arr[i - j as usize])
        }

        let mut deduped_items = items.clone();
        deduped_items.sort();
        deduped_items.dedup();

        println!("items: {:?}, deduped_items: {:?}", items, deduped_items);

        if deduped_items.len() == items.len() {
            current_pos -= 1;
            break;
        }
    }

    current_pos
}

#[wasm_bindgen]
pub fn get_start_packet_marker(message: String) -> i32 {
    get_marker(message, 4)
}

#[wasm_bindgen]
pub fn get_start_message_marker(message: String) -> i32 {
    get_marker(message, 14)
}