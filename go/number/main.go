// The `main` package defines an executable program.
// Execution starts in the `main()` function.
package main

// Import the necessary packages.
import (
	"bufio"
	"fmt"
	"math/rand/v2"
	"os"
	"strconv"
	"strings"
)

// Reads a valid integer guess from the user.
func readGuess(reader *bufio.Reader) int {
	for {
		fmt.Print("Your guess: ")

		// Read one line up to the newline character.
		// If reading fails, tell the user and ask again.
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("\nRead error:", err)
			continue
		}

		// Remove leading and trailing whitespace.
		input = strings.TrimSpace(input)

		// Try to convert the input into an integer.
		// If parsing fails, tell the user and ask again.
		guess, err := strconv.Atoi(input)
		if err != nil {
			fmt.Println("Invalid input!")
			continue
		}

		// Return the successfully parsed number.
		return guess
	}
}

// The entry point of the program.
func main() {
	// const minNumber = 1   // The minimum number.
	// const maxNumber = 100 // The maximum number.
	const (
		minNumber = 1   // The minimum number.
		maxNumber = 100 // The maximum number.
	)
	attempts := 0 // The number of attempts.

	// Generate a random number between the minimum and maximum.
	rangeSize := maxNumber - minNumber + 1
	secretNumber := rand.IntN(rangeSize) + minNumber

	// Create a buffered reader to read input from the user.
	reader := bufio.NewReader(os.Stdin)

	// Print the intro to the console.
	fmt.Printf("Guess my number between %d and %d!\n\n", minNumber, maxNumber)

	// The main loop.
	for {
		// Read the guess from the user.
		guess := readGuess(reader)

		// Increase the number of attempts.
		attempts++

		// Check if the guess is too low, too high or correct.
		if guess < secretNumber {
			fmt.Println("Too low!")
		} else if guess > secretNumber {
			fmt.Println("Too high!")
		} else {
			fmt.Printf("\nYou win, the number was %d!\n", secretNumber)
			fmt.Printf("You had %d attempts.\n", attempts)
			break
		}
	}
}
