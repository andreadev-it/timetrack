# Timetrack
This project is a time tracking utility heavily inspired by
[timetrap](https://github.com/samg/timetrap), but written in Rust,
to improve performance.

Timetrack does not aim to be a full timetrap clone, but includes
the most used commands and both a user-friendly output
and a JSON output that can be piped into other commands or
applications.

## Installation
Currently timetrack is only available through git, but it will
soon be published to crates.io.

To install it, you can use the following command:
```sh
cargo install --git https://github.com/andreadev-it/timetrack
```

After installing, the `t` command will be available.

You can then run `t help` to check all the available commands.
Then, for every comand, you can ask for more information like
this: `t display --help`.

## Usage
Timetrack is based on the idea of timesheets, which can be
started and stopped, with additional notes that can be passed
every time you start the timer. You can also think about it 
as lists of tasks.

You will start in the "default" sheet. To check into the
sheet (start the timer) run this command:
```sh
t in first-task
```

The "first-task" note is optional, but it's very useful.

When you're done with your work, you can check out of
the timesheet by running:
```sh
t out
```

Both the `in` and `out` command accept a `--at` parameter
where you can specify when the task was started or has been 
finished. It accepts values like "5 minutes ago", "yesterday
at 10:30" or "12pm".

When you've checked in and out, if you want to see the
current situation, you can run the following command:
```sh
t display
```

To change the active timesheet you can run:
```sh
t sheet new-sheet
```

Then you can run the following command to list all
available sheets:
```sh
t list
```

Remember also that every command can be shortened
to its first non-ambiguous letter. For example, 
`t sheet` becomes `t s`, `t out` becomes `t o`, etc.

## Next steps
- [x] Add the "month" command as an alias for display
- [x] Check for edge cases (e.g. all sheets removed)
- [x] Improve code quality
- [ ] Add errors contexts
- [ ] Add tests
- [ ] Write docs
- [ ] Publish on crates.io

