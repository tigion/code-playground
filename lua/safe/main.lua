---Generates a random numerical code with the specified length.
---@param length integer The length of the code.
---@return string # The generated code.
local function generate_code(length)
  -- Checks and sets a valid minimum length.
  if length < 1 then length = 1 end

  -- Fills the code with random digits from 0 to 9.
  local code = ''
  for _ = 1, length do
    code = code .. math.random(0, 9)
  end

  return code
end

---Compares the input with the code.
---
---- `*` if the digit is wrong and not in the code.
---- `?` if the digit is in the wrong position.
---- `0` to `9` if the correct digit is in the right position.
---
---@param length integer The length of the code.
---@param code string The code.
---@param input string The input.
---@return string # The result of the comparison.
local function compare_result(length, code, input)
  -- Fills the result character array with `*`.
  local result_chars = {}
  for i = 1, length do
    result_chars[i] = '*'
  end

  -- Sets the minimum length if the input is shorter.
  -- length = math.min(#code, #input)
  if #input < length then length = #input end

  -- Fills the result with `?` or the correct digit.
  for i = 1, length do
    -- Checks if a digit is correct.
    if input:sub(i, i) == code:sub(i, i) then
      result_chars[i] = code:sub(i, i)
    else
      for j = 1, length do
        -- Checks if a digit is in the wrong position.
        if input:sub(i, i) == code:sub(j, j) then
          result_chars[i] = '?'
          break
        end
      end
    end
  end

  -- Joins the result character array into a string.
  return table.concat(result_chars)
end

local code_length = 5 -- The length of the code.
local attempts = 6 -- The number of attempts.
local code = generate_code(code_length) -- The secret code.
local input = '' -- The input.

-- Prints the intro message.
print('Can you open the safe?')
print('The code is ' .. code_length .. ' digits long.\n')

-- The main loop.
while attempts > 0 do
  -- Asks for the input.
  io.write(' ' .. attempts .. ' > ')
  input = io.read()
  -- Limits the input to the code length.
  input = input:sub(1, code_length)
  -- Prints the result.
  print('     ' .. compare_result(code_length, code, input))
  -- Reduces the number of attempts by 1.
  attempts = attempts - 1

  -- Checks if the code is correct or the number of attempts is zero.
  if input == code then
    -- The code is correct.
    print('\nYou got it, the safe is open!')
    break
  elseif attempts == 0 then
    -- The number of attempts is zero.
    print("\nYou failed, the code was '" .. code .. "'.")
  end
end
