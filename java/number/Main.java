
// Import from built-in packages.
import java.util.Scanner;

/**
 * The main class of the program.
 */
public class Main {
  // Create a scanner object to read input from the console.
  static Scanner scanner = new Scanner(System.in);

  /**
   * Reads the guess from the user.
   *
   * @return The guess as an integer.
   */
  static int readGuess() {
    while (true) {
      System.out.print("Your guess: ");
      String input = scanner.nextLine();

      try {
        return Integer.parseInt(input);
      } catch (NumberFormatException e) {
        System.out.println("Invalid input!");
      }
    }
  }

  /**
   * The entry point of the program.
   *
   * @param args The command line arguments.
   */
  public static void main(String[] args) {
    int minNumber = 1; // The minimum number.
    int maxNumber = 100; // The maximum number.
    int attempts = 0; // The number of attempts.

    // Generate a random number between the minimum and maximum.
    int secretNumber = (int) (Math.random() * (maxNumber - minNumber + 1) + minNumber);

    System.out.println("Guess my number between " + minNumber + " and " + maxNumber + "!\n");

    // The main loop.
    while (true) {
      // Read the guess from the user.
      int guess = readGuess();

      // Increase the number of attempts.
      attempts++;

      // Check if the guess is too low, too high or correct.
      if (guess < secretNumber) {
        System.out.println("Too low!");
      } else if (guess > secretNumber) {
        System.out.println("Too high!");
      } else {
        System.out.println("\nYou win, the number was " + secretNumber + "!");
        System.out.println("You had " + attempts + " attempts.");
        break;
      }
    }
  }
}
