
// fn average(x:f64, y:f64)->f64{
//     return (x+y)/2.0;
// }
// #[derive(Clone)]
// #[derive(Copy)]
// struct Rectangle{
//     length: f64,
//     width: f64,
// }

// fn perimeter(structure:Rectangle)->f64{
//     (structure.length+structure.width)*2.0
// }
// fn perimeter2(structure: &Rectangle)->f64{
//     return (structure.length+structure.width)*2.0;
// }
fn print_references(){
    let x = 18;
    let y = 20;
    // let ref1 = &x;
    // let ref2 = &y;
    let mut ref3 =  x;
    let mut ref4 = y;
    // println!("ref1 : {:p}",ref1);
    // println!("ref2 : {}",ref2);
    println!("ref3 : {}",ref3);
    println!("ref4 : {}",ref4);
}
fn swap(a:f64,b:f64){
    let mut ref1 = a;
    let mut ref2 = b;

    ref1=b;
    ref2=a;
    println!("a = {}",ref1);
    println!("b = {}",ref2);

}
fn main() {
    let x = 2.0;
    let y = 4.0;
    // let x=4.0;
    // let y=2.0;
    // let my_rectangle = Rectangle{
    //     length:2.0,
    //     width:3.0,
    // };
    println!("Hello, world!");
    // print!("{}\n",average(x,y));
   
    // // print!("{}\n",average(x,y));
    // print!("{}\n",perimeter2(&my_rectangle));
    // print!("{}\n",perimeter(my_rectangle.clone()));
    // print!("{}\n",perimeter(my_rectangle.clone()));
    // print!("{}\n",perimeter(my_rectangle));
    // print!("{}\n",perimeter(my_rectangle));
    print_references();
    swap(x,y);
    swap(x,y);
    
  
}
