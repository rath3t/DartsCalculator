use std::fmt;
use std::mem::swap;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

 const fn dividable_by(i: u32 ) -> u32
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

#[derive(Debug,Copy, Clone)]

struct Neighboors(u32,u32);

#[derive(Debug)]
struct Complexity
{
    halfable: u32,
    darts: u32,
    neighboors: Vec<Neighboors>

}

const SINGLE: [u32;21] = [1,2,3,4,5,6,7,8,9,10, 11,12,13,14,15,16,17,18,19,20,25];
//const BOARD: [u32;20] = [20,1,18,4,13,6,10,15,2,17,3,19,7,16,8,11,14,9,12,5];

use phf::{phf_map};

static NEIGHBOORS: phf::Map<u32, Neighboors> = phf_map! {
    1u32 => Neighboors(20,18),
    2u32 => Neighboors(15,17),
    3u32 => Neighboors(17,19),
    4u32 => Neighboors(18,13),
    5u32 => Neighboors(12,20),
    6u32 => Neighboors(13,10),
    7u32 => Neighboors(19,16),
    8u32 => Neighboors(16,11),
    9u32 => Neighboors(14,12),
    10u32 => Neighboors(6,15),
    11u32 => Neighboors(8,14),
    12u32 => Neighboors(9,5),
    13u32 => Neighboors(4,6),
    14u32 => Neighboors(11,9),
    15u32 => Neighboors(10,2),
    16u32 => Neighboors(7,8),
    17u32 => Neighboors(2,3),
    18u32 => Neighboors(1,4),
    19u32 => Neighboors(3,7),
    20u32 => Neighboors(5,1),
    25u32 => Neighboors(0,0),
        };

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Checkout {
    first_dart: Dart,
    second_dart: Dart,
    third_dart: Dart,


}
impl Display for Neighboors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:>2}, {:>2}", self.0, self.1)
    }
}

impl Display for Complexity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str= String::from("Neighboors: ");
        for neighboor in &self.neighboors{
             str += &*format!(" {{{}}}, ", neighboor);

        }

        write!(f, "Darts: {}, Last dart halfability: {}, {}",
                   self.darts, self.halfable, str)

    }
}
impl Display for Checkout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let multiplier_string = |i: u32| -> &str { if i==1 {"S"}else if i==2 {"D"} else {"T"}};

        if self.first_dart.multiplier > 0 && self.second_dart.multiplier > 0
        {
            write!(f, "match {}{: >2} + {}{: >2} + D{: >2} -> Score {}",
                   multiplier_string(self.first_dart.multiplier), self.first_dart.value, multiplier_string(self.second_dart.multiplier), self.second_dart.value, self.third_dart.value, self.complexity())
        } else if self.first_dart.multiplier == 0 && self.second_dart.multiplier > 0
        {
            write!(f, "match      {}{: >2} + D{: >2} -> Score {}",
                   multiplier_string(self.second_dart.multiplier), self.second_dart.value, self.third_dart.value, self.complexity())
        } else {
            write!(f, "match            D{: >2} -> Score {}",
                   self.third_dart.value, self.complexity())
        }
    }
}

impl Checkout
{
    fn complexity(&self) -> Complexity
    {
        let first_val = self.first_dart.value;
        let second_val = self.second_dart.value;
        let third_val = self.third_dart.value;
        let halfable_v = HALFABLE[if (third_val as usize) <25 { third_val as usize -1} else { 20}];
        let mut count =3;
        if self.first_dart.multiplier==0
        { count -= 1; }
        if self.second_dart.multiplier==0
        { count -= 1; }

        let n1 :& Neighboors = &NEIGHBOORS[&first_val];
        let n2 : &Neighboors = &NEIGHBOORS[&second_val];
        let n3 : &Neighboors = &NEIGHBOORS[&third_val];

       return  Complexity{
           halfable: halfable_v,
           darts: count,
           neighboors: if count==1 { vec![*n3] } else if count==2 {vec![*n2,*n3]}else {vec![*n1,*n2,*n3]}
       };
    }


}



 const fn create_half_able< const COUNT: usize>(array: &[u32; COUNT]) -> [u32; COUNT]
{
    let mut i=0;
    let mut array_half_able: [u32;COUNT] = [0;COUNT];
    while i<COUNT { // needed a while loop since iteration is not const in rust https://github.com/rust-lang/rfcs/blob/master/text/2344-const-looping.md
        array_half_able[i] = dividable_by(array[i]);
        i+=1;
    }
    return array_half_able;
}

const COUNT: usize = 21;

const HALFABLE: [u32;COUNT] = create_half_able(&SINGLE);

fn main() {

    let mut checkouts: Vec<Checkout> = Vec::<Checkout>::new();

    let test_number = 121;
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
                        checkouts.push(Checkout{ first_dart: Dart { value: first_dart_value, multiplier: first_dart_mult },
                            second_dart: Dart { value: second_dart_value, multiplier: second_dart_mult }, third_dart: Dart { value: third_dart_value, multiplier: 2 } } )
                    }
                }
                }
            }
        }
    }

    let mut halfablemap = HashMap::<usize,u32>::new();
    for (i,h) in HALFABLE.iter().enumerate() {

        if i==20 { halfablemap.insert(25,0); continue}
        halfablemap.insert(i+1,*h);
    }

    checkouts.sort_by(|a,b| halfablemap[&(b.third_dart.value as usize)].cmp(&halfablemap[&(a.third_dart.value as usize)]));
    for c in &mut checkouts {
        if c.second_dart.multiplier==0 && c.first_dart.multiplier >0
        {
            swap(&mut c.first_dart,&mut c.second_dart);
        }
    }

checkouts.sort();
    checkouts.dedup();
    checkouts.sort_by(|a,b| a.first_dart.multiplier.cmp(&b.first_dart.multiplier));
    checkouts.sort_by(|a,b| a.second_dart.multiplier.cmp(&b.second_dart.multiplier));

    for c in &checkouts {
        println!("{}",c);
    }
    println!("Found {} matches",checkouts.len());
}
