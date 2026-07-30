const ASCII_FIRST: u8 = 0;
const ASCII_LAST: u8 = 127;

/// The number of columns to use for the ASCII table.
/// Use 1, 2, 4, or 8 columns for an optimal layout.
const COL_COUNT: usize = 4;

/// The number of ASCII codes to show in the table.
const CODE_COUNT: usize = (ASCII_LAST - ASCII_FIRST) as usize + 1;

/// The number of rows needed to fit all codes into the columns.
const ROW_COUNT: usize = CODE_COUNT.div_ceil(COL_COUNT);

/// Returns a printable character for the given ASCII code.
/// Non-printable characters are replaced with a dot.
fn printable(code: u8) -> char {
    if code.is_ascii_graphic() || code == b' ' {
        char::from(code)
    } else {
        '.'
    }
}

fn main() {
    // Print the header of the table based on the number of columns.
    println!("{}", ["Dec Hex Chr"; COL_COUNT].join(" | "));
    println!("{}", ["-----------"; COL_COUNT].join("-+-"));

    for row in 0..ROW_COUNT {
        for col in 0..COL_COUNT {
            // Add separators between columns.
            if col > 0 {
                print!(" | ");
            }

            // Calculate the index of the ASCII code.
            let index = row + col * ROW_COUNT;

            // Stop when the ASCII code exceeds the end of the range.
            if index >= CODE_COUNT {
                break;
            }

            // Calculate the ASCII code from the index.
            let code = ASCII_FIRST + index as u8;

            // Print the ASCII code in decimal, hexadecimal and as a character.
            print!("{code:>3} {code:3X}  {} ", printable(code));
        }
        // Newline after each row.
        println!();
    }
}
