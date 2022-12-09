use array2d::Array2D;
use std::cmp::max;

fn is_visible(arr: &Array2D<u32>, x: usize, y: usize) -> bool {
    let mysize = arr[(y,x)];
    return
        (0..x).all(|xp| arr[(y,xp)] < mysize) ||
        (x+1..arr.num_columns()).all(|xp| arr[(y,xp)] < mysize) ||
        (0..y).all(|yp| arr[(yp,x)] < mysize) ||
        (y+1..arr.num_rows()).all(|yp| arr[(yp,x)] < mysize);
}

fn count_visible(arr: &Array2D<u32>) -> usize {
    let mut count = 0;
    for y in 0..arr.num_rows() {
        count += (0..arr.num_columns()).filter(|x| is_visible(&arr, *x, y)).count();
    }
    return count;
}

fn view_score(arr: &Array2D<u32>, x: usize, y: usize) -> usize {
    if x == 0 || y == 0 || x == arr.num_columns() - 1 || y == arr.num_rows() - 1 {
        return 0; // Score on border is always 0
    }
    let mysize = arr[(y,x)];
    let west_view = (0..x).rev().position(|xp| arr[(y,xp)] >= mysize).unwrap_or(x-1)+1;
    let east_view = (x+1..arr.num_columns()).position(|xp| arr[(y,xp)] >= mysize).unwrap_or(arr.num_columns()-x-2)+1;
    let north_view = (0..y).rev().position(|yp| arr[(yp,x)] >= mysize).unwrap_or(y-1)+1;
    let south_view = (y+1..arr.num_rows()).position(|yp| arr[(yp,x)] >= mysize).unwrap_or(arr.num_rows()-y-2)+1;
    return west_view * east_view * north_view * south_view;
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
    println!("Star 1: {}", count_visible(&forest)); 
}

pub fn star2(filename: &str) {
    let forest = read_forest(filename);
    println!("Star 2: {}", best_view_score(&forest)); 
}
