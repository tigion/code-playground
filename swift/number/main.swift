/// Reads and returns a valid integer guess from the user.
func readGuess() -> Int {
  while true {
    print("Your guess: ", terminator: "")

    // Check if the read and the conversion to Int was successful.
    guard let input = readLine(), let guess = Int(input) else {
      print("Invalid input!")
      continue
    }

    return guess

    // // if version:
    // if let input = readLine(), let guess = Int(input) {
    //   return guess
    // }
    //
    // print("Invalid input!")
  }
}

/// The entry point of the program.
func main() {
  let minNumber = 1  // The minimum number.
  let maxNumber = 100  // The maximum number.
  var attempts = 0  // The number of attempts.

  // Generate a random number between the minimum and maximum.
  let secretNumber = Int.random(in: minNumber...maxNumber)

  print("Guess my number between \(minNumber) and \(maxNumber)!\n")

  // The main loop.
  while true {
    // Read the guess from the user.
    let guess = readGuess()

    // Increase the number of attempts.
    attempts += 1

    // Check if the guess is too low, too high or correct.
    if guess < secretNumber {
      print("Too low!")
    } else if guess > secretNumber {
      print("Too high!")
    } else {
      print("\nYou win, the number was \(secretNumber)!")
      print("You had \(attempts) attempts.")
      break
    }
  }
}

main()
