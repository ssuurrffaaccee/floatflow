pub mod error;
pub mod executor;
pub mod float;
pub mod graph;
pub mod op;
pub mod ops;
pub mod optimizer;
pub mod polynomial;
pub mod store;
// enum Op{
//     Add(usize,usize,usize),
//     Minus(usize,usize),
//     Mul(usize,usize,usize),
// }

// struct Graph{
//     ops:Vec<Op>,
//     inputs:HashMap<String,usize>,
//     paremeters:HashMap<String,usize>,
//     outputs:usize,
// }
// impl Graph{
//     fn forward(s:&mut Store){
//         ops.iter().map(|op|{
//             match op{
//                 Op::Add(r,l,res)=> Add::forward(r,l,res,s),
//                 Op::Minus(r,res)=> Minus::forward(r,res,s),
//                 Op::Mul(r,l,res)=> Mul::forward(r,l,res,s),
//                 _ => panic!("unkown op"),
//             }
//         }
//        );
//     }

//     fn backward(s,&mut Store){
//         ops.iter().rev.map(|op|{
//             match op{
//                 Op::Add(r,l,res)=> Add::backward(r,l,res,s),
//                 Op::Minus(r,res)=> Minus::backward(r,res,s),
//                 Op::Mul(r,l,res)=> Mul::backward(r,l,res,s),
//                 _ => panic!("unkown op"),
//             }
//         }
//        );
//     }
//     fn feed(&self,input_values:&HashMap<String,f32>){

//     }
//     fn init_parameter(&self,parameter_values:&HashMap<String,f32>){

//     }
// }

// fn parameter(name:String,g:&mut Graph)->usize{
//     g.alloc_paremeter(name)
// }
// fn parameter(name:String,g:&mut Graph)->usize{
//     g.alloc_input(name)
// }

// fn add(r:usize,l:usize,g:&mut Graph)-> usize{
//     let res = g.alloc();
//     graph.add(Op::Add(r,l,res));
//     return res;
// }

// fn minus(r:usize,g:&mut Graph)-> usize{
//     let res = g.alloc();
//     graph.add(Op::Minus(r));
//     return res;
// }

// fn mul(r:usize,l:usize,g:&mut Graph)->usize{
//     let res = g.alloc();
//     g.add(Op::Mul(r,l,res));
//     return res;
// }

// fn test(){
//     let x = Box::new(Rc::new(RefCell::new(1)));
//     let y = x.clone();
//     {
//       let mut z = (**x).borrow_mut();
//       *z+=1;
//     }
//     println!("{:?}",x);
//     {
//         let mut z = (**y).borrow_mut();
//         *z+=1;
//     }
//     println!("{:?}",y)

// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
