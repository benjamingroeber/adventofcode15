use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    if let Err(e) = run() {
        eprintln!("FATAL ERROR: {}", e);
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let presents: Vec<Present> = input.lines().map(|line|FromStr::from_str(line)).collect::<Result<_,_>>()?;

    let total_area: usize = presents.iter().map(|p|p.paper_needed()).sum();
    println!("Total area of paper needed is {}", total_area);

    let total_ribbon: usize = presents.iter().map(|p|p.ribbon_needed()).sum();
    println!("Total length of ribbon needed is {}", total_ribbon);
    Ok(())
}

#[derive(Copy,Clone,Debug,Eq, PartialEq)]
struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn side_areas(&self) -> [usize; 3] {
        [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
    }

    fn side_circumference(&self) -> [usize;3] {
        [
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ]
    }

    // surface plus slack equal to the smallest side area
    fn paper_needed(&self) -> usize {
        let sides = self.side_areas();
        let surface = sides.iter().sum::<usize>() * 2;
        let slack = sides.iter().min().expect("Minimum slack always exists");
        surface + slack
    }

    // smallest perimeter side plus bow equal to the volume
    fn ribbon_needed(&self) -> usize {
        let sides = self.side_circumference();
        let smallest_circumference = sides.iter().min().expect("Minimum circumference always exists");
        let bow = self.length * self.width * self.height;
        smallest_circumference + bow
    }
}

impl FromStr for Present {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let sides: Vec<_> = s
            .split('x')
            .map(|number| number.parse::<usize>())
            .collect::<Result<_,_>>()?;

        if sides.len() != 3 {
            return Err(From::from("Could not parse Present dimensions!"));
        }

        Ok(Present {
            length: sides[0],
            width: sides[1],
            height: sides[2],
        })
    }
}

#[test]
fn test_parse_dimensions() {
    let present: Present = FromStr::from_str("1x2x3").unwrap();
    assert_eq!(Present{
        length: 1,
        width: 2,
        height: 3
    }, present)
}

#[test]
fn test_paper_needed() {
    let presents = &[
        (
            Present {
                length: 2,
                width: 3,
                height: 4,
            },
            58,
        ),
        (
            Present {
                length: 1,
                width: 1,
                height: 10,
            },
            43,
        ),
    ];
    for (present, expected) in presents {
        assert_eq!(*expected, present.paper_needed());
    }
}

#[test]
fn test_ribbon_needed() {
    let presents = &[
        (
            Present {
                length: 2,
                width: 3,
                height: 4,
            },
            34,
        ),
        (
            Present {
                length: 1,
                width: 1,
                height: 10,
            },
            14,
        ),
    ];
    for (present, expected) in presents {
        assert_eq!(*expected, present.ribbon_needed());
    }
}