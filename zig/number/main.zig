// Import the needed modules.
const std = @import("std");

// Generate a random number between min and max (inclusive).
fn generateRandomNumber(io: std.Io, min: u32, max: u32) u32 {
    // Create a random number generator using the I/O context.
    const source: std.Random.IoSource = .{ .io = io };
    const random = source.interface();
    return random.intRangeAtMost(u32, min, max);
}

// Read a guess from the user and return it as a u32.
fn readGuess(writer: *std.Io.Writer, reader: *std.Io.Reader) !u32 {

    // Keep asking until the user enters a valid number.
    while (true) {
        try writer.print("Your guess: ", .{});
        try writer.flush();

        // Read one line up to the newline character.
        const line = (try reader.takeDelimiter('\n')) orelse return error.EndOfStream;

        // Remove leading and trailing whitespace.
        const trimmed = std.mem.trim(u8, line, " \t\r");

        // Try to convert the input into an unsigned 32-bit integer.
        // If parsing fails, tell the user and ask again.
        const guess = std.fmt.parseInt(u32, trimmed, 10) catch {
            try writer.print("Invalid input!\n", .{});
            continue;
        };

        // Return the successfully parsed number.
        return guess;
    }
}

// The main function of the program.
pub fn main(init: std.process.Init) !void {
    const io = init.io; // Get the I/O context.

    // Create stdout writer to write output to the console.
    var stdout_buffer: [1024]u8 = undefined;
    const stdout = std.Io.File.stdout();
    var stdout_writer = stdout.writer(io, &stdout_buffer);
    const writer = &stdout_writer.interface;

    // Create stdin reader to read input from the console.
    var stdin_buffer: [1024]u8 = undefined;
    const stdin = std.Io.File.stdin();
    var stdin_reader = stdin.reader(io, &stdin_buffer);
    const reader = &stdin_reader.interface;

    const min_number = 1;      // The minimum number.
    const max_number = 100;    // The maximum number.
    var attempts: usize = 0;   // The number of attempts.
    var has_won: bool = false; // Whether the user has won.

    // Generate a random number between the minimum and maximum.
    const secret_number = generateRandomNumber(io, min_number, max_number);

    try writer.print("Guess the number between {d} and {d}\n\n", .{min_number, max_number});
    try writer.flush();

    // The main loop.
    while (true) {
        // Read a guess from the user.
        const guess = try readGuess(writer, reader);

        // Increment the number of attempts.
        attempts += 1;

        // Check if the guess is too low, too high or correct.
        if (guess < secret_number) {
            try writer.print("Too low!\n", .{});
        } else if (guess > secret_number) {
            try writer.print("Too high!\n", .{});
        } else {
            has_won = true;
            try writer.print("\nYou win, the number was {d}!\n", .{secret_number});
            try writer.print("You had {d} attempts.\n", .{attempts});
        }
        try writer.flush();

        if (has_won) {
            break; // Exit the loop if the user has won.
        }
    }
}
