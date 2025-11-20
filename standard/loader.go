package standard


// LoaderInterruptions contains the methods that are standard and officially supported.
// This interface should be implemented by all loaders to ensure compatibility with other methods
// who use LoaderInterruptions.
//
// To implement the interface, you would need to realize it in your loader structure.
//
// WARNING: If you add these methods to your structure, make sure they're also available
// through using Interrupt method.
// You can find all gates in inline documentation of methods inside LoaderInterruptions.
type LoaderInterruptions interface {
	LoaderLeaver
}

type LoaderLeaver interface {
	// Leave exits the loader and stops loading your operating system,
	// without doing any cleanup.
	//
	// Its gate and signature:
	// 	0x00 ()
	Leave()
}
