use clap::Parser;
use std::thread;
use std::sync::RwLock;
use std::time::Duration;
use std::sync::Arc;
#[derive(Parser)]
struct Paramaters{
    n: usize
}

fn main(){
    let x = Arc::new(RwLock::new(0));
    

    let my_parameters=Paramaters::parse();
    
for _i in 0..my_parameters.n{
    let mut w = x.write().unwrap();
    *w += 1;
    println!("{}",w);
};   
thread::sleep(Duration::from_secs(1));
}