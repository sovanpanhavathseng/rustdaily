
extern crate crossbeam;

// 
fn main(){

// create a crate vector from 0-1000 - then for each of them for the crossbeam scope
  let all_nums: Vec<_> = (0..1_000_u64).into_iter().collect();
    let mut results = Vec::new();
    
    crossbeam::scope(|scope| {
    for num in &all_nums {
    results.push(scope.spawn(move || num * num + num * 5 + 2500));
    }
});
    
    let final_result: u64d = results.into_iter().map(|res| res.join()).sum();
    println!("Final result:{}", final_result);
}
