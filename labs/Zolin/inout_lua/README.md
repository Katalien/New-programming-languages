# In/out lab on Lua (Ivan Zolin)

## Deployment

```bash
docker build -t inout_lua_image .
docker run -it inout_lua_image
```

## Commands

1. Math expressions: `+-*/`(ex: 1+3)
2. `history` - show all history of commands
3. `delete` - delete all commands from hisory
4. `run <num>` - run a command (`<num>` is 1,2,...)
5. `exit` - exit from the loop
