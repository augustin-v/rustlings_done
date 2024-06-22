// fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
//     let mut res = vec![];
//     for asteroid in asteroids {
//         if asteroid > 0 {
//             res.push(asteroid);
//         } else {
//             while let Some(&last) = res.last() {
//                 if last < 0 {
//                     break;
//                 }
//                 if last == -asteroid {
//                     res.pop();
//                     break;
//                 } else if last > -asteroid {
//                     break;
//                 } else {
//                     res.pop();
//                 }
//             }
//             if res.is_empty() || res.last().unwrap() < &0 {
//                 res.push(asteroid);
//             }
//         }
//     }
//     res
// }

fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];

    for &asteroid in asteroids.iter() {
        let mut is_alive =true;
        while let  Some(&top) = stack.last() {
            if top > 0 && asteroid < 0 {
                if top < -asteroid {
                    stack.pop();
                    continue;
                } else if top == -asteroid {
                    stack.pop();
                }
                is_alive = false;
                break;
            }
            break;
        }
        if is_alive {
            stack.push(asteroid);
        }
    }
    stack
}