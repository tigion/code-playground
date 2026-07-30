#include <ctype.h>
#include <stdio.h>

int main(void) {
  printf("Dec Hex Chr | Dec Hex Chr | Dec Hex Chr | Dec Hex Chr\n");
  printf("------------+-------------+-------------+------------\n");

  const int colCount = 4;
  const int rowCount = 32;

  for (int row = 0; row < rowCount; row++) {
    for (int col = 0; col < colCount; col++) {
      if (col > 0) printf(" | ");

      int code = row + col * rowCount;
      char printableCode = (isgraph(code) || code == ' ') ? code : '.';

      printf("%3d %3X  %c ", code, code, printableCode);
    }
    printf("\n");
  }

  return 0;
}
