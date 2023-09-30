-- Initialize the history file
local historyFile = "calculator_history.txt"
local history = {}

-- Load the history from the file if it exists
local file = io.open(historyFile, "r+")  -- Change the mode to "r+" (read and write)
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
        local file = io.open(historyFile, "w")  -- Overwrite the file
        for _, cmd in ipairs(history) do
            print(cmd)
            file:write(cmd)
        end
        file:close()
        break
    elseif command == "history" then
        -- Display command history
        for i, cmd in ipairs(history) do
            print(i .. ": " .. cmd)
        end
    elseif command == "delete" then
        -- Delete history file
        os.remove(historyFile)
        print("History file deleted.")
        history = {}  -- Clear the history in memory
    elseif string.sub(command, 1, 4) == "run " then
        -- Execute a command from the history by index
        local index = tonumber(string.sub(command, 5))
        if index and index >= 1 and index <= #history then
            local selectedCommand = history[index]
            print("Running: " .. selectedCommand)
            executeCommand(selectedCommand)
        else
            print("Invalid index")
        end
    else
        -- Execute the entered command
        executeCommand(command)
    end
end
