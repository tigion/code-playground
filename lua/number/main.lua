local min_number = 1 -- The minimum number.
local max_number = 100 -- The maximum number.
local attempts = 0 -- The number of attempts.

-- Generates a random number between the minimum and maximum.
local secret_number = math.random(min_number, max_number)

-- Prints the intro message.
print('Guess my number between ' .. min_number .. ' and ' .. max_number .. '!\n')

-- The main loop.
while true do
  -- Increases the number of attempts by 1.
  attempts = attempts + 1

  -- Asks for the input (without a line break).
  io.write('Your guess: ')
  -- Converts the input to an integer (base 10).
  local guess = tonumber(io.read(), 10)

  -- Checks if the guess is a valid number, correct, to low or to high.
  if not guess then
    -- Is not a valid number.
    print('Invalid input!')
  elseif guess == secret_number then
    -- The guess is correct.
    print('\nYou win, the number was ' .. secret_number .. '!')
    print('You had ' .. attempts .. ' attempts.')
    -- Exits the loop.
    break
  elseif guess < secret_number then
    -- The guess is too low.
    print('Too low!')
  elseif guess > secret_number then
    -- The guess is too high.
    print('Too high!')
  end
end
