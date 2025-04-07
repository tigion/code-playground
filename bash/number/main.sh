#!/usr/bin/env bash

# Checks if the given argument `$1` is an integer.
# Returns the exit code `0` if it is, `1` otherwise.
function is_integer() {
  [[ "$1" =~ ^[0-9]+$ ]]
}

# The minimum number.
min_number=1
# The maximum number.
max_number=100
# The number of attempts.
attempts=0

# Generates a random number between the minimum and maximum.
secret_number=$((RANDOM % (max_number - min_number + 1) + min_number))

# Prints the intro message.
# echo "Guess my number between ${min_number} and ${max_number}!"
# echo
# echo -e "Guess my number between ${min_number} and ${max_number}!\n"
printf "Guess my number between %d and %d!\n\n" "$min_number" "$max_number"

# The main loop.
while true; do
  # Increases the number of attempts by 1.
  attempts=$((attempts + 1))

  # Asks for the input (without a line break).
  read -r -p "Your guess: " guess

  # Checks if the guess is a valid number, correct, to low or to high.
  if ! is_integer "$guess"; then
    # Is not an integer.
    echo "Invalid input!"
    continue
  elif [[ "$guess" -eq "$secret_number" ]]; then
    # The guess is correct.
    echo
    echo "You win, the number was ${secret_number}!"
    echo "You had ${attempts} attempts."
    # Exits the loop.
    break
  elif [[ "$guess" -lt "$secret_number" ]]; then
    # The guess is too low.
    echo "Too low!"
  elif [[ "$guess" -gt "$secret_number" ]]; then
    # The guess is too high.
    echo "Too high!"
  fi
done
