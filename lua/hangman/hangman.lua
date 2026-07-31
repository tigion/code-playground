--- The number of life points a player starts with.
local MAX_LIVES = 10

--- Results returned by `Hangman:guess()`.
---@enum GuessResult
local GuessResult = {
  --- The letter occurs in the word and at least one position was revealed.
  CORRECT = 1,
  --- The letter does not occur in the word and one life is lost.
  WRONG = 2,
  --- The letter was already guessed before, so nothing happens.
  DUPLICATE = 3,
  --- The input was invalid, e.g. more than one character or not a valid letter.
  INVALID = 4,
}

--- Converts a string to upper case and turns 'ß' into 'SS'.
--- Lua's `string.upper` does not convert 'ß' like Python or Rust do.
---@param str string
---@return string
local function to_upper(str)
  str = str:gsub('ß', 'SS')
  return string.upper(str)
end

--- Converts user input into a valid uppercase letter for the game.
--- Whitespace is removed and invalid input returns `nil`.
---@param input string The user input.
---@return string? letter A single uppercase letter from A to Z or `nil` if invalid.
local function normalize_letter(input)
  -- Remove surrounding whitespace.
  input = input:match('^%s*(.-)%s*$')

  -- The input must contain exactly one UTF-8 character.
  -- For example, `#'ä'` returns 2 because it counts bytes,
  -- while `utf8.len('ä')` returns 1 because it counts characters.
  if utf8.len(input) ~= 1 then return nil end

  -- The German eszett 'ß' is converted to 'SS' when uppercased.
  -- It is treated as a single accepted input and mapped to 'S'.
  if input == 'ß' then return 'S' end

  -- Accept letters A-Z in any case and return them uppercase.
  if input:match('^[a-zA-Z]$') then return string.upper(input) end

  -- Reject all other characters.
  return nil
end

--- A single letter of the word and its reveal state.
---@class HangmanLetter
---@field char string The character.
---@field revealed boolean Whether the character is visible.

--- Represents a Hangman game instance.
---@class Hangman
---@field _letters HangmanLetter[] The letters of the word to guess.
---@field _used_letters table<string, boolean> Letters that have already been guessed.
---@field _lives integer Remaining lives.
local Hangman = {}
Hangman.__index = Hangman

--- Creates a new Hangman game with the given word.
--- Returns `nil` if the word is empty or not provided.
---@param word string The word to guess.
---@return Hangman? game The new Hangman instance or `nil` if the word is invalid.
---@return string? err Error message if creation failed.
function Hangman:new(word)
  if not word or word == '' then return nil, 'Word must not be empty.' end

  word = to_upper(word)

  local obj = setmetatable({}, self)

  obj._letters = {}

  -- Lua strings are byte sequences, so iterating with `gmatch('.')`
  -- does not handle UTF-8 characters correctly. `utf8.codes()` iterates
  -- over Unicode code points, allowing each character to be stored separately.
  for _, codepoint in utf8.codes(word) do
    local char = utf8.char(codepoint)

    table.insert(obj._letters, {
      char = char,
      revealed = not char:match('^[A-Z]$'),
    })
  end

  obj._used_letters = {}
  obj._lives = MAX_LIVES

  return obj
end

--- Returns the secret word.
---@return string word The secret word.
function Hangman:word()
  local chars = {}

  for _, letter in ipairs(self._letters) do
    table.insert(chars, letter.char)
  end

  return table.concat(chars)
end

--- Returns the number of characters of the word.
---@return integer length The word length.
function Hangman:word_length() return #self._letters end

--- Returns the current and maximum number of lives.
---@return integer current Current remaining lives.
---@return integer max Maximum possible lives.
function Hangman:get_lives() return self._lives, MAX_LIVES end

--- Processes a guessed letter and updates the current game state.
--- Returns whether the guess was invalid, correct, wrong, or already used
---@param letter string The guessed letter.
---@return GuessResult result The result of the guess.
function Hangman:guess(letter)
  local normalized_letter = normalize_letter(letter)
  if not normalized_letter then return GuessResult.INVALID end

  -- A letter may only be used once, so a repeated guess is free.
  if self._used_letters[normalized_letter] then return GuessResult.DUPLICATE end

  -- Remember the letter and keep the list sorted.
  self._used_letters[normalized_letter] = true

  -- Reveal every position where the letter occurs. A letter can appear
  -- more than once, so the whole word is checked.
  local found = false
  for _, word_letter in ipairs(self._letters) do
    if word_letter.char == normalized_letter then
      word_letter.revealed = true
      found = true
    end
  end

  if found then return GuessResult.CORRECT end

  -- If the letter was not found, one life point is lost.
  self._lives = self._lives - 1

  return GuessResult.WRONG
end

--- Checks whether all letters of the word have been revealed.
---@return boolean complete `true` if the word is fully revealed, otherwise `false`.
function Hangman:is_word_complete()
  for _, word_letter in ipairs(self._letters) do
    if not word_letter.revealed then return false end
  end
  return true
end

--- Checks whether the player has no remaining life points.
---@return boolean dead `true` if the player has no life points left, otherwise `false`.
function Hangman:is_player_dead() return self._lives <= 0 end

--- Returns the word formatted for display with unrevealed letters hidden.
--- Example: `_ P P _ _` for the word `APPLE` if only `P` is revealed.
---@return string formatted The formatted word.
function Hangman:display_word()
  local letters = {}

  for _, letter in ipairs(self._letters) do
    table.insert(letters, letter.revealed and letter.char or '_')
  end

  return table.concat(letters, ' ')
end

--- Returns the used letters formatted for display.
---@return string formatted The formatted used letters.
function Hangman:display_used_letters()
  local letters = {}

  for letter, _ in pairs(self._used_letters) do
    table.insert(letters, letter)
  end

  table.sort(letters)

  return table.concat(letters, ',')
end

Hangman.GuessResult = GuessResult

return Hangman
