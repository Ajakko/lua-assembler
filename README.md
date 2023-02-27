# lua-assembler
Assembles all Lua files from a specificied path into one file.

## Usage
POSIX: 
```
./lua-assemble folder_of_lua_files path/index.lua
```

Windows
```
.\lua-assemble.exe folder_of_lua_files path\index.lua
```

### Demo
```
./lua-assemble files index.lua
```
Or
```
./lua-assemble files files/index.lua
```

```lua
-- files/player.lua
local player_name = "Dude"
```
```lua
-- files/enemy.lua
local enemy = "Lucy"
```
```lua
-- files/index.lua
-- files/player.lua
local player_name = "Dude"
-- files/enemy.lua
local enemy = "Lucy"
```