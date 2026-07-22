#!/usr/bin/env bash

# Checks if the given argument `$1` is an integer.
# Returns the exit code `0` if it is, `1` otherwise.
is_integer() {
  [[ "$1" =~ ^[0-9]+$ ]]
}

# Reads and returns a valid integer guess from the user.
read_guess() {
  local input

  while true; do
    read -r -p "Your guess: " input
    if is_integer "$input"; then
      echo "$input"
      return 0
    fi
    echo "Invalid input!" >&2
  done
}

min_number=1   # The minimum number.
max_number=100 # The maximum number.
attempts=0     # The number of attempts.

# Generate a random number between the minimum and maximum.
secret_number=$((RANDOM % (max_number - min_number + 1) + min_number))

echo "Guess my number between ${min_number} and ${max_number}!"
echo
# printf "Guess my number between %d and %d!\n\n" "$min_number" "$max_number"

# The main loop.
while true; do
  # Read the guess from the user.
  guess=$(read_guess)

  # Increase the number of attempts.
  attempts=$((attempts + 1))

  # Check if the guess is too low, too high or correct.
  if [[ $guess -lt $secret_number ]]; then
    echo "Too low!"
  elif [[ $guess -gt $secret_number ]]; then
    echo "Too high!"
  else
    echo
    echo "You win, the number was ${secret_number}!"
    echo "You had ${attempts} attempts."
    break
  fi
done
