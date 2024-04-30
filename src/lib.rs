mod attack;
mod placement_stats;
mod player_stats;
use placement_stats::CumulativePlacementStats;
use player_stats::PlayerStats;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
mod board_analyzer;
mod replay_response;
mod solver;
use replay_response::PlacementStats;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    task::JoinSet,
};

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn analyze(arr: *mut *mut c_char, size: usize) -> *const libc::c_char {
    let mut cumulative_stats = CumulativePlacementStats::default();
    let slice = unsafe { std::slice::from_raw_parts(arr, size) };
    for ptr in slice {
        let c_str = unsafe { CStr::from_ptr(*ptr) };
        let rust_string = c_str.to_string_lossy();
        let placements: Vec<PlacementStats> = serde_json::from_str(&rust_string).unwrap(); //something went wrong in the response loop, error should never happen
        cumulative_stats.absorb(CumulativePlacementStats::from(placements.as_slice()));
        //join all handles and their respective stats

        //  println!("{}", rust_string);
    }

    let stats = PlayerStats::from(&cumulative_stats);

    let result_json = serde_json::to_string(&stats).unwrap();
    std::ffi::CString::new(result_json).unwrap().into_raw()
    /*
    let c_json = unsafe { std::ffi::CStr::from_ptr(json) };
    let rust_json = c_json.to_str().unwrap();

    for placements in &placementsList {
        let result: CumulativePlacementStats =
            CumulativePlacementStats::from(placements.as_slice());
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
