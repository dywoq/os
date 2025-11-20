package virtual

// Mode represents the mode of [Os].
type Mode string

// Os represents the virtual operating system.
type Os interface {
	// Mode returns the current mode
	// Os is in.
	Mode() Mode

	// Enter enters the new mode.
	// It should move only forward, not backwards:
	// if the current mode is set to init mode, OS can't return to gathering mode.
	Enter(m Mode) error

	// Syscall creates a system call, with the unique Id
	// and provided arguments.
	//
	// All system calls table you may find
	// on your official OS documentation you use.
	Syscall(id uintptr, args ...any) (any, error)

	// Interruption causes a interruption,
	// with the set gate and provided arguments.
	//
	// All interruptions gates you may find
	// on your official OS documentation you use.
	Interruption(gate uintptr, args ...any) (any, error)

	// Startup starts the OS in the user mode,
	// with the provided data got from the gathering mode.
	//
	// You should use it after you initialized everything
	// required.
	Startup(data map[string]any) error
}

const (
	ModeGathering Mode = "mode-gathering"
	ModeInit      Mode = "mode-init"
	ModeStartup   Mode = "mode-startup"
)
