use tokenizers::Tokenizer;

use crate::ChunkSizer;

impl ChunkSizer for Tokenizer {
    /// Returns the number of tokens in a given text after tokenization.
    ///
    /// # Panics
    ///
    /// Will panic if you don't have a byte-level tokenizer and the splitter
    /// encounters text it can't tokenize.
    fn chunk_size(&self, chunk: &str) -> usize {
        chunk_size(self, chunk)
    }
}

impl ChunkSizer for &Tokenizer {
    /// Returns the number of tokens in a given text after tokenization.
    ///
    /// # Panics
    ///
    /// Will panic if you don't have a byte-level tokenizer and the splitter
    /// encounters text it can't tokenize.
    fn chunk_size(&self, chunk: &str) -> usize {
        chunk_size(self, chunk)
    }
}

fn chunk_size(tokenizer: &Tokenizer, chunk: &str) -> usize {
    tokenizer
        .encode(chunk, false)
        .map(|enc| enc.len())
        .expect("Unable to tokenize the following string {str}")
}
