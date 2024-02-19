use clap::Parser;
use port_scanner::is_open_sync;
use std::time::Instant;
use std::thread;
#[derive(Parser)]
 struct Options{
    host: String,
    port_min:u16,
    port_max:u16,
    timeout:u64
}

#[tokio::main]
async fn main(){
    let mut handles = vec![];
    let instant= Instant::now();
    let my_parameters=Options::parse();
    let h = my_parameters.host;
for i in my_parameters.port_min..my_parameters.port_max+1{
    let handle = thread::spawn( move ||{ 
        let k=is_open_sync(h,i,my_parameters.timeout);
        println!("statut :{} port :{}",k,i);
    });
    
    handles.push(handle);
}
for handle in handles {
    handle.join().unwrap();
}
println!("{:?}",instant.elapsed());
}