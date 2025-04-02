#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

char *generateCode(int len) {
  srand(time(NULL));
  char *secret = malloc(len + 1);
  for (int i = 0; i < len; i++) secret[i] = rand() % 10 + '0';
  // secret[len] = 0;
  return secret;
}

char *compareResult(int len, char *secret, char *input) {
  char *result = malloc(len + 1);
  for (int i = 0; i < len; i++) {
    result[i] = '*';

    if (input[i] == secret[i]) result[i] = secret[i];
    else {
      for (int j = 0; j < len; j++) {
        if (input[i] == secret[j]) {
          result[i] = '?';
          break;
        }
      }
    }
  }

  return result;
}

int main(int argc, char *argv[]) {
  int codeLength = 5;
  int attempts = 6;
  char *code = generateCode(codeLength);
  char input[256];

  puts("Can you open the safe?");
  printf("The code is %d digits long.\n\n", codeLength);

  while (1) {
    printf(" %02d > ", attempts);
    fgets(input, 256, stdin);
    input[codeLength] = 0;
    printf("      %s\n", compareResult(codeLength, code, input));
    attempts--;

    if (strcmp(input, code) == 0) {
      puts("\nYou got it, the safe is open!");
      break;
    } else if (attempts == 0) {
      printf("\nYou failed, the code was '%s'.\n", code);
      break;
    };
  }

  return 0;
}
