use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};

mod random;



// pub fn spawn<T , F>(f : F)-> JoinHandle<T>
// where 
//     F : FnOnce()-> T + Send + 'static,
//     T : Send + 'static
// {

// }
trait FnBox {
    fn call_box(self :Box<Self>);
}

impl<F : FnOnce()> FnBox for F {
    fn call_box(self : Box<F>){
        (*self)()
    }
}


type Job =Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job), 
    Terminate
}
struct Worker{
    id : usize, 
    thread : Option<JoinHandle<()>>
}
pub struct ThreadPool{
    workers : Vec<Worker>, 
    sender : Sender<Message>
}

impl Worker {
    fn new(id : usize , rx : Arc<Mutex<Receiver<Message>>> ) ->Self{
        Worker{
            id , 
            thread : Some(thread::spawn(move ||{
                loop{
                    let message = rx.lock().unwrap().recv().unwrap();
                    match message {
                        Message::NewJob(job)=>{

                            println!("Worker {} go a job; executing" ,id);
                            job.call_box();
                        } , 
                        Message::Terminate =>{
                            println!("Worker {} terminating" , id);
                            break;
                        }
                    }

                }
                
            }))
        }
    }
}

impl ThreadPool {
    pub fn new(size : usize ) -> Self{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (tx , rx)= channel();
        let receiver = Arc::new(Mutex::new(rx));
        for _ in 0..size{
            let id = random::random_number(1, 10000);
           let new_worker = Worker::new(id as usize , Arc::clone(&receiver));
           workers.push(new_worker);
        }

        ThreadPool{
            workers,
            sender : tx
        }
    }
    pub fn execute<F>(&self , f : F)
    where F : FnOnce() + Send +'static{
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for  ThreadPool {
    fn drop(&mut self) {
        println!("Sending the terminate message to workers");

        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");
       for worker in &mut self.workers{

        println!("shutting down worker {}" , worker.id);
        if let Some(thread) = worker.thread.take(){
            thread.join().unwrap();
            
        }
       
    }
    }
}