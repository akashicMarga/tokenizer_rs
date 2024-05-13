use crate::utils;
use std::collections::HashMap;

pub struct BasicTokenizer {
    vocab: HashMap<i32, Vec<u8>>,
    merges: HashMap<(i32, i32), i32>,
}

impl BasicTokenizer {
    pub fn new() -> Self {
        Self {
            vocab: HashMap::new(),
            merges: HashMap::new(),
        }
    }
    pub fn decode(&self, ids: &[i32]) -> String {
        // Given ids (list of integers), return Rust string
        let text_bytes: Vec<u8> = ids
            .iter()
            .flat_map(|&idx| self.vocab[&idx].clone())
            .collect();
        String::from_utf8_lossy(&text_bytes).into_owned()
    }

    pub fn encode(&self, text: &str) -> Vec<i32> {
        // Given a string text, return the token ids
        let text_bytes = text.as_bytes();
        let mut ids: Vec<i32> = text_bytes.iter().map(|&b| b as i32).collect();
        while ids.len() >= 2 {
            // Find the pair with the lowest merge index
            let stats = utils::get_stats(&ids);

            let pair_opt = stats
                .keys()
                .filter_map(|&pair| self.merges.get(&pair).map(|_| pair))
                .min_by_key(|&pair| self.merges[&pair]);

            match pair_opt {
                None => break, // If there are no more merges available, break
                Some(pair) => {
                    // Otherwise, merge the best pair (lowest merge index)
                    let idx = self.merges[&pair];
                    ids = utils::merge(&ids, pair, idx);
                }
            };
        }
        ids
    }

    pub fn train(&mut self, text: &str, vocab_zize: usize, verbose: bool) {
        assert!(vocab_zize >= 256);
        let num_merges = vocab_zize - 256;

        // Input text preprocessing
        let text_bytes = text.as_bytes();
        let mut ids: Vec<i32> = text_bytes.iter().map(|&b| b as i32).collect();

        //iteratively merge the most common pairs to create new tokens
        let mut merges: HashMap<(i32, i32), i32> = HashMap::new();
        let mut vocab: HashMap<i32, Vec<u8>> = (0..256).map(|idx| (idx, vec![idx as u8])).collect();

        for i in 0..num_merges {
            let stats = utils::get_stats(&ids);
            let pair = stats.iter().max_by_key(|(_, count)| *count).unwrap().0;

            let idx: i32 = 256 as i32 + i as i32;

            ids = utils::merge(&ids, *pair, idx);
            merges.insert(*pair, idx);
            vocab.insert(
                idx as i32,
                [vocab[&pair.0].clone(), vocab[&pair.1].clone()].concat(),
            );
            // vocab.insert(idx as u8, &vocab[&pair.0] + &vocab[&pair.1]);
            if verbose {
                println!(
                    "merge {} / {}: {:?} -> {:?} ({:?}) had {} occurrences",
                    i + 1,
                    num_merges,
                    pair,
                    idx,
                    vocab[&(idx as i32)],
                    stats.get(pair).unwrap()
                );
            }
        }

        self.merges = merges;
        self.vocab = vocab;

        // println!("Merge = {:?} and vocab= {:?}", self.merges, self.vocab);
    }
}
