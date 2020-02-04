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
    // each phil has a left and a right can.
    // each time he eats, he knocks over his own left and right cans, and sets up the right, left cans of his left, right neighbors (in that order)
// To see that this is starvation free, suppose that p1 finishes eating for the last time at t.
// first note that the following holds of all philosophers:
// (*) it is always true that at some time in the future, his right can will be in a different state from his right neighbor's left can, and likewise with left, right reversed.
// suppose that somebody has eaten for the last time.
// then, the cans must look like this (WLOG):
// 01 01 00 10 10
// the last can to be set must be somebody's left can.
// ...could that happen?  hmm



const TABLE_SIZE: usize = 5;
const STARVATION_FREE: bool = false;

pub fn dine(duration: u64, num_reports: u64) {
    let mut state_receivers = Vec::new();
    let chopsticks = (0..TABLE_SIZE).map(
        |_| Arc::new(TryLock::new(Chopstick))).collect::<Vec<_>>();
    let cans = (0..TABLE_SIZE).map(
        |_| [Arc::new(AtomicBool::new(true)),
             Arc::new(AtomicBool::new(true)),]).collect::<Vec<_>>();
    let stopped = Arc::new(AtomicBool::new(false));
    let philosophers : Vec<_> = (0..TABLE_SIZE).map(|id| {
        let chopsticks = [chopsticks[id].clone(),
                          chopsticks[(id + 1) % TABLE_SIZE].clone()];
        let own_cans = [cans[id][0].clone(), cans[id][1].clone()];
        let left_neighbor = (id + (TABLE_SIZE - 1)) % TABLE_SIZE;
        let right_neighbor = (id + 1) % TABLE_SIZE;
        let neighbor_cans = [cans[left_neighbor][0].clone(),
                             cans[right_neighbor][1].clone()];
        let stopped = stopped.clone();
        let (sender, receiver) = mpsc::channel();
        state_receivers.push(receiver);
        thread::spawn(|| {
            let mut philosopher =
                Philosopher::new(chopsticks, stopped, own_cans, neighbor_cans, sender);
            philosopher.dine();
        })
    }).collect();
    for i in 0 .. num_reports {
        thread::sleep(time::Duration::from_millis(duration / num_reports));
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
    cans: [Arc<AtomicBool>; 2],
    neighbor_cans: [Arc<AtomicBool>; 2],
    timer: Timer,
    sender: Sender<State>,
    amount_eaten: u64,
    stopped: Arc<AtomicBool>,
}


impl Philosopher {

    fn new(chopsticks: [Arc<TryLock<Chopstick>>; 2],
           stopped: Arc<AtomicBool>,
           cans: [Arc<AtomicBool>; 2],
           neighbor_cans: [Arc<AtomicBool>; 2],
           sender: Sender<State>) -> Self {
        Philosopher {
            state: State::Thinking,
            chopsticks: chopsticks.clone(),
            cans: cans.clone(),
            neighbor_cans: neighbor_cans.clone(),
            timer: Timer::new(),
            sender: sender,
            amount_eaten: 0,
            stopped: stopped.clone(),
        }
    }

    pub fn signal_done(&mut self) {
        self.cans.iter().for_each(|c|c.store(false, Ordering::SeqCst));
        self.neighbor_cans.iter().for_each(|c|c.store(true, Ordering::SeqCst));
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
        if !self.may_eat() {
            return
        }
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
                self.cans.iter().for_each(|c|c.store(false, Ordering::SeqCst));
                self.neighbor_cans.iter().for_each(|c|c.store(true, Ordering::SeqCst));
                // self.signal_done();
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

    fn may_eat(&self) -> bool {
        !STARVATION_FREE || 
            self.cans.iter()
            .map(|c|c.load(Ordering::SeqCst))
            .all(|b|b)
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

