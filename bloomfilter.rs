#[link(name = "bloomfilter", vers = "0.1", author = "bbrittain")];
#[crate_type = "lib"];

use std;
import bitv = std::bitv;
import sha1 = std::sha1;

#[doc = "Bloom Filter Type"]
type bloomfilter = @{storage: bitv::bitv, capacity: uint, mut count: uint};

fn bloomfilter(capacity: uint) -> bloomfilter {
    io::println("Capacity: " + uint::to_str(capacity, 10u));
    @{storage: bitv::bitv(capacity,false), capacity: capacity, mut count: 0u}
}

fn add(bloomfilter: bloomfilter, elem: str){
 //   log(error,bloomfilter.capacity);
    let sha1 = sha1::sha1();
    sha1.input_str(elem);
    let hashstr = sha1.result();
    sha1.reset();
    let mut i = 0u;
    while i < 20u {
        let loc = (hashstr[i] as uint) % (bloomfilter.capacity - 1u);
        i += 1u;
        bitv::set(bloomfilter.storage, loc, true);
    } 
    bloomfilter.count += 1u;
    assert bloomfilter.count <= bloomfilter.capacity;
}

fn contains(bloomfilter: bloomfilter, elem: str) -> bool {
    let sha1 = sha1::sha1();
    sha1.input_str(elem);
    let hashstr = sha1.result();
    sha1.reset();
    let mut i = 0u;
    while i < 20u {
        let loc = (hashstr[i] as uint) % (bloomfilter.capacity - 1u);
        let val = bitv::get(bloomfilter.storage, loc);
        if val == false {
            ret false;
        } 
        i += 1u;
    }
    ret true;
} 

fn equal(bloomfilter_one: bloomfilter, bloomfilter_two: bloomfilter) -> bool {
    assert bloomfilter_one.capacity == bloomfilter_two.capacity;
    if bitv::equal(bloomfilter_one.storage,bloomfilter_two.storage){
        true
    } else {
        false
    }
}

fn union(bloomfilter_one: bloomfilter, bloomfilter_two: bloomfilter) -> bloomfilter {
    assert bloomfilter_one.capacity == bloomfilter_two.capacity;
    let bloomfilter_new = copy bloomfilter_one;
    bitv::union(bloomfilter_new.storage,bloomfilter_two.storage);
    let count_new = bloomfilter_one.count + bloomfilter_two.count;
    bloomfilter_new.count = count_new;
    bloomfilter_new
}
