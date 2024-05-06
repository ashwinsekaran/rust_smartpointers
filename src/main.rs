//Box//

// fn main()  
// {  
//   let a = 20;  
//   //let b = &a; 
//   let b = Box::new(a);  
//   let c= *b;
//   let d= Box::new(c);  
//   //print!("Value of b is {}",b);   
//   if a==*b  
//   {  
//     println!("a and *b are equal");  
//   }  
    
//   else  
//   {  
//     println!("they are not equal");  
//   }  
//   println!("{} {}",c, d);
  
// }  


//My box example for <T> issue//
// struct MyBox<T>(T);  
// impl<T> MyBox<T>  
// {  
//   fn example(y : T)->MyBox<T>  
//   {  
//     MyBox(y)  
//   }  
// }  
// fn main()  
// {  
//   let a = 8;  
//   let b = MyBox::example(a);  
//   print!("Value of *b is {}",*b);  
// }  

// box issue with tuple - without dref trait
// solving using dref trait - with dref trait
// dref coercion - passing argument as reference of box & print function

// struct MyBox<T>  
// {  
//   a : T,  
// }  
// use :: std::ops::{Deref, DerefMut};
// impl<T> Deref for MyBox<T>  
// {  
//   type Target = T;  
//   fn deref(&self) ->&T  
//   {  
//     &self.a  
//   }  
// }  
// // impl<T> DerefMut for MyBox<T> {
// //     fn deref_mut(&mut self) -> &mut Self::Target {
// //         &mut self.a
// //     }
// // }
// // fn print(m : &i32)  
// // {  
// //   print!("{}",m);  
// // }  
// fn main()  
// {  
//   let b = MyBox{a : 2};  
//   //*b = 3;
//   println!("{}",*b);  
//   //print(&b);
// } 

//drop trait - for unmanaged code which rust cannot call destructor - especially for n/w and files (using std::mem::drop)
// struct Example  
// {  
//   a : String,  
// }  
  
// impl Drop for Example  
// {  
//   fn drop(&mut self)  
//   {  
//     println!("Dropping the instance of Example with data : {}", self.a);  
//   }  
// }  
  
// fn main()  
// {  
//   let a1 = Example{a : String::from("Hello")};  
//   a1.drop();  
//   let b1 = Example{a: String::from("World")};  
//   println!("Instances of Example type are created");  
// }  

//Rc<T> - multiple data owners/share the data
//using ref count - release memory once moved out of scope
// enum List   
// {  
//   //Ashwin(i32, Box<List>), 
//   Ashwin(i32, Rc<List>),   
//   NA,  
// }  
// use List::{Ashwin,NA};  
// use std::rc::Rc;  
// fn main()  
// {  
// //   let a = Ashwin(10, Box::new(Ashwin(15,Box::new(NA))));  
// //   let b = Ashwin(2, Box::new(a));  
// //   let c = Ashwin(1, Box::new(a));   
//   let a = Rc::new(Ashwin(10, Rc::new(Ashwin(15,Rc::new(NA)))));  
//   println!("Reference count after creating a List : {}", Rc::strong_count(&a)); 
//   let b = Ashwin(2, a.clone());  
//   println!("Reference count after creating b List : {}", Rc::strong_count(&a));  
//   {
//   let c = Ashwin(1, a.clone());   
//   println!("Reference count after creating c List : {}",Rc::strong_count(&a));  
//   }  
//   println!("Reference count when c goes out of the scope : {}",Rc::strong_count(&a));  
// }  

//Arc - thread safe to run in multiple threads
// #![allow(dead_code)]
// #![allow(unused_variables)]
// use std::rc::Rc;
// use std::sync::Arc;
// use std::thread;

// struct Person
// {
//   name: Arc<String>
// }

// impl Person 
// {
//   fn new(name: Arc<String>) -> Person 
//   {
//     Person {name: name}
//   }
//   fn greet(&self)
//   {
//     println!("Hello {}", self.name)
//   }
// }
// fn main(){
//   arc();
// }

// fn arc(){
//   let name = Arc::new("ashwin".to_string());
//   let person = Person::new(name.clone());

//   let t=thread::spawn(move || {
//     person.greet();
//   });
//   println!("name is {}", name);

//   t.join().unwrap();
// }

//Ref cell - pattern used to mutate the ref if we have immut ref. Refcell<T> used to acheive interior mutability
//single ownership
//violating RUST borrowing rules using unsafe code using safeAPI
//borrows runtime and uses deref trait
//borrow() - uses smart pointer Ref<T>, many immutable borrows allowed
//borrow_mut() - uses RefMut<T>, only one mutable borrows allowed
//refcell<T> - keep count of ref and refmut pointers are active using borrow (once used increment by 1, out of scope will reduce by 1)
// fn main()  
// {  
//   let a = 15;  
//   let b = &mut a;  
// }  
// use std::cell::RefCell;  


// fn main()  
// {  
//   let a = RefCell::new(15);  
//   let b = a.borrow_mut();  
//   //let c =b.deref_mut();
//   //let c = a.borrow_mut();  
//   //let c = a.borrow_mut();  
//   println!("Value of b is : {}",b);  
//   //println!("Value of c is : {}",c);  
// }  

//combine RC and Ref  - Assign multiple owners for mut data
 #[derive(Debug)]  
enum List  
{  
 Cons(Rc<RefCell<String>>,Rc<List>),  
 Nil,  
}  
use List:: {Cons,Nil};  
use std::rc::Rc;  
use std::cell::RefCell;  
fn main()  
{  
  let val = Rc::new(RefCell::new(String::from(".net")));  
  let a = Rc::new(Cons(Rc::clone(&val),Rc::new(Nil)));  
  let b = Cons(Rc::new(RefCell::new(String::from("C"))),Rc::clone(&a));  
  let c = Cons(Rc::new(RefCell::new(String::from("C++"))),Rc::clone(&a));  
  *val.borrow_mut() = String::from("RUST Overwritten");  
  println!("value of a is : {:?}",a);  
  println!("value of b is : {:?}",b);  
  println!("value of c is : {:?}",c);  
}  