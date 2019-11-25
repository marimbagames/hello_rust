use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    red: i32,
    blue: i32,
    green: i32,
}

fn main() {
    let my_color = Color { red: 10, blue: 20, green: 30 };
    let my_color_ser = serde_json::to_string(&my_color).unwrap();
    println!("stringified: {}", my_color_ser);
    let deserialized: Color = serde_json::from_str(&my_color_ser).unwrap();
    println!("deserialized: {:?}", deserialized);


    println!("{:?}", two_sum(vec![2, 7, 11, 15], 22));
    let mut myvec = vec![1, 2, 3];
    add_one(&mut myvec);
    println!("{:?}", myvec);
    let mystr = "hello".to_string();
    let refstr = &mystr;
    println!("{}", refstr.len());
    println!("{:?}", group_anagrams(&vec!["eat", "tea", "art", "tan", "are", "tar", "ate", "nat", "bat", "era", "rat", "ear"]));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Result<Vec<i32>, String> {
    let l = nums.len();
    for first in 0..l-1 {
        for second in first..l {
            if nums[first] + nums[second] == target {
                return Ok(vec![first as i32, second as i32]);
            }
        }
    }
    return Err(String::from("Result not found"));
}

fn add_one(vals: &mut Vec<i32>) {
    for v in vals {
        *v += 1;
    }
}

fn group_anagrams<'a>(word_list: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut grouped_words = HashMap::new();
    for word in word_list {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let sorted_word: String = chars.into_iter().collect();
        if !grouped_words.contains_key(&sorted_word) {
            grouped_words.insert(sorted_word.clone(), Some(Vec::<&str>::new()));
        }
        grouped_words.get_mut(&sorted_word).unwrap().as_mut().unwrap().push(word);
    }
    let mut result: Vec<Vec<&str>> = Vec::new();
    grouped_words.values_mut().for_each(|v| result.push(v.take().unwrap()));
    return result;
}
