use clap::Parser;
use std::thread;
#[derive(Parser)]
struct Paramaters{
    n: usize
}

fn main(){
    let mut handles = vec![];
    let my_parameters=Paramaters::parse();
    let message = "Bonjour";
for i in 0..my_parameters.n{
    let handle = thread::spawn(move ||{ 
    println!("{} n° {}",message,i);
    println!("Aurevoir n° {}",i);
    });
     
    handles.push(handle);
}
    for handle in handles {
        handle.join().unwrap();
    }
}

