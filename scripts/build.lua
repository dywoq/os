local os = require("os")

-- Executes the given command with arguments.
-- May terminate the process of the program if the command exited with 1 code.
local function execute_and_maybe_exit(command, args)
	local ok, exit_type, code = os.execute(command .. " " .. args)
	if exit_type ~= "exit" or code ~= 0 then
    	print("Command failed with exit code " .. tostring(code) .. ": " .. command .. " " .. args)
     	os.exit(1)
	end
end
