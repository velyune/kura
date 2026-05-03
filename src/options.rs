#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SyncMode {
    Always,
    Manual,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub sync_mode: SyncMode,
    pub memtable_bytes: usize,
    pub compaction_trigger_tables: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            sync_mode: SyncMode::Always,
            memtable_bytes: 4 * 1024 * 1024,
            compaction_trigger_tables: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_use_documented_values() {
        let options = Options::default();

        assert_eq!(options.sync_mode, SyncMode::Always);
        assert_eq!(options.memtable_bytes, 4 * 1024 * 1024);
        assert_eq!(options.compaction_trigger_tables, 4);
    }
}
