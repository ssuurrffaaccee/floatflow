use floatflow::error::{FFError, Result};
use floatflow::executor::Executor;
use floatflow::graph::NodeAllocator;
use floatflow::optimizer::{AdaGrad, Adam, Optimizer, SGD};
use floatflow::polynomial::{Polynomial, Term};
use floatflow::store::Store;
use std::collections::HashMap;

#[derive(Debug)]
struct PolynomialConfig {
    multi_poly: Vec<Polynomial>,
    init: HashMap<String, f32>,
}
impl PolynomialConfig {
    pub fn new() -> Self {
        PolynomialConfig {
            multi_poly: Vec::new(),
            init: HashMap::new(),
        }
    }
    pub fn parse(&mut self, string: &String) -> Result<()> {
        for line in string.trim().lines() {
            let words = line
                .split('@')
                .map(|s| String::from(s))
                .collect::<Vec<String>>();
            println!("{:?}", words);
            let mut iter = words.into_iter().peekable();
            let token = iter
                .peek()
                .ok_or(FFError::Parse(format!("parse fiailed!!!")))?;
            if token == "i" {
                //skip i
                iter.next()
                    .ok_or(FFError::Parse(format!("parse fiailed!!!")))?;
                while let Some(_) = iter.peek() {
                    //parse name
                    let name = iter
                        .next()
                        .ok_or(FFError::Parse(format!("parse fiailed!!!")))?;
                    let init = iter
                        .next()
                        .ok_or(FFError::Parse(format!("parse fiailed!!!")))?
                        .parse::<f32>()?;
                    self.init.insert(name, init);
                }
                // parse_init(&mut iter)?
            } else if token == "c" {
                let mut poly = Polynomial::new();
                while let Some(_) = iter.peek() {
                    let mut t = Term::new();
                    //skip c
                    iter.next()
                        .ok_or(FFError::Parse(format!("parse fiailed!!!")))?;
                    //parse coefficient
                    t.set_coefficient(
                        iter.next()
                            .ok_or(FFError::Parse(format!("parse fiailed!!!")))?
                            .parse::<f32>()?,
                    );
                    if iter.peek().is_none() {
                        println!("{:?}", t);
                        poly.add_term(t);
                        break;
                    }
                    //parse name
                    let name = iter
                        .next()
                        .ok_or(FFError::Parse(format!("parse fiailed!!!")))?;
                    //parse order
                    let order = iter
                        .next()
                        .ok_or(FFError::Parse(format!("parse fiailed!!!")))?
                        .parse::<usize>()?;
                    t.add_variable(&name, order);
                    println!("{:?}", t);
                    poly.add_term(t);
                }
                self.multi_poly.push(poly)
            }
        }
        return Ok(());
    }
}
//fn main() -> std::io::Result<()> {
fn main() -> Result<()> {
    // x+y = 3,x-y = 1
    let context =
        String::from("i@x@4.0@y@1.0\nc@1.0@x@1@c@1.0@y@1@c@-3.0\nc@1.0@x@1@c@-1.0@y@1@c@-1.0");
    println!("\ncontext {}", context);
    let mut polynomial_onfig = PolynomialConfig::new();
    polynomial_onfig.parse(&context)?;
    println!("\npolynomial_onfig {:?}", polynomial_onfig);

    let mut allocator = NodeAllocator::new();
    let mut name_record = HashMap::<String, usize>::new();
    let mut execs = polynomial_onfig
        .multi_poly
        .iter()
        .map(|poly| {
            let graph = poly.tranform_to_graph(&mut name_record, &mut allocator);
            println!("\ngraph {:?}", graph);
            //let mut exec = Executor::new(graph,Optimizer::Adam(Adam::new(0.01,0.9,0.99)));
            let exec = Executor::new(graph, Optimizer::SGD(SGD::new(0.01)));
            return exec;
        })
        .collect::<Vec<Executor>>();
    let mut s = Store::new();
    for exec in execs.iter_mut() {
        exec.init_parameter(&polynomial_onfig.init, &mut s)?;
        exec.init_contants(&mut s);
    }
    for _ in 0..200 {
        for exec in execs.iter_mut() {
            exec.forward(&mut s)?;
            exec.backward(&mut s)?;
            exec.optimize(&mut s)?;
            exec.clear_grad(&mut s)?;
            let f = exec.get_output_value(&mut s);
            println!("\nloss {:?}", f);
        }
        let mut parameters = Vec::new();
        execs.iter_mut().for_each(|exec| {
            exec.get_parameters(&mut s)
                .into_iter()
                .for_each(|e| parameters.push(e));
        });
        println!("\npara {:?}", parameters);
    }
    Ok(())
}
