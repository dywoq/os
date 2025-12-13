local executing = require("./executing")
executing.maybe_exit("cd", "../")
executing.maybe_exit("cargo", "clean")
