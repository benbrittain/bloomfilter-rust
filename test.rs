import sha1 = std::sha1;

#[doc = "an example hashing function using the standard library sha1"]
fn sha1(elem: str) -> [u8] {
    let sha1 = sha1::sha1();
    sha1.input_str(elem);
    let hashstr = sha1.result();
    sha1.reset();
    hashstr 
}

#[test]
fn test1() {
    let bfilter = bloomfilter(10000u, [sha1]);
    add(bfilter, "a");
    assert contains(bfilter,"a");
    add(bfilter, "variety");
    assert contains(bfilter,"variety");
    add(bfilter, "of test");
    assert contains(bfilter,"of test");
    add(bfilter, "in");
    assert contains(bfilter,"in");
    add(bfilter, "Rust!");
    assert contains(bfilter,"Rust!");
    assert (contains(bfilter,"randomstuff") == false);
}

#[test]
fn test2() {
    let bfilter = bloomfilter(10u, [sha1]);
    add(bfilter, "a collision test");
    assert contains(bfilter,"a col test");
    assert contains(bfilter,"test");
}

fn test3() {
    let bfilter = bloomfilter(100u, [sha1]);
    let bfilter2 = bloomfilter(100u, [sha1]);
    add(bfilter, "a merge test");
    assert contains(bfilter,"a merge test");
    assert (contains(bfilter,"Rust!") == false);
    assert (contains(bfilter,"not in there!") == false);
    add(bfilter2, "Rust!");
    assert contains(bfilter2,"Rust!");
    assert (contains(bfilter2,"a merge test") == false);
    assert (contains(bfilter2,"not in there!") == false);
    let bfilter3 = union(bfilter, bfilter2);
    assert contains(bfilter3,"Rust!");
    assert contains(bfilter3,"a merge test") ;
    assert (contains(bfilter3,"not in there!") == false);

}

fn test4() {
    fn psuedosha1(elem: str) -> [u8] {
        let sha1 = sha1::sha1();
        sha1.input_str(elem);
        let hashstr = sha1.result();
        sha1.reset();
        let mut newhash: [u8] = [];
        for vec::each(hashstr) |elm| {
            newhash = newhash + [elm + 3 as u8];
        }
        newhash 
    }

    let bfilter = bloomfilter(10000u, [psuedosha1,sha1]);
    add(bfilter, "a");
    assert contains(bfilter,"a");
    add(bfilter, "variety");
    assert contains(bfilter,"variety");
    add(bfilter, "of test");
    assert contains(bfilter,"of test");
    add(bfilter, "in");
    assert contains(bfilter,"in");
    add(bfilter, "Rust!");
    assert contains(bfilter,"Rust!");
    assert (contains(bfilter,"randomstuff") == false);
}

