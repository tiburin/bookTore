use indexmap::IndexMap;
use std::collections::HashSet;

use std::{self, fs};
static SENTENCE_START: isize = 3;
static SENTENCE_END: usize = 9;
static TAKE: usize = 3;
const SPANISH: bool = false;

fn get_spanish(letter: char) -> bool {
    let mut store = HashSet::new();
    "ñáíéóúü".chars().for_each(|n| {
        store.insert(n);
    });
    store.contains(&letter)
}

struct English;
impl English {
    fn start() -> i32 {
        97
    }
    fn end() -> i32 {
        122
    }

    fn is_range(letter: char) -> bool {
        let value = letter as i32;
        let is_english = value >= English::start() && value <= English::end();

        if SPANISH {
            is_english || get_spanish(letter)
        } else {
            is_english
        }
    }

    fn valid_english(input: &str) -> bool {
        for letter in input.chars() {
            if !English::is_range(letter) {
                return false;
            }
        }
        input.len() > 0
    }
}
struct Str;
impl Str {
    fn rm_start(input: &str) -> &str {
        let mut start = 0;
        let mut list = input.chars();
        let mut current = list.next();
        while let Some(letter) = current {
            if !English::is_range(letter) {
                start += letter.len_utf8();
            } else {
                return &input[start..input.len()];
            }
            current = list.next();
        }
        &input[start..input.len()]
    }
    fn rm_end(input: &str) -> &str {
        let mut end = Str::utf_count(input);
        let mut list = input.chars();
        let mut current = list.next_back();
        while let Some(letter) = current {
            if !English::is_range(letter) {
                end -= letter.len_utf8();
            } else {
                return &input[0..end];
            }
            current = list.next_back();
        }
        &input[0..end]
    }
    fn rm_start_end(input: &str) -> &str {
        Str::rm_end(Str::rm_start(input))
    }
    fn utf_count(input: &str) -> usize {
        input.chars().fold(0, |acc, letter| acc + letter.len_utf8())
    }
}

fn get_sentence_end(index_at: usize, inner: &Inner) -> String {
    if index_at >= inner.list.len() {
        return format!("{}", "");
    }
    let mut index_end = index_at + SENTENCE_END;

    while index_end > inner.list.len() {
        index_end -= 1;
    }

    format!("{}", &inner.list[index_at..index_end].join(" "))
}
fn get_sentence_start(index_at: isize, end_at: usize, inner: &Inner) -> String {
    if index_at < 0 {
        return format!("{}", "");
    }
    let mut start_point = index_at - SENTENCE_START;

    while start_point < 0 {
        start_point += 1;
    }

    format!("{}", &inner.list[start_point as usize..end_at].join(" "))
}

#[derive(Debug)]
struct Data {
    sentences: Vec<usize>,
}
struct Inner {
    store: IndexMap<String, Data>,
    list: Vec<String>,
}

pub fn parse_word(word: &str) -> String {
    let word = word.trim().to_lowercase();
    Str::rm_start_end(&word).to_owned()
}

fn parse_content(content: String) -> Inner {
    let mut store: IndexMap<String, Data> = IndexMap::new();
    let list: Vec<String> = content.split_whitespace().map(|n| n.to_owned()).collect();

    for (index, line) in list.clone().into_iter().enumerate() {
        let word = parse_word(&line);
        if let Some(data) = store.get_mut(&word) {
            data.sentences.push(index);
        } else {
            let data = Data {
                sentences: vec![index],
            };
            store.insert(word, data);
        }
    }

    Inner { store, list }
}

fn compose_sentence(content: String) -> String {
    let mut total = 0;
    let list: Vec<_> = content
        .split_ascii_whitespace()
        .take_while(|a| {
            total += a.len();
            total <= 50
        })
        .collect();

    list.join(" ")
}

fn stitch_words_sentences(index: usize, inner: &Inner) -> String {
    let left = get_sentence_start(index as isize - 1, index, inner);
    let word = &inner.list[index];
    let right = get_sentence_end(index + 1, inner);
    let right = compose_sentence(right);

    format!("{}, {} {} \n", left, word, right)
}

fn get_popularity_sort(inner: &Inner) -> Vec<(String, &Data)> {
    let mut acc = vec![];
    for (word, data) in &inner.store {
        if English::valid_english(word) && word.len() > 1 {
            acc.push((word.to_owned(), data));
        }
    }

    acc.sort_by(|(_, a), (_, b)| a.sentences.len().cmp(&b.sentences.len()).reverse());
    let todo: Vec<_> = acc.clone().iter().map(|(w, _)| w.to_owned()).collect();
    fs::write("palabras.on", todo.join("\n")).unwrap();
    acc
}

fn get_public_domain_books() -> String {
    let dir_content = fs_extra::dir::get_dir_content("public_domain").unwrap();
    let mut acc = String::new();
    for file_name in dir_content.files {
        if !file_name.contains(".txt") {
            continue;
        }
        let content = fs::read_to_string(file_name).unwrap();
        let list: Vec<_> = content.split_ascii_whitespace().map(|n| n.trim()).collect();

        let content = list.join(" ");
        acc.push_str(&content);
    }
    acc
}
fn get_content_single_file() -> Vec<String> {
    let content = fs::read_to_string("word.on").unwrap();
    let mut acc = vec![];
    for word in content.split("\n").map(|n| n.trim()).filter(|n| n != &"") {
        acc.push(word.to_owned());
    }
    acc
}

fn get_word_file(inner: &Inner) -> Vec<(String, &Data)> {
    let mut acc = vec![];

    let list_of_mas_words = get_content_single_file();

    for word in list_of_mas_words {
        if inner.store.contains_key(&word) {
            let data = inner.store.get(&word).unwrap();
            acc.push((word, data));
        }
    }
    let todo: Vec<_> = acc.clone().iter().map(|(w, _)| w.to_owned()).collect();
    fs::write("palabras.on", todo.join("\n")).unwrap();
    acc
}

fn get_insertion_sort(inner: &Inner) -> Vec<(String, &Data)> {
    let mut acc = vec![];
    for (word, data) in &inner.store {
        acc.push((word.to_owned(), data));
    }
    acc
}

fn main() {
    for name in vec!["word.on", "word.off", "palabras.on", "data.on"] {
        if !std::path::Path::new(name).exists() {
            fs::File::create(name).unwrap();
        }
    }
    let content = get_public_domain_books();
    let inner = parse_content(content);

    let acc = match "popularity" {
        "insertion" => get_insertion_sort(&inner),
        "word" => get_word_file(&inner),
        _ => get_popularity_sort(&inner),
    };
    let mut release = vec![];
    for (word, data) in &acc {
        let x = data
            .sentences
            .iter()
            .take(TAKE)
            .fold(String::new(), |mut acc, b| {
                acc.push_str(&stitch_words_sentences(*b, &inner));
                acc
            });

        let sentence = format!("{}\n", x.trim());
        release.push((word, sentence));
    }
    let ready: Vec<String> = release
        .iter()
        .enumerate()
        .map(|(index, (w, s))| {
            format!(
                "
                            {}: {}\n{}",
                index + 1,
                w,
                s
            )
        })
        .collect();

    println!("Total: {:?}", &ready.len());
    fs::write("data.on", ready.join("\n")).unwrap();
}
