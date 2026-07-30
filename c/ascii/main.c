#include <ctype.h>
#include <stdio.h>

/**
 * Prints the given text repeated a specified number of times,
 * separated by a given separator. Adds a newline at the end.
 */
static void printRepeated(const char *text, int count, const char *separator) {
  for (int i = 0; i < count; i++) {
    printf("%s%s", i > 0 ? separator : "", text);
  }
  printf("\n");
}

/**
 * Returns a printable character for the given ASCII code.
 * Non-printable characters are replaced with a dot.
 */
static char printable(unsigned char code) {
  return (isgraph(code) || code == ' ') ? code : '.';
  // if (isgraph(code) || code == ' ') {
  //   return code;
  // }
  // return '.';
}

int main(void) {
  const int asciiFirst = 0;  // The first ASCII code to display.
  const int asciiLast = 127; // The last ASCII code to display.
  const int colCount = 4;    // The number of columns in the table.

  // Print the header of the table based on the number of columns.
  printRepeated("Dec Hex Chr", colCount, " | ");
  printRepeated("-----------", colCount, "-+-");

  // The number of ASCII codes to show in the table.
  int codeCount = asciiLast - asciiFirst + 1;
  // The number of rows needed to fit all codes into the columns.
  int rowCount = (codeCount + colCount - 1) / colCount;

  for (int row = 0; row < rowCount; row++) {
    for (int col = 0; col < colCount; col++) {
      // Add separators between columns.
      if (col > 0) printf(" | ");

      // Calculate the index of the ASCII code
      // based on the current row and column.
      int index = row + col * rowCount;

      // Stop when the ASCII code exceeds the end of the range.
      if (index >= codeCount) break;

      // Calculate the ASCII code from the index.
      int code = asciiFirst + index;

      // Print the ASCII code in decimal, hexadecimal and as a character.
      printf("%3d %3X  %c ", code, code, printable(code));
    }
    printf("\n");
  }

  return 0;
}
