In order to get `core` to cross-compile and link successfully (w/ `no_std` and `no_main`)
it is required that you cross-compile libcore and copy the cross-compiled libs to your
rustc sysroot AND `xargo` must be used. Directions for this process can be found here:

	https://github.com/japaric/rust-cross#rust-cross

