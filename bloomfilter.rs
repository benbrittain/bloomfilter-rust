#[link(name = "bloomfilter", vers = "0.1", author = "bbrittain")];
#[crate_type = "lib"];

use std;
import bitv = std::bitv;
import sha1 = std::sha1;

#[doc = "Bloom Filter Type"]
type bloomfilter = @{storage: bitv::bitv, capacity: uint, count: uint, error_rate: f64};

fn bloomfilter(capacity: uint, error_rate: f64) -> bloomfilter {
    io::println("Capacity: " + uint::to_str(capacity, 10u));
//    let num_slices = (f64::ceil(f64::logarithm(((1.0 as f64) / (error_rate as f64) ), (2 as f64)))) as uint;
 //   let bits_per_slice= (f64::ceil(((2 as f64) * (capacity as f64) * f64::abs(f64::ln(error_rate))) /
  //                      ((num_slices as f64) * (f64::pow((f64::ln(2 as f64)),2 as f64))))) as uint;
    //@{storage: bitv::bitv(capacity,false), num_bits: (num_slices * bits_per_slice), bits_per_slice: bits_per_slice, num_slices: num_slices, count: 0u, error_rate: error_rate}
    @{storage: bitv::bitv(capacity,false), capacity: capacity, count: 0u,  error_rate: error_rate}
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
  //      log(error,(loc));
        i += 1u;
        bitv::set(bloomfilter.storage, loc, true);
    } 
    
   // log(error,hashstr[0]); 
}

fn contains(bloomfilter: bloomfilter, elem: str) -> bool {
    let sha1 = sha1::sha1();
    sha1.input_str(elem);
    let hashstr = sha1.result();
    sha1.reset();
    let mut i = 0u;
    while i < 20u {
        let loc = (hashstr[i] as uint) % (bloomfilter.capacity - 1u);
//        log(error,(loc));
       
        let val = bitv::get(bloomfilter.storage, loc);
        if val == false {
            ret false;
        } 
        i += 1u;
    }
    ret true;
} 
