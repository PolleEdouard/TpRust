use clap::Parser;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
#[derive(Parser)]
struct Paramaters{
    n: usize
}

fn main(){
    let x = Arc::new(12345);
    

    let my_parameters=Paramaters::parse();
    
for i in 0..my_parameters.n{
    let xclone = x.clone();

    println!("{}\n",xclone);

 
};   thread::sleep(Duration::from_secs(1));
}