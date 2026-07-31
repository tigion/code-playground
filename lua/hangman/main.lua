-- Seed the random number generator with the current time
-- so random choices differ between program runs.
-- This is usually done in the main file of a program and
-- before any random numbers are generated.
math.randomseed(os.time())

-- Import required modules.
local WordList = require('wordlist')
local Hangman = require('hangman')

-- Pick a random word from the word list or exit if the list is empty.
local word = WordList.random()
if word == nil then
  print('The word list is empty!')
  return
end

-- Initialize a new Hangman game.
local game, err = Hangman:new(word)
if game == nil then
  print('Failed to initialize the game: ' .. err)
  return
end

print('Welcome to Hangman!')
print('\nFind a word with ' .. game:word_length() .. ' letters.')

-- The main loop.
while true do
  -- Prints the current state of the round.
  print('\n' .. game:display_word() .. '\n')
  local current_lives, max_lives = game:get_lives()
  print(current_lives .. '/' .. max_lives .. ' lives')
  print('Used: ' .. game:display_used_letters())

  -- Checks both end conditions before asking for the next letter.
  if game:is_word_complete() then
    print("\nYou won!\nYou guessed '" .. game:word() .. "' correctly.")
    break
  elseif game:is_player_dead() then
    print("\nYou lost!\nThe word was '" .. game:word() .. "'.")
    break
  end

  -- Asks for a letter and ends the game when the input is over.
  io.write('Which letter is in the word? ')
  local input = io.read()
  if input == nil then break end

  -- React to the guess result and print the corresponding message.
  local result = game:guess(input)
  if result == game.GuessResult.CORRECT then
    print('> Correct :)')
  elseif result == game.GuessResult.WRONG then
    print('> Wrong, you lose a life :(')
  elseif result == game.GuessResult.DUPLICATE then
    print('> Careful, already used!')
  elseif result == game.GuessResult.INVALID then
    print('> Please use only one letter from A to Z!')
  end
end
