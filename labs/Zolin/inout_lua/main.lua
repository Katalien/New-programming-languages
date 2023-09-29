-- Initialize the history file
local historyFile = "calculator_history.txt"
local history = {}

-- Load the history from the file if it exists
local file = io.open(historyFile, "r")
if file then
    for line in file:lines() do
        table.insert(history, line)
    end
    file:close()
end

-- Function to execute a calculator command
local function executeCommand(command)
    local result = load("return " .. command)
    if result then
        local success, value = pcall(result)
        if success then
            print("Result: " .. value)
            table.insert(history, command)
        else
            print("Error: " .. value)
        end
    else
        print("Invalid command")
    end
end

-- Main loop
while true do
    io.write("> ")
    local command = io.read()

    if command == "exit" then
        -- Save history to the file and exit
        local file = io.open(historyFile, "w")
        for _, cmd in ipairs(history) do
            file:write(cmd .. "\n")
        end
        file:close()
        break
    elseif command == "history" then
        -- Display command history
        for i, cmd in ipairs(history) do
            print(i .. ": " .. cmd)
        end
    else
        -- Execute the entered command
        executeCommand(command)
    end
end
