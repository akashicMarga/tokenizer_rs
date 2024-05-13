mod basic;
mod utils;

fn main() {
    let text = "aaabdaaabac";
    let mut tokenizer = basic::BasicTokenizer::new();
    tokenizer.train(text, 256 + 3, false);
    println!("tokenized text {:?}", tokenizer.encode(text));
    println!(
        "decoded text {:?}",
        tokenizer.decode(&[258, 100, 258, 97, 99])
    );
}
