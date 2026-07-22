--- Reads and returns a valid integer guess from the user.
---@return integer
local function read_guess()
  while true do
    io.write('Your guess: ')
    -- Converts the input to an integer (base 10).
    local guess = tonumber(io.read(), 10)
    if guess then return guess end
    print('Invalid input!')
  end
end

local min_number = 1 -- The minimum number.
local max_number = 100 -- The maximum number.
local attempts = 0 -- The number of attempts.

-- Generate a random number between the minimum and maximum.
local secret_number = math.random(min_number, max_number)

-- Print the intro message.
print('Guess my number between ' .. min_number .. ' and ' .. max_number .. '!\n')

-- The main loop.
while true do
  -- Read the guess from the user.
  local guess = read_guess()

  -- Increase the number of attempts.
  attempts = attempts + 1

  -- Check if the guess is too low, too high or correct.
  if guess < secret_number then
    print('Too low!')
  elseif guess > secret_number then
    print('Too high!')
  else
    print('\nYou win, the number was ' .. secret_number .. '!')
    print('You had ' .. attempts .. ' attempts.')
    break
  end
end
