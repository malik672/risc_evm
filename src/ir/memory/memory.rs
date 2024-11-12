use std::collections::HashMap;

const WORD_SIZE: usize = 32; // 32 bytes per word
const CHUNK_SIZE: usize = 1024 * WORD_SIZE; // 32KB chunks (1024 words)

pub struct Memory {
    chunks: Vec<Vec<u8>>,
    sparse: HashMap<usize, [u8; WORD_SIZE]>,
    size: usize, // Size in bytes
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            chunks: vec![vec![0; CHUNK_SIZE]],
            sparse: HashMap::new(),
            size: 0,
        }
    }

    pub fn read_word(&self, index: usize) -> [u8; WORD_SIZE] {
        let aligned_index = index & !31; // Align to 32-byte boundary
        if aligned_index < self.chunks.len() * CHUNK_SIZE {
            let chunk_index = aligned_index / CHUNK_SIZE;
            let word_index = (aligned_index % CHUNK_SIZE) / WORD_SIZE;
            let start = word_index * WORD_SIZE;
            let end = start + WORD_SIZE;
            self.chunks[chunk_index][start..end].try_into().unwrap()
        } else {
            *self.sparse.get(&aligned_index).unwrap_or(&[0; WORD_SIZE])
        }
    }

    pub fn write_word(&mut self, index: usize, value: [u8; WORD_SIZE]) {
        let aligned_index = index & !31; // Align to 32-byte boundary
        if aligned_index >= self.size {
            self.size = aligned_index + WORD_SIZE;
        }

        if aligned_index < self.chunks.len() * CHUNK_SIZE {
            let chunk_index = aligned_index / CHUNK_SIZE;
            let word_index = (aligned_index % CHUNK_SIZE) / WORD_SIZE;
            let start = word_index * WORD_SIZE;
            let end = start + WORD_SIZE;
            self.chunks[chunk_index][start..end].copy_from_slice(&value);
        } else {
            self.sparse.insert(aligned_index, value);
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn expand(&mut self, new_size: usize) {
        let aligned_new_size = (new_size + 31) & !31; // Round up to nearest 32-byte boundary
        if aligned_new_size > self.size {
            self.size = aligned_new_size;
            let required_chunks = (aligned_new_size + CHUNK_SIZE - 1) / CHUNK_SIZE;
            while self.chunks.len() < required_chunks {
                self.chunks.push(vec![0; CHUNK_SIZE]);
            }
        }
    }

    // Helper method to read a single byte (for compatibility with byte-level operations)
    pub fn read_byte(&self, index: usize) -> u8 {
        let word = self.read_word(index);
        word[index % WORD_SIZE]
    }

    // Helper method to write a single byte (for compatibility with byte-level operations)
    pub fn write_byte(&mut self, index: usize, value: u8) {
        let mut word = self.read_word(index);
        word[index % WORD_SIZE] = value;
        self.write_word(index, word);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_operations() {
        let mut mem = Memory::new();
        
        let test_word = [1u8; WORD_SIZE];
        mem.write_word(64, test_word);
        
        assert_eq!(mem.read_word(64), test_word);
        assert_eq!(mem.read_word(65), test_word); // Should read the same word
        
        assert_eq!(mem.size(), 96); // 64 + 32
    }

    #[test]
    fn test_byte_operations() {
        let mut mem = Memory::new();
        
        mem.write_byte(100, 42);
        assert_eq!(mem.read_byte(100), 42);
        
        // Check that it's in the correct position within the word
        let word = mem.read_word(96); // 96 is the word-aligned address for byte 100
        assert_eq!(word[4], 42);
    }

    #[test]
    fn test_expansion() {
        let mut mem = Memory::new();
        
        mem.expand(CHUNK_SIZE + 100);
        assert_eq!(mem.chunks.len(), 2);
        
        mem.write_word(CHUNK_SIZE, [5u8; WORD_SIZE]);
        assert_eq!(mem.read_word(CHUNK_SIZE), [5u8; WORD_SIZE]);
    }
}