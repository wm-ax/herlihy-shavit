Dining Philosophers (starvation free version)
=============================================

This version of the dining philosophers puzzle is supposed to be starvation free: in other words, each philosopher must eat infinitely many times.

The idea of the implementation is that each philosopher has a left and a right can.  He can eat only if both of his cans are up.  Each time he eats, he knocks over both of his own cans, and sets up again the adjacent can of each of his two neighbors (i.e., the left can of the right neighbor, and the right can of the left).  They begin the meal with all cans up.

I surrounded the array of all cans with a single mutex.  This means that no philosopher's sequence of four can manipulations is ever interleaved with another's. It follows that: 
(D) *when no such sequence is taking place, any two adjacent cans of two different philosophers must be in different states.*

Now, suppose, for example, that p2 eats only finitely many times.  Then there must be some t such that p2 knocks over his cans for the last time at t.  Then at t, the cans must look like this:

?? ?1 00 1? ??

If both p1 and p3 eat after t, then both of p2's cans will be set, and he will eat again.  So suppose, WLOG, that p3 does not eat after p2 does.  Then, at least one of p3's cans must be unset; so at t, the cans must actually look like this:

?? ?1 00 10 ??

By (D), in fact we have

?? ?1 00 10 1?

But if p4 eats after t, then p4 will reset p3's can and p3 will eat after t, a contradiction.  so p4 does not eat after t.  so still at t, we must have 

?? ?1 00 10 10

and by the same reasoning we must have, still at t

10 ?1 00 10 10

again by (D), p1's left can must be set

10 11 00 10 10

so that p1 will eat after t.  Then p1 will reset p0's can and p0 will eat after t, a contradition.
