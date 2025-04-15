pub struct Solution;

impl Solution {

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        
        let (mut left, mut right) = (0, nums.len() - 1);
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid] == target {
                return mid as i32;
            }
            
            // Check if the left half is sorted
            if nums[left] <= nums[mid] {
                // If target is in the left sorted half
                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } 
            // Right half is sorted
            else {
                // If target is in the right sorted half
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        
        -1 // Target not found
    }

    // https://excalidraw.com/#json=iT8mYhX2FZbzCyhwz4ZMG,UZj6jt-NIft1FSBBN2Brrw
    pub fn search_initial(nums: Vec<i32>, target: i32) -> i32 {
        // println!("nums={nums:?}");
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let m = l + (r - l ) / 2;
            // println!("l={l} r={r} m={m}");
            if nums[m] == target {
                return m as i32;
            }
            if nums[m] > target {
                if nums[m] > nums[r] { // left
                    if target > nums[r] {
                        if m > 0 {
                            r = m - 1;
                        } else {
                            break;
                        }
                    } else {
                        l = m + 1;
                    }
                } else {
                    if m > 0 {
                        r = m - 1;
                    } else {
                        break;
                    }
                }
            } else {
                if nums[m] > nums[r] { // left
                    l = m + 1;
                } else {
                    if target > nums[r] {
                        if m > 0 {
                            r = m - 1;
                        } else {
                            break;
                        }
                    } else {
                        l = m + 1;
                    }
                }
            }
        }
        return -1;
    }
}
