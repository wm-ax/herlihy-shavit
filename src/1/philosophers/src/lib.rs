// 2840
use std::sync::Arc;
use std::thread;
use rand::rngs::ThreadRng;
// use rand::distributions::Uniform;
use rand::{Rng};
use try_lock::TryLock;
use std::time;
use std::sync::atomic::{AtomicBool, Ordering};


const TABLE_SIZE: usize = 5;


pub fn dine(duration: u64) {
    let chopsticks = (0..TABLE_SIZE).map(
        |_| Arc::new(TryLock::new(Chopstick))
    ).collect::<Vec<_>>();
    let stopped = Arc::new(AtomicBool::new(false));
    let philosophers : Vec<_> = (0..TABLE_SIZE).map(|id| {
        let chopsticks = [chopsticks[id].clone(),
                          chopsticks[(id + 1) % TABLE_SIZE].clone()];
        let stopped = stopped.clone();
        thread::spawn(|| {
            let mut philosopher = Philosopher::new(chopsticks, stopped);
            // println!("initialized a philosopher"); 
            philosopher.dine();
        })
    }).collect();
    thread::sleep(time::Duration::from_millis(duration));
    stopped.store(true, Ordering::SeqCst);
    for thr in philosophers {
        thr.join().unwrap();
    }
}


struct Philosopher {
    chopsticks: [Arc<TryLock<Chopstick>>; 2],
    state: State,
    timer: Timer,
    amount_eaten: u64,
    stopped: Arc<AtomicBool>,
}


impl Philosopher {

    fn new(chopsticks: [Arc<TryLock<Chopstick>>; 2],
           stopped: Arc<AtomicBool>) -> Self {
        Philosopher {
            state: State::Thinking,
            chopsticks: chopsticks.clone(),
            timer: Timer::new(),
            amount_eaten: 0,
            stopped: stopped.clone(),
        }
    }

    pub fn dine(&mut self) {
        println!("philosopher on {:?} is dining", thread::current().id());
        while !self.is_done() {
            match self.state {
                State::Eating => 
                    self.think(),
                State::Thinking => {
                    self.wait()
                }
                State::Waiting => 
                    self.try_to_eat()
            }
        }
        println!("Philosopher on {:?} done, having eaten {} grains of rice",
                 thread::current().id(),
                 self.amount_eaten);
    }

    fn try_to_eat(&mut self) {
        let chopsticks = self.chopsticks.into_iter()
            .map(
                |chopstick| chopstick.try_lock()
            )
            .collect::<Option<Vec<_>>>();
        match chopsticks {
            Some(_) => {
                self.state = State::Eating;
                let amount = self.timer.sleep_random();
                self.amount_eaten += amount;
            },
            _ => {},
        }
    }

    fn think(&mut self) {
        self.state = State::Thinking;
        self.timer.sleep_random();
    }

    fn wait(&mut self) {
        self.state = State::Waiting;
    }

    fn is_done(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }

}


struct Timer {
    rng: ThreadRng,
}


impl Timer {
    fn new() -> Self {
        Timer {rng: rand::thread_rng()}
    }

    fn sleep_random(&mut self) -> u64 {
        let num_millis = self.rng.gen_range(0, 10);
        let random_duration = time::Duration::from_millis(
            num_millis);
        thread::sleep(random_duration);
        num_millis
    }
}


enum State {
    Eating,
    Thinking,
    Waiting,
}


struct Chopstick;

