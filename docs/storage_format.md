# Storage Format

## Contents

- [Directory Layout and File Naming](#directory-layout-and-file-naming)
- [File Identification and Format Versions](#file-identification-and-format-versions)
- [Internal Keys](#internal-keys)
- [CURRENT File Format](#current-file-format)
- [Manifest File Format](#manifest-file-format)

## Directory Layout and File Naming

```text
db/
    CURRENT
    LOCK
    MANIFEST-00000000000000000001
    wal/
        00000000000000000002.wal
    sst/
        00000000000000000003.sst
    tmp/
```

---

## File Identification and Format Versions

| File Type | Magic Bytes | Version Field | Format Version |
|-----------|-------------|---------------|----------------|
| MANIFEST  | `KURAMNF`   | file header   | `1`            |
| WAL       | `KURAWAL`   | file header   | `1`            |
| SSTable   | `KURASST`   | footer        | `1`            |

---

## Internal Keys

The logical internal key is:

```text
(user_key, sequence_number, value_type)
```

### Value Types

| Value | Name        |
|-------|-------------|
| `1`   | `VALUE`     |
| `2`   | `TOMBSTONE` |

### Comparison Order

| Priority | Field             | Order                        | Purpose                   |
|----------|-------------------|------------------------------|---------------------------|
| 1        | `user_key`        | byte-lexicographic ascending | Primary key ordering      |
| 2        | `sequence_number` | descending                   | Newer versions sort first |
| 3        | `value_type`      | ascending                    | Deterministic tie-breaker |

---

## CURRENT File Format

`CURRENT` is a UTF-8 text file containing the active manifest filename followed by a
single newline.

Example:

```text
MANIFEST-00000000000000000001\n
```

---

## Manifest File Format

`MANIFEST` is a binary file. All integer fields are encoded in little-endian.

Schematic field order:

```text
{MANIFEST_MAGIC}{MANIFEST_FORMAT_VERSION}{next_file_number}{last_sequence}{wal_file_count}{wal_files[wal_file_count]}{sstable_count}{sstable_files[sstable_count]}
```

### Header

| Field                     | Type      | Meaning                              |
|---------------------------|-----------|--------------------------------------|
| `MANIFEST_MAGIC`          | `[u8; 7]` | Manifest file magic bytes: `KURAMNF` |
| `MANIFEST_FORMAT_VERSION` | `u16`     | Manifest format version              |

### Snapshot

| Field              | Type                | Meaning                      |
|--------------------|---------------------|------------------------------|
| `next_file_number` | `u64`               | Next available file number   |
| `last_sequence`    | `u64`               | Last durable sequence number |
| `wal_file_count`   | `u32`               | Number of live WAL files     |
| `wal_files[i]`     | `u64`               | Live WAL file number         |
| `sstable_count`    | `u32`               | Number of live SSTables      |
| `sstable_files[i]` | `SstableDescriptor` | Live SSTable descriptor      |

### SSTable Descriptor

| Field              | Type                     | Meaning                         |
|--------------------|--------------------------|---------------------------------|
| `file_number`      | `u64`                    | SSTable file number             |
| `min_user_key_len` | `u32`                    | Length of the smallest user key |
| `min_user_key`     | `[u8; min_user_key_len]` | Smallest user key               |
| `max_user_key_len` | `u32`                    | Length of the largest user key  |
| `max_user_key`     | `[u8; max_user_key_len]` | Largest user key                |
| `min_sequence`     | `u64`                    | Smallest sequence number        |
| `max_sequence`     | `u64`                    | Largest sequence number         |
