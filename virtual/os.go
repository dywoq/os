package virtual

type Mode string

// Os represents the virtual operating system.
type Os interface {
	// Mode returns the current mode
	// Os is in.
	Mode() Mode

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
}

const (
	ModeGathering Mode = "mode-gathering"
	ModeInit      Mode = "mode-init"
	ModeStartup   Mode = "mode-startup"
)
