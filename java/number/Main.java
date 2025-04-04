
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
    int minNumber = 1; // The minimum number.
    int maxNumber = 100; // The maximum number.
    int attempts = 0; // The number of attempts.
    String input; // The input.
    int guess = 0; // The guessed number.

    // Generates a random number between the minimum and maximum.
    int secretNumber = (int) (Math.random() * (maxNumber - minNumber + 1) + minNumber);

    // Creates a scanner object to read input from the console.
    Scanner scanner = new Scanner(System.in);

    System.out.println("Guess my number between " + minNumber + " and " + maxNumber + "!\n");

    // The main loop.
    while (true) {
      // Increases the number of attempts.
      attempts++;

      // Asks for the input.
      System.out.printf("Your guess: ");
      input = scanner.nextLine();

      // Checks if it is a valid number.
      try {
        guess = Integer.parseInt(input);
      } catch (NumberFormatException e) {
        System.out.println("Invalid input!");
        continue;
      }

      // Checks if the guess is correct, to low or to high.
      if (guess == secretNumber) {
        // The guess is correct.
        System.out.println("\nYou win, the number was " + secretNumber + "!");
        System.out.println("You had " + attempts + " attempts.");
        break;
      } else if (guess < secretNumber) {
        // The guess is too low.
        System.out.println("Too low!");
      } else {
        // The guess is too high.
        System.out.println("Too high!");
      }
    }

    // Closes the scanner object.
    scanner.close();
  }
}
