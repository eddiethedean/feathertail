use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

// String interning for memory efficiency
#[derive(Clone)]
pub struct StringPool {
    strings: Arc<RwLock<HashMap<String, u32>>>,
    reverse: Arc<RwLock<HashMap<u32, String>>>,
    next_id: Arc<RwLock<u32>>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            strings: Arc::new(RwLock::new(HashMap::new())),
            reverse: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
        }
    }

    pub fn intern(&self, s: &str) -> u32 {
        // Check if string already exists
        {
            let strings = self.strings.read().unwrap();
            if let Some(&id) = strings.get(s) {
                return id;
            }
        }

        // String doesn't exist, add it
        let mut strings = self.strings.write().unwrap();
        let mut reverse = self.reverse.write().unwrap();
        let mut next_id = self.next_id.write().unwrap();

        // Double-check in case another thread added it
        if let Some(&id) = strings.get(s) {
            return id;
        }

        let id = *next_id;
        *next_id += 1;
        strings.insert(s.to_string(), id);
        reverse.insert(id, s.to_string());
        id
    }

    pub fn get(&self, id: u32) -> Option<String> {
        let reverse = self.reverse.read().unwrap();
        reverse.get(&id).cloned()
    }

    pub fn len(&self) -> usize {
        let strings = self.strings.read().unwrap();
        strings.len()
    }

    pub fn memory_usage(&self) -> usize {
        let strings = self.strings.read().unwrap();
        let reverse = self.reverse.read().unwrap();
        
        // Estimate memory usage
        let string_memory: usize = strings.keys().map(|s| s.len()).sum();
        let id_memory = strings.len() * 4; // u32 = 4 bytes
        let reverse_memory: usize = reverse.values().map(|s| s.len()).sum();
        
        string_memory + id_memory + reverse_memory
    }
}

// Optimized string column using interning
#[derive(Clone)]
pub struct OptimizedStrColumn {
    ids: Vec<u32>,
    pool: StringPool,
}

impl OptimizedStrColumn {
    pub fn new(pool: StringPool) -> Self {
        Self {
            ids: Vec::new(),
            pool,
        }
    }

    pub fn push(&mut self, s: &str) {
        let id = self.pool.intern(s);
        self.ids.push(id);
    }

    pub fn get(&self, index: usize) -> Option<String> {
        self.ids.get(index).and_then(|&id| self.pool.get(id))
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn memory_usage(&self) -> usize {
        // IDs memory + pool memory
        self.ids.len() * 4 + self.pool.memory_usage()
    }

    pub fn iter(&self) -> OptimizedStrColumnIter {
        OptimizedStrColumnIter {
            column: self,
            index: 0,
        }
    }

    pub fn to_regular_strings(&self) -> Vec<String> {
        self.ids.iter()
            .filter_map(|&id| self.pool.get(id))
            .collect()
    }
}

pub struct OptimizedStrColumnIter<'a> {
    column: &'a OptimizedStrColumn,
    index: usize,
}

impl<'a> Iterator for OptimizedStrColumnIter<'a> {
    type Item = Option<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.column.len() {
            return None;
        }

        let id = self.column.ids[self.index];
        self.index += 1;
        Some(self.column.pool.get(id))
    }
}

// String deduplication for regular string columns
pub struct StringDeduplicator {
    pool: StringPool,
}

impl StringDeduplicator {
    pub fn new() -> Self {
        Self {
            pool: StringPool::new(),
        }
    }

    pub fn deduplicate_strings(&self, strings: &[String]) -> Vec<u32> {
        strings.iter()
            .map(|s| self.pool.intern(s))
            .collect()
    }

    pub fn restore_strings(&self, ids: &[u32]) -> Vec<String> {
        ids.iter()
            .filter_map(|&id| self.pool.get(id))
            .collect()
    }

    pub fn memory_savings(&self, original_strings: &[String]) -> usize {
        let original_memory: usize = original_strings.iter().map(|s| s.len()).sum();
        let deduplicated_memory = self.pool.memory_usage();
        
        if original_memory > deduplicated_memory {
            original_memory - deduplicated_memory
        } else {
            0
        }
    }
}

// String compression utilities
pub struct StringCompressor {
    // Simple dictionary-based compression
    dictionary: HashMap<String, u16>,
    reverse_dict: HashMap<u16, String>,
    next_code: u16,
}

impl StringCompressor {
    pub fn new() -> Self {
        Self {
            dictionary: HashMap::new(),
            reverse_dict: HashMap::new(),
            next_code: 0,
        }
    }

    pub fn compress_strings(&mut self, strings: &[String]) -> Vec<u16> {
        strings.iter()
            .map(|s| self.compress_string(s))
            .collect()
    }

    fn compress_string(&mut self, s: &str) -> u16 {
        if let Some(&code) = self.dictionary.get(s) {
            code
        } else {
            let code = self.next_code;
            self.next_code += 1;
            self.dictionary.insert(s.to_string(), code);
            self.reverse_dict.insert(code, s.to_string());
            code
        }
    }

    pub fn decompress_strings(&self, codes: &[u16]) -> Vec<String> {
        codes.iter()
            .filter_map(|&code| self.reverse_dict.get(&code).cloned())
            .collect()
    }
}

// Memory-efficient string operations
pub struct StringOps {
    pool: StringPool,
}

impl StringOps {
    pub fn new() -> Self {
        Self {
            pool: StringPool::new(),
        }
    }

    pub fn upper_case(&self, s: &str) -> String {
        s.to_uppercase()
    }

    pub fn lower_case(&self, s: &str) -> String {
        s.to_lowercase()
    }

    pub fn contains(&self, haystack: &str, needle: &str) -> bool {
        haystack.contains(needle)
    }

    pub fn starts_with(&self, s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }

    pub fn ends_with(&self, s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }

    pub fn strip(&self, s: &str) -> String {
        s.trim().to_string()
    }

    pub fn replace(&self, s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }

    // Batch operations for efficiency
    pub fn batch_upper_case(&self, strings: &[String]) -> Vec<String> {
        strings.iter()
            .map(|s| self.upper_case(s))
            .collect()
    }

    pub fn batch_lower_case(&self, strings: &[String]) -> Vec<String> {
        strings.iter()
            .map(|s| self.lower_case(s))
            .collect()
    }

    pub fn batch_contains(&self, strings: &[String], needle: &str) -> Vec<bool> {
        strings.iter()
            .map(|s| self.contains(s, needle))
            .collect()
    }
}
