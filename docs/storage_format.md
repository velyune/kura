# Storage Format

## Contents

- [Directory Layout and File Naming](#directory-layout-and-file-naming)
- [File Identification and Format Versions](#file-identification-and-format-versions)
- [Internal Keys](#internal-keys)

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
