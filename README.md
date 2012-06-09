bloomfilter-rust
================

A bloomfilter implemented in Rust

Ben Brittain
8/9/2012

contains a bloomfilter library and a sample-use/test file


bloomfilter.rs
================
a library file. usage like so:

to create:
    let bfilter = bloomfilter::bloomfilter(10000u, [sha1]);

to add:
    bloomfilter::add(bfilter, "example");

to check if in filter:
    bloomfilter::contains(bfilter,"example");

to combine filters:
    let bfilter3 = bloomfilter::union(bfilter, bfilter2);
 

any hashing function or functions can be used. They must be able to handle the type of the elements and return the hash in [u8].


bloomfilter-test.rs
================
ensures everything works throughout the development of rust.

an example hashing function from within the test file

    import sha1 = std::sha1;

    fn sha1(elem: str) -> [u8] {

        let sha1 = sha1::sha1();

        sha1.input_str(elem);

        let hashstr = sha1.result();

        sha1.reset();

        hashstr

    }

