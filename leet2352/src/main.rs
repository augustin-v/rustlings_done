fn equal_pairs(grid:Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut count = 0;
    for i in 0..len {
        for j in 0..len {
            let mut match_found = true;
            for k in 0..len {
                if grid[i][k] != grid[k][j] {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                count +=1
            }
        }
    }
    count
}