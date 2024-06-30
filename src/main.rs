// A method to return the medium of two unsorted arrays as the medium of a sorted array. 

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut vec_medium  = [nums1, nums2].concat();
        vec_medium.sort();
        let vec_len = vec_medium.len();
        let  mid = vec_len / 2;

        if vec_len % 2 == 0{
            let right = mid - 1;
            let left = mid;
            
            return ((vec_medium[left] + vec_medium[right])as f64)/ 2.0            
        }
        else {
            return (vec_medium[mid]) as f64
           
        }
    }
}


fn main(){
    let medium = Solution::find_median_sorted_arrays(vec![1,2],vec![3,4]);

    println!("{}",medium);
}