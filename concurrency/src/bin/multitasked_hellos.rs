
use clap::Parser;

#[derive(Parser)]
struct Paramaters{
    n: usize
}

#[tokio::main]
async fn main(){
    let mut handles = vec![];
    let my_parameters=Paramaters::parse();
for i in 0..my_parameters.n{
    let handle = tokio::spawn(async move{ 
    println!("Bonjour n° {}",i);
    println!("Aurevoir n° {}",i);
    });
     
    handles.push(handle);
}
    for handle in handles {
        handle.await;
    }
}
