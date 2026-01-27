use std::collections::HashMap;

enum location {
    end,
    mid(arg_insert)
}

struct arg_insert{
    index: usize,
    number: i64,
}

fn main() {
    
    let test: [i64;7] = [5,4,3,3,3,2,1];
    let med_ans = median(&test);
    let mod_ans = mode(&test);
    println!("median: {} mode: {}" ,med_ans,mod_ans);
}

fn mode(arr: &[i64]) -> i64 {
    let mut counts = HashMap::new();
    for num in arr.iter().enumerate() {
        let (i,x): (usize, &i64) = num;
        let count = counts.entry(x).or_insert(0);
        *count += 1;
    }
    println!("{:?}",counts);
    let mut max = 0;
    let mut answer: i64 = 0; 
    for (&key, &val) in &counts {
        if val > max {
            max = val;
            answer = *key;
        }
    }
    return answer;
}

fn median(arr: &[i64]) -> i64 {
    let mut sorted: Vec<i64> = sort(arr);
    if sorted.len() % 2 != 0 {
        let index = (sorted.len() - 1) / 2;
        let answer = sorted[index];
        return answer;
    }
    else {
        let index = (sorted.len()/2);
        let answer = sorted[(index + index - 1)/2];
        return answer;
    }
}

fn sort(arr:&[i64]) -> Vec<i64> {
    let mut sorted: Vec<i64> = Vec::new();
    for num in arr.iter().enumerate() {
        let (i,x): (usize, &i64) = num;
        let check_empty = sorted.is_empty();
        let mut args = location::end;
        
        if check_empty == false {            
            for item in &sorted {                
                let index = sorted.iter().position(|&y| y == *item).unwrap();
                let length = sorted.len() - 1;
                if x <= &item {
                    args = location::mid(arg_insert{index: index,number: *x});
                    break;
                }
                else {
                    if length != index {
                        if *x <  sorted[index + 1]{
                            args = location::mid(arg_insert{index: index+1,number: *x});
                            break;
                        }
                    }
                }
            }
            match args {
                location::mid(arg_insert) => sorted.insert(arg_insert.index,arg_insert.number),
                location::end => sorted.push(*x),
            }  
        }
        else {
            sorted.push(*x);
        }
    }
    return sorted
}