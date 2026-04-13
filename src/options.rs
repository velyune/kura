#[derive(Clone, Copy, Debug)]
pub enum SyncMode {
    Always,
    Manual,
}

#[derive(Clone, Debug)]
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
