const PRESENT_THRESH: u32 = 29_000_000;
const ARRAY_CAP: usize = 50_000_000;

fn main() {
    let mut houses: Vec<u32> = Vec::with_capacity(ARRAY_CAP);
    let mut min_house: usize = usize::MAX;
    let mut got_answer = false;

    for _ in 0..ARRAY_CAP {
        houses.push(0);
    }

    for i in 1usize.. {
        let presents: u32 = 11 * i as u32;

        for j in 1..=50 {
            let elem = i as usize * j as usize;

            if got_answer && elem > min_house {
                if j == 1 {
                    println!("House {} gets {} presents", min_house, houses[min_house]);
                    return    
                }
            }

            let new_presents = houses[elem] + presents;

            if new_presents >= PRESENT_THRESH {
                if elem < min_house {
                    got_answer = true;
                    min_house = elem;
                }
            } else {
                houses[elem] = new_presents;
            }
        }
    }
}
