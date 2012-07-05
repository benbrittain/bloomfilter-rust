use bloomfilter;
use std;
import sha1 = std::sha1;

#[doc = "an example hashing function using the standard library sha1"]
fn sha1(elem: str) -> [u8] {
    let sha1 = sha1::sha1();
    sha1.input_str(elem);
    let hashstr = sha1.result();
    sha1.reset();
    hashstr 
}


#[doc = "A variety of test cases"]
fn main(){
    
    test1();
    test2();
    test3();
    test4();
    io::println("bloomin!");
}
fn test1() {
    let bfilter = bloomfilter::bloomfilter(10000u, [sha1]);
    bloomfilter::add(bfilter, "a");
    assert bloomfilter::contains(bfilter,"a");
    bloomfilter::add(bfilter, "variety");
    assert bloomfilter::contains(bfilter,"variety");
    bloomfilter::add(bfilter, "of test");
    assert bloomfilter::contains(bfilter,"of test");
    bloomfilter::add(bfilter, "in");
    assert bloomfilter::contains(bfilter,"in");
    bloomfilter::add(bfilter, "Rust!");
    assert bloomfilter::contains(bfilter,"Rust!");
    assert (bloomfilter::contains(bfilter,"randomstuff") == false);
}

fn test2() {
    let bfilter = bloomfilter::bloomfilter(10u, [sha1]);
    bloomfilter::add(bfilter, "a collision test");
    assert bloomfilter::contains(bfilter,"a col test");
    assert bloomfilter::contains(bfilter,"test");
}

fn test3() {
    let bfilter = bloomfilter::bloomfilter(100u, [sha1]);
    let bfilter2 = bloomfilter::bloomfilter(100u, [sha1]);
    bloomfilter::add(bfilter, "a merge test");
    assert bloomfilter::contains(bfilter,"a merge test");
    assert (bloomfilter::contains(bfilter,"Rust!") == false);
    assert (bloomfilter::contains(bfilter,"not in there!") == false);
    bloomfilter::add(bfilter2, "Rust!");
    assert bloomfilter::contains(bfilter2,"Rust!");
    assert (bloomfilter::contains(bfilter2,"a merge test") == false);
    assert (bloomfilter::contains(bfilter2,"not in there!") == false);
    let bfilter3 = bloomfilter::union(bfilter, bfilter2);
    assert bloomfilter::contains(bfilter3,"Rust!");
    assert bloomfilter::contains(bfilter3,"a merge test") ;
    assert (bloomfilter::contains(bfilter3,"not in there!") == false);

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

    let bfilter = bloomfilter::bloomfilter(10000u, [psuedosha1,sha1]);
    bloomfilter::add(bfilter, "a");
    assert bloomfilter::contains(bfilter,"a");
    bloomfilter::add(bfilter, "variety");
    assert bloomfilter::contains(bfilter,"variety");
    bloomfilter::add(bfilter, "of test");
    assert bloomfilter::contains(bfilter,"of test");
    bloomfilter::add(bfilter, "in");
    assert bloomfilter::contains(bfilter,"in");
    bloomfilter::add(bfilter, "Rust!");
    assert bloomfilter::contains(bfilter,"Rust!");
    assert (bloomfilter::contains(bfilter,"randomstuff") == false);
}

