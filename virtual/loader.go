package virtual

// Loader represents any loader what loads OS
// and helps initialize it.
type Loader interface {
	// Interrupt causes a interruption to the loader.
	// Must return an error if the current mode is not Gathering mode.
	//
	// All interruptions gate you can see on the virtual loaders'
	// documentation.
	Interrupt(o Os, gate uintptr, args ...any) (any, error)
}
