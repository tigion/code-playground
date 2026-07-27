# Reads and returns a valid integer guess from the user.
def read_guess
  loop do
    print "Your guess: "
    input = gets

    # Exit cleanly if input stream is closed (e.g., Ctrl + D).
    exit(0) if input.nil?

    # Strip whitespace and newline characters from the input.
    text = input.strip

    # Integer() raises ArgumentError when the input is not a valid integer.
    return Integer(text)
  rescue ArgumentError
    puts "Invalid input!"
  end
end

# Entry point of the program.
def main
  min_number = 1
  max_number = 100
  attempts = 0

  # Generate a random number between the minimum and maximum.
  secret_number = rand(min_number..max_number)

  puts "Guess my number between #{min_number} and #{max_number}!"
  puts

  # The main loop.
  loop do
    # Read the guess from the user.
    guess = read_guess

    # Increase the number of attempts.
    attempts += 1

    # Check if the guess is too low, too high or correct.
    if guess < secret_number
      puts "Too low!"
    elsif guess > secret_number
      puts "Too high!"
    else
      puts
      puts "You win, the number was #{secret_number}!"
      puts "You had #{attempts} attempts."
      break
    end
  end
end

# Call the main function to start the program.
main
