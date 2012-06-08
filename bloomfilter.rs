#[link(name = "bloomfilter", vers = "0.1", author = "bbrittain")];
#[crate_type = "lib"];

use std;
import bitv = std::bitv;
import sha1 = std::sha1;

#[doc = "Bloom Filter Type"]
type bloomfilter = @{storage: bitv::bitv, capacity: uint, mut count: uint, hash_funcs: [(fn@(str) -> [u8])]};

fn bloomfilter(capacity: uint, hash_funcs: [(fn@(str) -> [u8])]) -> bloomfilter {
    @{storage: bitv::bitv(capacity,false), capacity: capacity, mut count: 0u, hash_funcs: copy hash_funcs}
}

fn add(bloomfilter: bloomfilter, elem: str){
    for vec::each(bloomfilter.hash_funcs) {|func|
        let hashstr:[u8] = func(elem); 
            for vec::each(hashstr) {|elm|
                let loc = (elm as uint) % (bloomfilter.capacity - 1u);
                bitv::set(bloomfilter.storage, loc, true);
            }
    }
    bloomfilter.count += 1u;
    assert bloomfilter.count <= bloomfilter.capacity;
}

fn contains(bloomfilter: bloomfilter, elem: str) -> bool {
    for vec::each(bloomfilter.hash_funcs) {|func|
        let hashstr:[u8] = func(elem); 
            for vec::each(hashstr) {|elm|
                let loc = (elm as uint) % (bloomfilter.capacity - 1u);
                let val = bitv::get(bloomfilter.storage, loc);
                if val == false {
                    ret false;
                } 
                bitv::set(bloomfilter.storage, loc, true);
            }
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
