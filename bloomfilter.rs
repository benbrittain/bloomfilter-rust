#[link(name = "bloomfilter", vers = "0.1", author = "bbrittain")];
#[crate_type = "lib"];

use std;
import bitv = std::bitv;

#[doc = "Bloom Filter Type"]
type bloomfilter<T> = @{storage: bitv::bitv, capacity: uint, mut count: uint, hash_funcs: [(fn@(++T) -> [u8])]};

#[doc = "create a Bloom Filter. Requires a capacity and an array of hashing functions to use. The hashing functions return type must be [u8]"]
fn bloomfilter<T>(capacity: uint, hash_funcs: [(fn@(++T) -> [u8])]) -> bloomfilter<T> {
    @{storage: bitv::bitv(capacity,false), capacity: capacity, mut count: 0u, hash_funcs: copy hash_funcs}
}


#[doc = "add an element to the bloomfilter"]
fn add<T>(bloomfilter: bloomfilter<T>, elem: T ){
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

#[doc = "check to see if the element might be in the bloomfilter (or certainly not)"]
fn contains<T>(bloomfilter: bloomfilter<T>, elem: T ) -> bool {
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

#[doc = "check to see if two bloomfilters are equal. Must be the same size"]
fn equal<T>(bloomfilter_one: bloomfilter<T>, bloomfilter_two: bloomfilter<T>) -> bool {
    assert bloomfilter_one.capacity == bloomfilter_two.capacity;
    if bitv::equal(bloomfilter_one.storage,bloomfilter_two.storage){
        true
    } else {
        false
    }
}

#[doc = "merge two bloom filters together. Must be the same size"]
fn union<T>(bloomfilter_one: bloomfilter<T>, bloomfilter_two: bloomfilter<T>) -> bloomfilter<T> {
    assert bloomfilter_one.capacity == bloomfilter_two.capacity;
    let bloomfilter_new = copy bloomfilter_one;
    bitv::union(bloomfilter_new.storage,bloomfilter_two.storage);
    let count_new = bloomfilter_one.count + bloomfilter_two.count;
    bloomfilter_new.count = count_new;
    bloomfilter_new
}
