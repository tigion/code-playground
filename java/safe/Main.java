
// Imprts from built-in packages.
import java.util.Scanner;

/**
 * The main class of the program.
 */
public class Main {

  /**
   * The entry point of the program.
   *
   * It contains the main loop for the functionality.
   *
   * @param args The command line arguments.
   */
  public static void main(String[] args) {
    int codeLength = 5; // The length of the code.
    int attempts = 6; // The number of attempts.
    String code = generateCode(codeLength); // The secret code.
    String input; // The input.

    // Creates a scanner object to read input from the console.
    Scanner scanner = new Scanner(System.in);

    System.out.println("Can you open the safe?");
    System.out.println("The code is " + codeLength + " digits long.\n");

    // The main loop.
    while (attempts > 0) {
      // Asks for the input.
      System.out.printf(" %d > ", attempts);
      input = scanner.nextLine();
      // Limits the input to the code length.
      if (input.length() > codeLength)
        input = input.substring(0, codeLength);
      // Prints the result.
      System.out.println("     " + compareResult(codeLength, code, input));
      // Reduces the number of attempts by one.
      attempts--;

      // Checks if the code is correct or the number of attempts is zero.
      if (input.equals(code)) {
        // The code is correct.
        System.out.println("\nYou got it, the safe is open!");
        break;
      } else if (attempts == 0) {
        // The number of attempts is zero.
        System.out.println("\nYou failed, the code was '" + code + "'.");
      }
    }

    // Closes the scanner object.
    scanner.close();
  }

  /**
   * Generates a random numerical code with the specified length.
   *
   * @param length The length of the code.
   * @return The generated code.
   */
  private static String generateCode(int length) {
    // Checks and sets a valid minimum length.
    if (length < 1)
      length = 1;

    // Fills the code with random digits from 0 to 9.
    String code = "";
    for (int i = 0; i < length; i++)
      code += (int) (Math.random() * 10);

    return code;
  }

  /**
   * Compares the input with the code.
   *
   * - `*` if the digit is wrong and not in the code.
   * - `?` if the digit is in the wrong position.
   * - `0` to `9` if the correct digit is in the right position.
   *
   * @param length The length of the code.
   * @param code   The code.
   * @param input  The input.
   * @return The result of the comparison.
   */
  private static String compareResult(int length, String code, String input) {
    // Creates a new string builder and fills result with `*`.
    // A string builder is a mutable string. Strings are immutable in java.
    StringBuilder result = new StringBuilder("*".repeat(length));

    // Sets the minimum length if the input is shorter.
    if (input.length() < length)
      length = input.length();

    // Fills the result with `?` or the correct digit.
    for (int i = 0; i < length; i++) {
      // Checks if a digit is correct.
      if (input.charAt(i) == code.charAt(i)) {
        result.setCharAt(i, code.charAt(i));
      } else {
        // Checks if a digit is in the wrong position.
        for (int j = 0; j < length; j++) {
          if (input.charAt(i) == code.charAt(j)) {
            result.setCharAt(i, '?');
            break;
          }
        }
      }
    }

    // Converts the string builder to a string and returns it.
    return String.valueOf(result);
  }
}
