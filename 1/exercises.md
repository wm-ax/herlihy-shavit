1. See [source](https://gitlab.com/max.morgan.weiss/umb-concurrency/-/tree/master/src/1/philosophers/src).

2. Which is a "safety" property?  Which a "liveness" property?  I think the distinction is slightly vague, at least if it is understand as a merely logical one.  It seems clear that purely universal properties are safety, while unqualified existential properties are liveness.  But what about "universal-existential"?  A case could sometimes be made for either.  Anyway, here are my guesses:
  
    1. SAFETY: Patrons are served in the order they arrive.
    2. SAFETY: What goes up must come down.
    3. LIVENESS: If two or more processes are waiting to enter their critical sections, at least one succeeds.
    4. LIVENESS?: If an interrupt occurs, then a message is printed within one second.
    5. LIVENESS: If an interrupt occurs, then a message is printed.
    6. SAFETY: The cost of living never decreases.
    7. LIVENESS: Two things are certain: death and taxes.
    8. SAFETY: You can always tell a Harvard man.

3. In the producer–consumer fable, we assumed that Bob can see whether
the can on Alice’s windowsill is up or down. Design a producer–consumer protocol using cans and strings that works even if Bob cannot see the state of Alice’s
can (this is how real-world interrupt bits work).

Let's put a can on each of Alice and Bob's windowsills, so that each person can see only their own can, but can still unset the other's can.  And, let's prescribe to them a protocol as follows.

    0. They begin with no food in the yard; Alice's can is up and Bob's can is down.
    1. Bob's loop: wait until his can is down, put food in the yard, reset his can, and then unset Alice's can.
    2. Alice's loop: wait until her can is down and then release her dogs; when their food is depleted reset her own can and unset Bob's can.

    To verify safety, I'll argue that Alice's loop preserves this invariant: after each execution
    (a) Alice's can is up and Bob's is down, and Alice and Bob are in their wait() subloops
    (b) Alice and Bob have not been in the yard simultaneously.

This is trivial for iteration $i=0$.  So suppose it holds for i.  Then Alice waits and cannot enter the yard, while: Bob reads, enters and leaves the yard, sets up Bob's can, and unsets Alice's can.  So the dogs' i+1th entry cannot happen until after her can has been unset, which is after Bob has set back up his can and entered his wait() loop:

* Bob in yard (i+1) -> Bob sets his can (i+1) -> Bob unsets Alice's can (i+1) -> Bob waits -> dogs in yard (i+1)

So, the dogs' i+1th visit of the yard does not overlap with any of Bob's visits, and this ensures (b).

Furthermore, the ordering (whose last moment t is the end of Alice's i+1th loop iteration)

* Bob waits -> dogs in yard (i+1) -> Alice sets up Alice's can (i+1) -> Alice unsets Bob's can (i+1)

guarantees that Bob must be waiting at t, while Alice's can is up and Bob's can is down.  So invariant (b) must hold as well.

4. ...?

5. The prisoners agree that the last prisoner in line (the first to guess) will guess "Red" iff the number of red hats he sees is even.

6a. Suppose a computer program has a method M that cannot be parallelized, and
that this method accounts for 40% of the program’s execution time. What is
the limit for the overall speedup that can be achieved by running the program
on an n-processor multiprocessor machine?

Here, p=3/5, so the speedup is 1/(2/5 + 3/(5n)), which simplifies to 5n / (2n + 3).  The latter converges to 2.5 from below, so this is the maximum possible speedup.

6b. Suppose the method M accounts for 30% of the program’s computation time.  What should be the speedup of M so that the overall execution time improves by a factor of 2?

The overall time is a function of both the execution time of M and the number N of processors.  So, a priori the speedup of M required to half the overall time will be a function of N.  Let p' be the proportion p' of execution time required by the parallelizable part in the nonparallelized execution, given that the speedup has been applied to M.  Amdahl's law now characterizes the required speedup as

1 / (1 - p' + p'/n) = 2 / (1 - p + p/n)

which simplifies to

1 - p + p/n = 2(1 - p' + p'/n)

Fixing p = 7/10, it follows that

p' = (17n - 7) / 10(n-1)

Furthermore, we can express p' as

p' = p / (p + s(1-p)) = 7 / (3s+7).

Putting these two reults together, it follows that

s = (63 - 53n) / (21 - 51n)

where s is the reciprocal of the speedup.

For example, if n=2, then s = 43/81; if n=3, then s = 8/11.

6c. Suppose the method M can be sped up three-fold.  What fraction of the overall execution time must M account for in order to double the overall speedup of the program?

As before, we have

1 - p + p/n = 2(1 - p' + p'/n)

and

p' = p / (p + s(1-p)).

In this case, we know s = 1/3, and the goal is to express p as a function of n.  The second equation implies

p' = 1 / (p + (1-p)/3) = 3 / (2p + 1)

while the first equation gives

p' = (p - pn - n) / (2 - 2n).

Putting the last two together, we get the condition

3(n - 2) / n - 1 = 2p^2 + p.

For example, if n = 2, then we must have p=0 (and this makes sense, since we cannot double the overall speed with n = 2 unless the process is entirely parallelizable).  [I must be doing something wrong, because this has no roots if n = 3.]


7. Running your application on two processors yields a speedup of S2. Use Amdahl’s Law to derive a formula for Sn, the speedup on n processors, in terms of n and S2.

The first assumption says that 

S2 = 2/(2-p)

and so 

p = (2S2 - 2)/S2.

So,

Sn = 1/(1 - p + p/n) = nS2 / (2S2 + 2n - nS2 - 2).

Note that this does have to be positive because S2 <= 2.  And in particular, if S2 = 2, then Sn = n (as it should be, since S2=2 implies p=0).

8. You have a choice between buying one uniprocessor that executes five
zillion instructions per second, or a ten-processor multiprocessor where each processor executes one zillion instructions per second. Using Amdahl’s Law, explain
how you would decide which to buy for a particular application.

The first processor computes at a constant speed of 5 zps.  The speed of the second processor is a function of the proportion p of parallelizability of the given task, namely 10 / (10-9p).  So the second processor runs faster than the first iff 

10 / (10 - 9p) > 5

which holds iff p > 8/9.
