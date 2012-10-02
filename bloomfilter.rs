use bitv = std::bitv;

#[doc = "Bloom Filter Type"]
type bloomfilter<T> = @{storage: bitv::Bitv, capacity: uint, mut count: uint, hash_funcs: ~[(fn@(&T) -> ~[u8])]};

#[doc = "create a Bloom Filter. Requires a capacity and an array of hashing functions to use. The hashing functions return type must be [u8]"]
fn bloomfilter<T>(capacity: uint, hash_funcs: ~[(fn@(&T) -> ~[u8])]) -> bloomfilter<T> {
    @{storage: bitv::Bitv(capacity,false), capacity: capacity, mut count: 0u, hash_funcs: copy hash_funcs}
}


#[doc = "add an element to the bloomfilter"]
fn add<T>(bloomfilter: bloomfilter<T>, elem: &T ){
    for vec::each(bloomfilter.hash_funcs) |func| {
        let hashstr:~[u8] = (*func)(elem); 
            for vec::each(hashstr) |elm| {
                let loc = (*elm as uint) % (bloomfilter.capacity - 1u);
                bloomfilter.storage.set(loc, true);
            }
    }
    bloomfilter.count += 1u;
    assert bloomfilter.count <= bloomfilter.capacity;
}

#[doc = "check to see if the element might be in the bloomfilter (or certainly not)"]
fn contains<T>(bloomfilter: bloomfilter<T>, elem: &T ) -> bool {
    for vec::each(bloomfilter.hash_funcs) |func| {
        let hashstr:~[u8] = (*func)(elem); 
        for vec::each(hashstr) |elm| {
                let loc = (*elm as uint) % (bloomfilter.capacity - 1u);
                let val = bloomfilter.storage.get(loc);
                if val == false {
                    return false;
                } 
                bloomfilter.storage.set(loc, true);
            }
    }
    return true;
} 

#[doc = "check to see if two bloomfilters are equal. Must be the same size"]
fn equal<T>(bloomfilter_one: bloomfilter<T>, bloomfilter_two: bloomfilter<T>) -> bool {
    assert bloomfilter_one.capacity == bloomfilter_two.capacity;
    if bloomfilter_one.storage.equal(&bloomfilter_two.storage) {
        true
    } else {
        false
    }
}

#[doc = "merge two bloom filters together. Must be the same size"]
fn union<T>(bloomfilter_one: bloomfilter<T>, bloomfilter_two: bloomfilter<T>) -> bloomfilter<T> {
    assert bloomfilter_one.capacity == bloomfilter_two.capacity;
    let bloomfilter_new = copy bloomfilter_one;
    bloomfilter_new.storage.union(&bloomfilter_two.storage);
    let count_new = bloomfilter_one.count + bloomfilter_two.count;
    bloomfilter_new.count = count_new;
    bloomfilter_new
}


