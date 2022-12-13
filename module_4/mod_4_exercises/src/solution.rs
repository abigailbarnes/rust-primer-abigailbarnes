use std::f64::consts::PI;
use std::thread;
use std::sync::{Arc, RwLock, Mutex};

pub trait Geometry 
{
    fn get_area(&self) -> f64; // return area of shape
    fn get_name(&self) -> String; // return name of shape
}

pub struct Rectangle 
{
    pub length: f64,
    pub width: f64,
}

impl Geometry for Rectangle 
{
    fn get_area(&self) -> f64
    {
        let mut area = self.length * self.width;
        area = area*10.0;
        area = area.round();
        area = area/10.0;
        return area;
    }

    fn get_name(&self) -> String
    {
        let name = "Rectangle";
        return name.to_string();
    }

}


pub struct Circle 
{
    pub radius: f64,
}

//using PI from previous import statement
impl Geometry for Circle 
{
    fn get_area(&self) -> f64 // return area of shape
    {
        
        let mut area = self.radius * self.radius;
        area = area * PI;
        return area;
    }

    fn get_name(&self) -> String // return name of shape   
    {
        let name = "Circle";
        return name.to_string();
    }
}


//PROBLEM 2

struct Counter {
    count: i32
}

// define a function incr() to increment count in Counter by 1
//make mutable so we can change the value
//parameter variable must be of a Counter type
fn incr(count_var: &mut Counter) 
{
    //get count value within the struct var
    count_var.count = count_var.count + 1;
}
 
fn counter() {
    // declare a counter
    let counter = Arc::new(Mutex::new(Counter{count: 0}));

    //clone for multiple thread use
    let cln = counter.clone();


    // spawn a thread here to call incr() 50 times
    let handle = thread::spawn(move|| {
        for _i in 1..50 {
            let mut data = cln.lock().unwrap();
            incr(&mut data);
            println!("thread spawned count {}", data.count);
        }
    });

    // in the main thread, call incr() 50 times
    for _i in 1..50 
    {
        let mut data1 = counter.lock().unwrap();
        incr(&mut data1);
        println!("thread main count {}", _i);
    }
    handle.join().unwrap();
}


//PROBLEM 3
/*
 *  Modify the following function to use RwLocks. Run both blocks, one with standard mutexes
 *  and the other with RwLocks. What differences do you observe in their behavior/output? Does
 *  this match your understanding of how Read/Write locks work?
 */
fn read_write() {
    let lock = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(10);

    for _i in 0..10 {
        let reader_lock = lock.clone();
        let reader = thread::spawn(move || {
            for _j in 0..20 {
                let r = reader_lock.lock().unwrap();
                println!("Read value as {}", *r);
            }
        });
        handles.push(reader)
    }

    for _j in 0..20 {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("Incremented value by 1 to {}", *val);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
