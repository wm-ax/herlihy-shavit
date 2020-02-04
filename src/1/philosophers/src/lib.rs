// 2840
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::{Sender};
use std::sync::mpsc;
use std::time;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::rngs::ThreadRng;
use rand::{Rng};
use try_lock::TryLock;

// starvation-free version

const TABLE_SIZE: usize = 5;


pub fn dine(duration: u64) {
    let mut state_receivers = Vec::new();
    let chopsticks = (0..TABLE_SIZE).map(
        |_| Arc::new(TryLock::new(Chopstick))
    ).collect::<Vec<_>>();
    let stopped = Arc::new(AtomicBool::new(false));
    let philosophers : Vec<_> = (0..TABLE_SIZE).map(|id| {
        let chopsticks = [chopsticks[id].clone(),
                          chopsticks[(id + 1) % TABLE_SIZE].clone()];
        let stopped = stopped.clone();
        let (sender, receiver) = mpsc::channel();
        state_receivers.push(receiver);
        thread::spawn(|| {
            let mut philosopher = Philosopher::new(chopsticks, stopped, sender);
            philosopher.dine();
        })
    }).collect();
    let num_checks = 10;
    for i in 0 .. num_checks {
        thread::sleep(time::Duration::from_millis(duration / num_checks));
        print!("Report {}: their states are ", i);
        let states : Vec<_> = state_receivers.iter().map(
            |rcv| rcv.try_iter().last().unwrap()).collect();
        println!("{:?}", states);
    }
    stopped.store(true, Ordering::SeqCst);
    for thr in philosophers {
        thr.join().unwrap();
    }
}


struct Philosopher {
    chopsticks: [Arc<TryLock<Chopstick>>; 2],
    state: State,
    timer: Timer,
    sender: Sender<State>,
    amount_eaten: u64,
    stopped: Arc<AtomicBool>,
}


impl Philosopher {

    fn new(chopsticks: [Arc<TryLock<Chopstick>>; 2],
           stopped: Arc<AtomicBool>,
           sender: Sender<State>) -> Self {
        Philosopher {
            state: State::Thinking,
            chopsticks: chopsticks.clone(),
            timer: Timer::new(),
            sender: sender,
            amount_eaten: 0,
            stopped: stopped.clone(),
        }
    }

    pub fn dine(&mut self) {
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
        println!("Philosopher {:?} done, having eaten {} grains of rice",
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
                self.sender.send(self.state).unwrap();
                let amount = self.timer.sleep_random();
                self.amount_eaten += amount;
            },
            _ => {},
        }
    }

    fn think(&mut self) {
        self.state = State::Thinking;
        self.sender.send(self.state).unwrap();
        self.timer.sleep_random();
    }

    fn wait(&mut self) {
        self.state = State::Waiting;
        self.sender.send(self.state).unwrap();
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


#[derive(Debug, Copy, Clone)]
enum State {
    Eating,
    Thinking,
    Waiting,
}


struct Chopstick;

