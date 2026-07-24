# Notes Zig

## Input/Output in Console

### stdin

```zig
const std = @import("std");

pub fn main(init: std.process.Init) !void {
    const io = init.io; // Get the I/O context.

    // Create stdin reader to read input from the console.
    var stdin_buffer: [1024]u8 = undefined; // Buffer used internally by the stdin reader.
    const stdin = std.Io.File.stdin(); // Get a handle to standard input.
    var stdin_reader = stdin.reader(io, &stdin_buffer); // Create a buffered reader for stdin.
    const reader = &stdin_reader.interface; // Use the reader interface for reading lines.

    // Read one line up to the newline character.
    // `takeDelimiter()` can fail and can also return no line (null at EOF).
    // `try` handles errors, and `orelse` handles the optional null case.
    const input = (try reader.takeDelimiter('\n')) orelse return error.EndOfStream;
}
```

### stdout

```zig
const std = @import("std");

pub fn main(init: std.process.Init) !void {
    const io = init.io; // Get the I/O context.

    // Create stdout writer to write output to the console.
    var stdout_buffer: [1024]u8 = undefined; // Buffer used internally by the stdout writer.
    const stdout = std.Io.File.stdout(); // Get a handle to standard output.
    var stdout_writer = stdout.writer(io, &stdout_buffer); // Create a buffered writer for stdout.
    const writer = &stdout_writer.interface; // Use the writer interface for writing output.

    // Write a string to the console.
    try writer.print("Hello, World!\n", .{});

    // Write a formatted string to the console.
    const text = "Lorem ipsum";
    const number = 42;
    try writer.print("Output: {} and {d}\n", .{text, number});

    // Flush the writer to ensure all buffered output is written to the console.
    try writer.flush();
}
```

```zig
const std = @import("std");

pub fn main(init: std.process.Init) !void {
    const io = init.io; // Get the I/O context.

    const stdout = std.Io.File.stdout();
    try stdout.writeStreamingAll(io, "Hello, World!\n");
}
```

### stderr

```zig
const std = @import("std");

pub fn main() !void {
    const a: i32 = 11;
    const b: i32 = 22;

    std.debug.print("a: {d} and b: {d}\n", .{a, b});

    std.log.err("a: {d}\n", .{a});
    std.log.warn("a: {d}\n", .{a});
    std.log.info("a: {d}\n", .{a});
    std.log.debug("a: {d} and b: {d}\n", .{a, b});
}
```
