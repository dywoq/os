package standard

// LoaderInterruptions contains the methods that are standard and officially supported.
// This interface should be implemented by all loaders to ensure compatibility with other methods
// who use LoaderInterruptions.
//
// To implement the interface, you would need to realize it in your loader structure.
//
// All methods inside LoaderInterruptions must run only in the gathering mode,
// otherwise, they return an error.
//
// WARNING: If you add these methods to your structure, make sure they're also available
// through using Interrupt method.
// You can find all gates in inline documentation of methods inside LoaderInterruptions.
type LoaderInterruptions interface {
	LoaderLeaver
	LoaderPrinter
	LoaderRam
}

type LoaderLeaver interface {
	// Leave exits the loader and stops loading your operating system,
	// without doing any cleanup.
	//
	// Its gate and signature:
	// 	0x00 ()
	Leave() error
}

type LoaderPrinter interface {
	// Print prints the message.
	//
	// Its gate and signature:
	//  0x01 (string)
	Print(string) error
}

type LoaderRam interface {
	// Ram returns the available RAM.
	//
	// Its gate and signature:
	//  0x02 (uint64)
	Ram() (uint64, error)
}
