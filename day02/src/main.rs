use memmap::Mmap;
use std::{cmp::min, fs::File, io::{BufRead, BufReader}};
use std::str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parcels = load_input("input02.txt")?;

    let paper: u32 = parcels.iter().map(|p| p.paper()).sum();

    println!("Total paper (part 1): {}", paper);

    let ribbon: u32 = parcels.iter().map(|p| p.ribbon()).sum();

    println!("Total ribbon (part 2): {}", ribbon);

    Ok(())
}

#[derive(Debug)]
struct Parcel {
    l: u32,
    h: u32,
    w: u32
}

impl Parcel {

    fn paper(&self) -> u32 {
        let s1 = self.l * self.w;
        let s2 = self.w * self.h;
        let s3 = self.h * self.l;

        let smallest = min(s1, min(s2, s3));

        (2 * s1) + (2 * s2) + (2 * s3) + smallest
    }

    fn ribbon(&self) -> u32 {
        let p1 = (2 * self.l) + (2 * self.w);
        let p2 = (2 * self.w) + (2 * self.h);
        let p3 = (2 * self.h) + (2 * self.l);

        let smallest = min(p1, min(p2, p3));

        smallest + (self.l * self.w * self.h)
    }

}

#[test]
fn test_parcel() {
    let p1 = Parcel {
        l: 2,
        h: 3,
        w: 4
    };

    let paper1 = p1.paper();
    assert!(paper1 == 58, "Paper should be 58 (not {})", paper1);

    let ribbon1 = p1.ribbon();
    assert!(ribbon1 == 34, "Ribbon should be 34 (not {})", ribbon1);

    let p2 = Parcel {
        l: 1,
        h: 1,
        w: 10
    };

    let paper2 = p2.paper();
    assert!(paper2 == 43, "Paper should be 43 (not {})", paper2);

    let ribbon2 = p2.ribbon();
    assert!(ribbon2 == 14, "Ribbon should be 14 (not {})", ribbon2);
}

fn load_input(file: &str) -> Result<Vec<Parcel>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create parcels vector
    let mut parcels = Vec::new();

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line != "" {
            let dim: Vec<_>  = line.split("x").map(|val| val.parse::<u32>().unwrap()).collect();

            parcels.push(Parcel {
                l: dim[0],
                h: dim[1],
                w: dim[2]
            })
        }
    }

    Ok(parcels)
}
