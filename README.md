# Timtrack
**! This project has just started and it's not yet usable !**

This project is a time tracking utility heavily inspired by
[timetrap](https://github.com/samg/timetrap), but written in Rust,
to improve performance.

Timetrack does not aim to be a full timetrap clone, but will
include the most used commands and both a user-friendly output
and a JSON output that can be piped into other commands or
applications.

## Things that work, but might be incomplete
- [x] `t in [--at] [TASK]`
- [x] `t out [--at]`
- [x] `t current`
- [x] `t display [--json] [--start] [--end]`
- [x] `t sheet [name]`
- [x] `t list`
- [x] `t edit [--id] [--start] [--end] [--move] [NOTES]`
- [x] `t kill [--id] [TIMESHEET]`
