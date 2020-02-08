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

    0. They begin with no food in the yard; Bob's can is up and Alice's can is down.
    1. Bob waits until his can is down, puts food in the yard, resets his own can, and then unsets Alice's can.
    2. Alice waits until her can is down and then releases her dogs; when their food is depleted she resets her own can and unsets Bob's can.

    Safety: suppose Bob and the dogs are in the yard at once t.  Then Alice must've unset Bob's can before this; let t' be the time this last happened.  At t', Alice's dogs must be inside, and her can must be up.  Since her dogs are out at t, Bob must knock over her can between t' and t.  ...?

4. ...?

5. The prisoners agree that the last prisoner in line (the first to guess) will guess "Red" iff the number of red hats he sees is even.

6. Use Amdahl’s Law to resolve the following questions:
(a) Suppose a computer program has a method M that cannot be parallelized, and
that this method accounts for 40% of the program’s execution time. What is
the limit for the overall speedup that can be achieved by running the program
on an n-processor multiprocessor machine?

Here, p=3/5, so the speedup is 1/(2/5 + 3/(5n)), which simplifies to 5n / (2n + 3).  The latter ratio converges to 2.5 from below, so this is the maximum possible speedup.

<!-- (b) Suppose the method M accounts for 30% of the program’s computation time.  What should be the speedup of M so that the overall execution time improves by a factor of 2? -->

<!-- I think this means the following: find this speedup s as a function of the number of processors. -->

<!-- Let's write p=3/10. For s to speed up the computation by a factor of two, we must arrange for the limit of sM + (1 - sM)/N be half of M + (1 - M)/N.  Some algebra shows that we must have  -->

<!-- s = (np + 1 + p) / 2(np + p) -->

<!-- so  -->

<!-- s = (3n + 13) / (6n + 6). -->

<!-- For example, if n = 3, then s = 22 / 24 -->
<!-- For example, if n = 2, then s = 19 / 18 -->
<!-- ...seems wrong somewhere3 -->

<!-- (c) Suppose the method M can be sped up three-fold.  What fraction of the overall execution time must M account for in order to double the overall speedup of the program? -->

<!-- The condition amounts to -->

<!-- lim_{N} p/3 + (1 - p/3)/N -->

<!-- being half of -->

<!-- lim_{N} p + (1 - p)/N. -->

<!-- Since the right-hand summands vanish, this means p/ -->

<!-- which implies -->

<!-- 1/2 = p/3 -->


<!-- 6.  -->
