# Lua Notes

## Best Practices

### Append to a table

Both of the following examples append values to the end of a sequence.

Direct assignment avoids the overhead of a function call and is therefore
slightly faster:

```lua
values[#values + 1] = value
```

Using `table.insert()` is equivalent and may be preferred for readability.

```lua
table.insert(values, value)
```

### Initialize the random number generator

Call `math.randomseed(os.time())` once at the start of the main program before
using `math.random()`.

The seed initializes Lua's random number generator. Using the current time
ensures that random values differ between program runs.

Keep this initialization in the main program instead of individual modules,
because the random number generator is global and should only be initialized
once.

```lua
-- Seed the random number generator with the current time.
math.randomseed(os.time())

-- Import required modules.
local Module = require('module')
```

### Name: `error`

Avoid using `error` as a variable or function name, because it shadows Lua's
global `error()` function. Use names like `err`, `error_code`, or
`error_message` instead.

> [!TIP]
>
> Shadowing means that a local variable or function has the same name as an
> existing one, making the original name inaccessible within that scope.

```lua
error(message [, level])
```

```lua
local function do_something(text)
  if text == nil then
    return nil, "text cannot be nil"
  end

  return text
end

local result, err = do_something(nil)
if not result then
  error(err)
end
```

## Compatibility

### `utf8`

The standard `utf8` library was introduced in Lua 5.3. Functions such as
`utf8.len()`, `utf8.codes()`, and `utf8.char()` are not available in earlier Lua
versions.

### LuaJIT

LuaJIT is largely compatible with Lua 5.1 and includes some features from newer
Lua versions. Neovim uses LuaJIT as its embedded Lua runtime, so plugins and
configuration code run on LuaJIT rather than standard Lua 5.3 or later.
