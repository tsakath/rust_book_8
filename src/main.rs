use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let v = vec![1, 1, 1, 2, 3, 4, 5, 6, 3, 4, 4, 3];

    match median(&v) {
        Some(res) => println!("Median {}", res),
        None => println!("Cannot get median for {:?}", v),
    }
    match mode(&v) {
        Some(res) => println!("Mode {}", res),
        None => println!("Cannot get mode for {:?}", v),
    }
}

// media returns the median of a list
fn median(vec: &Vec<i32>) -> Option<f64> {
    if vec.len() == 0 {
        return None;
    }
    let v1 = *vec.get(vec.len() / 2).unwrap_or(&0) as f64;
    if vec.len() % 2 != 0 {
        Some(v1)
    } else {
        let v2 = *vec.get(vec.len() / 2 - 1).unwrap_or(&0) as f64;
        Some((v1 + v2) / 2.0)
    }
}

// mode returns the mode of a list
// If there are multiple modes the first occurrence is returned
fn mode(vec: &Vec<i32>) -> Option<i32> {
	if vec.len() == 0 {
		return None;
	}
	let mut reg = HashMap::new();
	let mut freq: i32 = 0;
	let mut max: i32 = 0;
	for v in vec.iter() {
		let count = reg.entry(v).or_insert(0);
		*count+=1;
		if *count > max {
			max = *count;
			freq = *v;
		}
	}	
	Some(freq)
}
