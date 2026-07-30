-- NOTE: Lua compatibility issues:
--
-- - `string.rep(string, anzahl [, separator])`:
--
--   Separator is not supported in Lua 5.1 or LuaJIT
--   use here `string.concat()` with a table instead.
--
--   ```lua
--   local header_parts = {}
--   local sep_parts = {}
--   for i = 1, col_count do
--     header_parts[i] = 'Dec Hex Chr'
--     sep_parts[i] = '-----------'
--   end
--   print(table.concat(header_parts, ' | '))
--   print(table.concat(sep_parts, '-+-'))
--   ```

--- Returns a printable character for the given ASCII code.
--- Non-printable characters are replaced with a dot.
---@param code integer
---@return string
local function printable(code)
  -- Check if the character is printable (space through tilde).
  if code >= 32 and code <= 126 then
    -- Convert the ASCII code to a character.
    return string.char(code)
  end

  return '.'
end

local ascii_first = 0 -- The first ASCII code to display.
local ascii_last = 127 -- The last ASCII code to display.

-- The number of columns to use for the ASCII table.
-- Use 1, 2, 4, or 8 columns for an optimal layout.
local col_count = 4

-- The number of ASCII codes to show in the table.
local code_count = ascii_last - ascii_first + 1

-- The number of rows needed to fit all codes into the columns.
local row_count = math.ceil(code_count / col_count)

-- Print the header of the table based on the number of columns.
print(string.rep('Dec Hex Chr', col_count, ' | '))
print(string.rep('-----------', col_count, '-+-'))

for row = 0, row_count - 1 do
  local line = ''
  for col = 0, col_count - 1 do
    -- Add separators between columns.
    if col > 0 then line = line .. ' | ' end

    -- Calculate the index of the ASCII code.
    local index = row + col * row_count

    -- Stop when the ASCII code exceeds the end of the range.
    if index >= code_count then break end

    -- Calculate the ASCII code from the index.
    local code = ascii_first + index

    -- Add the ASCII code in decimal, hexadecimal and as a character.
    -- With `%c` we can print the code as a character directly,
    -- but it will not be printable for non-printable characters.
    line = line .. string.format('%3d %3X  %s ', code, code, printable(code))
  end
  print(line)
end
