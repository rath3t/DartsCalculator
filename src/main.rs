use std::iter::Enumerate;
use circular_buffer::CircularBuffer;
use std::fmt;
use std::mem::swap;
use std::ops::{DivAssign, Rem};

use num_traits::PrimInt;
const fn dividableBy<T>(i: T ) -> u32 where
T: PrimInt + std::cmp::PartialEq<u32> + std::fmt::Display + std::cmp::PartialEq<u32> + std::ops::DivAssign<u32> + TryFrom<u32>
{
    let mut count = 0;
    if i==0
    { return 0; }

    let mut i2: _ = i;
    println!("I: {i}");
    let two = T::try_from(2).ok().unwrap();

    while (i2%two)==0 {

        i2/=2;
        println!("{i2}");
        count+=1;
        if i2 ==0
        { return 0; }
    }
    println!("C: {count}");

    return count;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Dart
{
    value: u32,
    multiplier: u32

}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Checkout {
    first_dart: Dart,
    second_dart: Dart,
    third_dart: Dart,
}

const SINGLE: [u32;21] = [1,2,3,4,5,6,7,8,9,10, 11,12,13,14,15,16,17,18,19,20,25];

const fn create_half_able<T: num_traits::PrimInt + std::cmp::PartialEq<u32> + std::fmt::Display + std::ops::DivAssign<u32> + std::convert::From<u32>, const COUNT: usize>(array: &[T; COUNT]) -> [T; COUNT]
{
    let mut i=0;
    let mut array_half_able: [T;COUNT];
    while i<=COUNT { // needed a while loop since iteration is not const in rust https://github.com/rust-lang/rfcs/blob/master/text/2344-const-looping.md
        array_half_able[i] = dividableBy(array[i]).into();

    }
    return array_half_able;
}


const HALFABLE: [u32;21] = create_half_able(&SINGLE);

fn main() {
    let arrayWithoutBull: [u32;20] = [1,2,3,4,5,6,7,8,9,10, 11,12,13,14,15,16,17,18,19,20];

    for (i,h) in HALFABLE.iter().enumerate() {
        let str = format!("{:?} is {} dividable by 2", SINGLE[i], h);
        println!("{}", str.as_str() )
    }
    let board = CircularBuffer::<21, u32>::from(SINGLE);
    let mut checkouts: Vec<Checkout> = Vec::<Checkout>::new();



    let test_number = 5;
    for first_dart_mult in [0,1,2,3] {
        for first_dart_value in SINGLE {
            if first_dart_mult == 0 && first_dart_value > 1 {continue}

            if first_dart_value == 25 && first_dart_mult == 3 { continue }
            let first_dart = first_dart_mult * first_dart_value;
            for second_dart_mult in [0,1,2,3] {

                for second_dart_value in SINGLE {
                    if second_dart_mult == 0 && second_dart_value > 1 {continue}
                    if second_dart_value == 25 && second_dart_mult == 3 { continue }

                    let second_dart = second_dart_mult * second_dart_value;

                for third_dart_value in SINGLE {
                    let third_dart = 2 * third_dart_value;

                    if test_number == first_dart + second_dart + third_dart {
                        let test_number_reproduction =first_dart_mult*first_dart_value+ second_dart_mult*second_dart_value+ 2*third_dart_value;
                        //println!("match {first_dart_mult}*{first_dart_value}+ {second_dart_mult}*{second_dart_value}+ 2*{third_dart_value} = {test_number_reproduction}");
                        checkouts.push(Checkout{ first_dart: Dart { value: first_dart_value, multiplier: first_dart_mult },
                            second_dart: Dart { value: second_dart_value, multiplier: second_dart_mult }, third_dart: Dart { value: third_dart_value, multiplier: 2 } } )
                    }
                }
                }
            }
        }
    }
    for c in &checkouts {
        println!("match {}*{}+ {}*{}+ 2*{}",
                 c.first_dart.multiplier,c.first_dart.value,c.second_dart.multiplier,c.second_dart.value, c.third_dart.value);

    }

    use std::collections::HashMap;

    let mut halfablemap = HashMap::<usize,u32>::new();
    for (i,h) in HALFABLE.iter().enumerate() {
        //println!(" key: {} value: {}",i,h);

        if i==20 { halfablemap.insert(25,0); continue}
        halfablemap.insert(i+1,*h);
    }
//
   //     println!("v {v}");
  //  }
    for c in &halfablemap {
        println!(" key: {} value: {}",c.0,c.1);

    }
    checkouts.sort_by(|a,b| halfablemap[&(b.third_dart.value as usize)].cmp(&halfablemap[&(a.third_dart.value as usize)]));
    for mut c in &mut checkouts {
        if c.second_dart.multiplier==0 && c.first_dart.multiplier >0
        {
            swap(&mut c.first_dart,&mut c.second_dart);
        }
    }




     checkouts.sort_by(|a,b| a.first_dart.multiplier.cmp(&b.first_dart.multiplier));
checkouts.sort();
    checkouts.dedup();

    for c in &checkouts {
        if  c.first_dart.multiplier>0
        {
            println!("match {}*{}+ {}*{}+ 2*{} -> Score {}",
                     c.first_dart.multiplier, c.first_dart.value, c.second_dart.multiplier, c.second_dart.value, c.third_dart.value, halfablemap[&(c.third_dart.value as usize)]);
        }else {  println!("match      {}*{}+ 2*{} -> Score {}",
                           c.second_dart.multiplier, c.second_dart.value, c.third_dart.value, halfablemap[&(c.third_dart.value as usize)]);
        }

    }
    println!("Found {} matches",checkouts.len());

}
