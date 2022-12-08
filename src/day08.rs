use array2d::Array2D;
use std::cmp::max;

fn is_visible(arr: &Array2D<u32>, x: usize, y: usize) -> bool {
    let mysize = &arr[(y,x)];
    let west_blocked = (0..x).any(|xp| &arr[(y,xp)] >= &mysize);
    let east_blocked = (x+1..arr.num_columns()).any(|xp| &arr[(y,xp)] >= &mysize);
    let north_blocked = (0..y).any(|yp| &arr[(yp,x)] >= &mysize);
    let south_blocked = (y+1..arr.num_rows()).any(|yp| &arr[(yp,x)] >= &mysize);
    return !(west_blocked && east_blocked && north_blocked && south_blocked);
}

fn count_visible(arr: &Array2D<u32>) -> usize {
    let mut count = 0;
    for y in 0..arr.num_rows() {
        count += (0..arr.num_columns()).filter(|x| is_visible(&arr, *x, y)).count();
    }
    return count;
}

fn view_score(arr: &Array2D<u32>, x: usize, y: usize) -> usize {
    let mysize = &arr[(y,x)];
    let mut res = 1;
    // There would have been a much more elegant way if usize could be negative ...
    if let Some(west_view) = (0..x).rev().position(|xp| &arr[(y,xp)] >= &mysize) {
        res *= west_view+1;
    } else {
        res *= x;
    }
    if let Some(east_view) = (x+1..arr.num_columns()).position(|xp| &arr[(y,xp)] >= &mysize) {
        res *= east_view+1;
    } else {
        res *= arr.num_columns()-x-1;
    }
    if let Some(north_view) = (0..y).rev().position(|yp| &arr[(yp,x)] >= &mysize) {
        res *= north_view+1;
    } else {
        res *= y;
    }
    if let Some(south_view) = (y+1..arr.num_rows()).position(|yp| &arr[(yp,x)] >= &mysize) {
        res *= south_view+1;
    } else {
        res *= arr.num_rows()-y-1;
    }
    return res;
}

fn best_view_score(arr: &Array2D<u32>) -> usize {
    let mut best = 0;
    for y in 0..arr.num_rows() {
        best = max(best, (0..arr.num_columns()).map(|x| view_score(&arr, x, y)).max().unwrap());
    }
    return best;    
}

fn read_forest(filename: &str) -> Array2D<u32> {
    let lines = super::utils::read_lines(filename);
    return Array2D::from_rows(&lines.iter().map(
        |l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    ).collect::<Vec<Vec<u32>>>());
}

pub fn star1(filename: &str) {
    let forest = read_forest(filename);
    println!("{:?}", count_visible(&forest)); 
}

pub fn star2(filename: &str) {
    let forest = read_forest(filename);
    println!("Star 2: {}", best_view_score(&forest)); 
}
