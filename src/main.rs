impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut vec_medium  = [nums1, nums2].concat();
        vec_medium.sort();
        let vec_len = vec_medium.len();
        let mut mid = vec_len / 2;

        if vec_len % 2 == 0{
            let right = mid - 1;
            let left = mid;
            
            return ((vec_medium[left] + vec_medium[right])as f64)/ 2.0;            
        }
        else {
            return (vec_medium[mid]) as f64
           
        }
    }
}


fn main(){
    let medium = Solution::find_median_sorted_arrays([1,2,3],[4,5,6]);

    println!(medium);
}