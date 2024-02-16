use std::iter::Enumerate;
use circular_buffer::CircularBuffer;
use std::fmt;
use std::mem::swap;
use std::collections::HashMap;
use once_cell::sync::Lazy;
 const fn dividableBy(i: u32 ) -> u32
{
    let mut count = 0;
    if i==0
    { return 0; }

    let mut i2: _ = i;
    //println!("I: {i}");

    while (i2%2)==0 {

        i2/=2;
       // println!("{i2}");
        count+=1;
        if i2 ==0
        { return 0; }
    }
   // println!("C: {count}");

    return count;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Dart
{
    value: u32,
    multiplier: u32

}

#[derive(Debug)]

struct Neighboors(u32,u32);

#[derive(Debug)]
struct Complexity
{
    halfable: u32,
    darts: u32,
    neighboors: Vec<Neighboors>

}

const SINGLE: [u32;21] = [1,2,3,4,5,6,7,8,9,10, 11,12,13,14,15,16,17,18,19,20,25];
const BOARD: [u32;20] = [20,1,18,4,13,6,10,15,2,17,3,19,7,16,8,11,14,9,12,5];

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Checkout {
    first_dart: Dart,
    second_dart: Dart,
    third_dart: Dart,


}
impl Checkout
{
    fn complexity(&self) -> Complexity
    {
        let first_val = self.first_dart.value;
        let second_val = self.second_dart.value;
        let third_val = self.third_dart.value;
        let halfableV = HALFABLE[if (third_val as usize) <25 { third_val as usize -1} else { 20}];
        let mut count =3;
        if self.first_dart.multiplier==0
        { count -= 1; }
        if self.second_dart.multiplier==0
        { count -= 1; }
        let firsti = BOARD.se

        let  firstleftneighboor = if first_val== *BOARD.first().unwrap() { *BOARD.last().unwrap() } else {*BOARD.get(first_val as usize-1).unwrap()};
        let  firstrightneighboor = if first_val== *BOARD.last().unwrap() { *BOARD.first().unwrap() } else {*BOARD.get(first_val as usize+1).unwrap()};

        let n1 : Neighboors = Neighboors(firstleftneighboor, firstrightneighboor);

        let  secondleftneighboor = if second_val== *BOARD.first().unwrap() { *BOARD.last().unwrap() } else {*BOARD.get(second_val as usize-1).unwrap()};
        let  secondrightneighboor = if second_val== *BOARD.last().unwrap() { *BOARD.first().unwrap() } else {*BOARD.get(second_val as usize+1).unwrap()};
        let n2 : Neighboors = Neighboors(secondleftneighboor, secondrightneighboor);

        let  thirdleftneighboor = if third_val== *BOARD.first().unwrap() { *BOARD.last().unwrap() } else {*BOARD.get(third_val as usize-1).unwrap()};
        let  thirdrightneighboor = if third_val== *BOARD.last().unwrap() { *BOARD.first().unwrap() } else {*BOARD.get(third_val as usize+1).unwrap()};
        let n3 : Neighboors = Neighboors(thirdleftneighboor, thirdrightneighboor);
       return  Complexity{
           halfable: halfableV,
           darts: count,
           neighboors: if count==1 { vec![n3] } else if count==2 {vec![n2,n3]}else {vec![n1,n2,n3]}
       };
    }
}



 const fn create_half_able< const COUNT: usize>(array: &[u32; COUNT]) -> [u32; COUNT]
{
    let mut i=0;
    let mut array_half_able: [u32;COUNT] = [0;COUNT];
    while i<COUNT { // needed a while loop since iteration is not const in rust https://github.com/rust-lang/rfcs/blob/master/text/2344-const-looping.md
        array_half_able[i] = dividableBy(array[i]);
        i+=1;
    }
    return array_half_able;
}

const COUNT: usize = 21;

const HALFABLE: [u32;21] = create_half_able(&SINGLE);

use phf::{phf_map};
// const HALFABLEMAP: phf::Map::<usize,u32> = phf_map!{
//     1 => 0,
//     2 => 1,
//     3 => 0,
//     4 => 2,
//     5 => 0,
//     6 => 1,
//     7 => 0,
//     8 => 3,
//     9 => 0,
//     10 => 1,
//     11 => 0,
//     12 => 2,
//     13 => 0,
//     14 => 1,
//     15 => 0,
//     16 => 4,
//     };

// const fn createHalfAbleMap() -> phf::Map::<usize,u32>
// {
//     let mut halfablemap = phf::Map::<usize,u32>::new();
//     let mut i=0;
//
//     while i<COUNT  {
//         if i==20 { halfablemap.(25,0); continue}
//         halfablemap.insert(i+1,HALFABLEMAP[&i]);
//         i+=1;
//     }
//     return halfablemap;
// }





fn main() {

    let arrayWithoutBull: [u32;20] = [1,2,3,4,5,6,7,8,9,10, 11,12,13,14,15,16,17,18,19,20];

    for (i,h) in HALFABLE.iter().enumerate() {
        let str = format!("{:?} is {} dividable by 2", SINGLE[i], h);
        println!("{}", str.as_str() )
    }
    let board = CircularBuffer::<21, u32>::from(BOARD);
    let mut checkouts: Vec<Checkout> = Vec::<Checkout>::new();



    let test_number = 6;
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
     checkouts.sort_by(|a,b| a.second_dart.multiplier.cmp(&b.second_dart.multiplier));
checkouts.sort();
    checkouts.dedup();

    for c in &checkouts {
        if  c.first_dart.multiplier>0 && c.second_dart.multiplier>0
        {
            println!("match {}*{}+ {}*{}+ 2*{} -> Score {:?}",
                     c.first_dart.multiplier, c.first_dart.value, c.second_dart.multiplier, c.second_dart.value, c.third_dart.value, c.complexity());
        }else if c.first_dart.multiplier==0 && c.second_dart.multiplier>0
        {
            println!("match      {}*{}+ 2*{} -> Score {:?}",
                    c.second_dart.multiplier, c.second_dart.value, c.third_dart.value, c.complexity());
        }else  {
            println!("match           2*{} -> Score {:?}",
                           c.third_dart.value,  c.complexity());
        }

    }
    println!("Found {} matches",checkouts.len());

}
