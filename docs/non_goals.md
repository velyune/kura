# Non-Goals

The items below are explicitly out of scope for Kura unless the project charter changes.

## Do Not Build

| Non-goal                       | Why It Is Excluded                                                                                                       |
|--------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| Sharding                       | Kura is a single-node engine and should not split data across nodes or partitions                                        |
| Replication                    | Cross-node durability would introduce transport, membership, and failure coordination concerns outside the current scope |
| Distributed consensus          | Raft/Paxos-style coordination is unrelated to the embedded storage goal                                                  |
| Cluster management             | Node orchestration, placement, and rolling upgrades belong to a different class of system                                |
| SQL layer                      | Kura is a key-value engine, not a relational database                                                                    |
| MVCC                           | Historical version management would substantially increase read-path, compaction, and metadata complexity                |
| Complex transactions           | Multi-key transactional semantics are not required for the target workload                                               |
| Secondary indexes              | Prefix-based key design is the intended access model                                                                     |
| Admin dashboards               | Operations should remain file- and library-level, not UI-driven                                                          |
| Network server                 | Kura is meant to be embedded directly in Rust applications                                                               |
| Multiple compaction strategies | One well-chosen compaction strategy is enough for the roadmap                                                            |
| Cloud platform features        | Managed service concerns are outside the project’s academic and engineering scope                                        |

## What Kura Is

Kura is:

- a single-node embedded storage engine
- an LSM-tree design with WAL, memtable, SSTables, and compaction
- optimized for small write-heavy state
- focused on point lookups, prefix scans, and crash recovery
