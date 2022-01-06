// https://leetcode.com/problems/car-pooling/

#[derive(Debug)]
struct Location {
    x: i32,
    trip_id: usize,
}

struct Solution;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut passengers: i32 = trips[0][0];
        
        // fill in data structure
        let mut start_locations: Vec<Location> = Vec::new();
        let mut end_locations: Vec<Location> = Vec::new();
        for i in 0..trips.len() {
            start_locations.push(Location {
                x: trips[i][1],
                trip_id: i ,
            });
            end_locations.push(Location {
                x: trips[i][2],
                trip_id: i,
            });
        }

        // sort start and end locations
        start_locations.sort_by(|a, b| a.x.cmp(&b.x));
        end_locations.sort_by(|a, b| a.x.cmp(&b.x));
        print!("sorted start_locations: {:#?}", start_locations);
        print!("sorted end_locations: {:#?}", end_locations);

        return true;
        
    }
}

fn print_test(result: bool, expected: bool) {
    unsafe {
        if result == expected {
            println!("success: test[{}], result: {}, expected: {}", TEST_NUMBER, result, expected);
        } else {
            println!("failure: test[{}], result: {}, expected: {}", TEST_NUMBER, result, expected);
        }
        TEST_NUMBER += 1;
    }
}

static mut TEST_NUMBER: i32 = 0;

fn main() {
    // test cases
    print_test(Solution::car_pooling(vec![vec![2,1,5],vec![3,3,7]], 4), false);
    print_test(Solution::car_pooling(vec![vec![2,1,5],vec![3,3,7]], 5), true);
    print_test(Solution::car_pooling(vec![vec![2,1,5],vec![3,3,7]], 0), false);
    print_test(Solution::car_pooling(vec![vec![2,1,5],vec![3,3,7]], 1), false);
}


