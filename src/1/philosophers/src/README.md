Dining Philosophers (starvation free version)
=============================================

The dining philosopher's puzzle is due to Dijkstra.  According to the puzzle, five philosophers surround at a table, with one chopstick between each adjacent philosopher pair.  At random intervals, each philosopher sits down, grabs both chopsticks if possible, and eats for a while.  Then he relinquishes the chopsticks, gets up again and resumes thinking.  The puzzle is this: (i) model their behavior so that no two philosophers hold the same chopstick at the same time; (ii) ensure the meal is never deadlocked, where each philosopher holds one chopstick while waiting for another philosopher to relinquish the other; (iii) ensure that each philosopher eats infinitely often; (iv) generalize to arbitrarily many philosophers.

This version of the dining philosophers puzzle handles parts (i), (iii), and (iv).  For part (iv), I simply used a constant TABLE_SIZE to define the number of philosophers (and chposticks).  For part (i), I simply surrounded each chopstick with a lock, which a philosopher holds for the entire time he is eating, and then releases it (in Rust, the lock is released when its reference goes out of scope).  

I'm not sure about part (ii).  The approach I followed is this: when a philosopher sits down to eat, tries to pick up the left chopstick.  If he's successful, he tries to pick up the right.  If this succeeds, then he eats for a while; else he releases the left chopstick and another loop iteration begins.  It is (I guess) possible that the loops could become synchronized so that this happens repeatedly: all philosophers simultaneously pick up their left chopstick, try and fail to pick up the right one, and releases the left.  To prevent such a scenario, it would be possible to surround the array of all chopsticks with another lock which guards the process of chopstick acquisition, so that chopstick acquisition processes could not be interleaved.  Since the above scenario seems rather unlikely, I didn't bother with this.

I found part (iii) the most interesting, namely to ensure starvation-freedom.  The idea of my implementation is that each philosopher has a left and a right can.  He can eat only if both of his cans are up.  Each time he eats, he knocks over both of his own cans, and sets up again the adjacent can of each of his two neighbors (i.e., the left can of the right neighbor, and the right can of the left).  They begin the meal with all cans up.

I surrounded the array of all cans with a single mutex.  This means that no philosopher's sequence of four can manipulations is ever interleaved with another's. It follows that: 
(D) *when no such sequence is taking place, any two adjacent cans of two different philosophers must be in different states.*

Now, suppose, for example, that p2 eats only finitely many times.  Then there must be some t such that p2 knocks over his cans for the last time at t.  Then at t, the cans must look like this:

?? ?1 00 1? ??

If both p1 and p3 eat after t, then both of p2's cans will be set, and he will eat again.  So suppose, WLOG, that p3 does not eat after t.  Then, at least one of p3's cans must be unset; so at t, the cans must actually look like this:

?? ?1 00 10 ??

By (D), in fact we have

?? ?1 00 10 1?

But if p4 eats after t, then p4 will reset p3's can and p3 will eat after t, a contradiction.  so p4 does not eat after t.  so still at t, we must have 

?? ?1 00 10 10

and by the same reasoning we must have, still at t

10 ?1 00 10 10

Again by (D), p1's left can must be set; but since p0 does not eat after t, p1 must not eat either.  So, p1's right can must be unset at t.  This contradicts our conclusion that his right can is set at t.
