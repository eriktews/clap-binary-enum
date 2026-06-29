# Changelog

## 0.1.2 — 2026-06-29

- The generated `*Arg` struct now also derives `Clone` (in addition to `clap::Args`
  and `Debug`). This is a non-breaking addition since the struct only contains
  `bool` fields.
