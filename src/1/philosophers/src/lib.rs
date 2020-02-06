// 2840
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::sync::mpsc::{Sender};
use std::time;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::rngs::ThreadRng;
use rand::{Rng};
use try_lock::TryLock;

// starvation-free version
    // each phil has a left and a right can.
    // each time he eats, he knocks over his own left and right cans, and sets up the right, left cans of his left, right neighbors (in that order)
// To see that this is starvation free, first note that the array of cans is synchronized, so that no philosopher's sequence of four can manipulations is ever interleaved with another.  It follows that:
// (*) when no such sequence is taking place, any two adjacent cans of two different philosophers must be in different states.

// Now, suppose that p2 knocks over his cans for the last time at t.  Then at t, the cans must look like this:
// ?? ?1 00 1? ??
// If both p1 and p3 eat after t, then both of p2's cans will be set, and he will eat again.  So suppose, WLOG, that p3 does not eat after p2 does.  Then, at least one of p3's cans must be unset; so at t, the cans must actually look like this:
// ?? ?1 00 10 ??
// By (*), in fact we have
// ?? ?1 00 10 1?
// But if p4 eats after t, then p4 will reset p3's can and p3 will eat after t, a contradiction.  so p4 does not eat after t.  so still at t, we must have 
// ?? ?1 00 10 10
// and by the same reasoning we must have, still at t
// 10 ?1 00 10 10
// again by (*), p1's left can must be set
// 10 11 00 10 10
// so that p1 will eat after t.  Then p1 will reset p0's can and p0 will eat after t, a contradition.


const TABLE_SIZE: usize = 5;
const STARVATION_FREE: bool = true;

pub fn dine(duration: u64, num_reports: u64) {
    let mut state_receivers = Vec::new();
    let chopsticks = (0..TABLE_SIZE).map(
        |_| Arc::new(TryLock::new(Chopstick))).collect::<Vec<_>>();
    // let cans = (0..TABLE_SIZE).map(
    //     |_| [Arc::new(AtomicBool::new(true)),
    //          Arc::new(AtomicBool::new(true)),]).collect::<Vec<_>>();
    let schedule = Arc::new(Mutex::new([[true, true]; TABLE_SIZE]));
    let stopped = Arc::new(AtomicBool::new(false));
    let philosophers : Vec<_> = (0..TABLE_SIZE).map(|id| {
        let chopsticks = [chopsticks[id].clone(),
                          chopsticks[(id + 1) % TABLE_SIZE].clone()];
        let scheduler = Scheduler {id: id,
                                   schedule: schedule.clone()};
        let stopped = stopped.clone();
        let (state_tx, state_rx) = mpsc::channel();
        state_receivers.push(state_rx);
        thread::spawn(|| {
            let mut philosopher =
                Philosopher::new(chopsticks, scheduler, stopped, state_tx);
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
    scheduler: Scheduler,
    timer: Timer,
    state_tx: Sender<State>,
    amount_eaten: u64,
    stopped: Arc<AtomicBool>,
}


impl Philosopher {

    fn new(chopsticks: [Arc<TryLock<Chopstick>>; 2],
           scheduler: Scheduler,
           stopped: Arc<AtomicBool>,
           state_tx: Sender<State>) -> Self {
        Philosopher {
            scheduler,
            state_tx,
            stopped,
            chopsticks: chopsticks.clone(),
            state: State::Thinking,
            timer: Timer::new(),
            amount_eaten: 0,
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
                self.state_tx.send(self.state).unwrap();
                let amount = self.timer.sleep_random();
                self.amount_eaten += amount;
                // self.cans.iter().for_each(|c|c.store(false, Ordering::SeqCst));
                // self.neighbor_cans.iter().for_each(|c|c.store(true, Ordering::SeqCst));
                self.scheduler.signal_done();
            },
            _ => {},
        }
    }

    fn think(&mut self) {
        self.state = State::Thinking;
        self.state_tx.send(self.state).unwrap();
        self.timer.sleep_random();
    }

    fn wait(&mut self) {
        self.state = State::Waiting;
        self.state_tx.send(self.state).unwrap();
    }

    fn may_eat(&self) -> bool {
        !STARVATION_FREE || 
            self.scheduler.permits_eating()
    }

    fn is_done(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }

}


struct Scheduler {
    id: usize,
    schedule: Arc<Mutex<[[bool; 2]; TABLE_SIZE]>>,
}

impl Scheduler {
    fn signal_done(&self) {
        let mut schedule = self.schedule.lock().unwrap();
        let left_neighbor = (self.id + (TABLE_SIZE - 1)) % TABLE_SIZE;
        let right_neighbor = (self.id + 1) % TABLE_SIZE;
        let mut neighbor_cans = [schedule[left_neighbor][0],
                                 schedule[right_neighbor][1]];
        schedule[self.id] = [false, false];
        schedule[left_neighbor][1] = true;
        schedule[right_neighbor][0] = true;
        // println!("Signal done for {}. schedule = {:?}", self.id, schedule);
    }
    fn permits_eating(&self) -> bool {
        let schedule = self.schedule.lock().unwrap();
        let cans = schedule[self.id];
        let ret = cans[0] && cans[1];
        // if ret {
        //     println!("philosopher {} had to wait", self.id);
        // }
        ret
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

