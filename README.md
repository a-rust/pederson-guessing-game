# Pederson commitment scheme over a guessing game
This is an implementation of the Pederson commitment scheme applied to a simple guessing game.

## Formalizing the Pederson commitment scheme:
Suppose Alice wants to send a message $m$ to Bob, but also wants Bob to wait until she is ready for him to open it. She can send a commitment to Bob containing, but also hiding (through a private key), her message. Whenever Alice is ready, she can give Bob both the private key and $m$ for him to verify the commitment.

The Pederson commitment scheme can be formalized (with some intuition) as follows:
* Alice chooses a (public) multiplicative cyclic group $Z_n^*$ for some integer $n$ in which the Pederson commitment scheme will be performed.
    * Since $Z_n^*$ needs to be cyclic, $n$ is typically prime (i.e., $Z_p^*$).
* Alice considers 2 generators of $Z_p^*$, namely $g_1$ and $g_2$.
* Alice also considers 2 secret elements of $Z_p^*$, namely $s_1$ and $s_2$.
* Alice computes the commit $c = g_1^{s_1} * g_2^{(s_1*m)}$, where $m$ is Alice's message. 
    * Alice sends $c$, $g_1$, and $g_2$ to Bob, and keeps $s_1$, $s_2$, and $m$ private.
* When Alice is ready to let Bob verify the commitment $c$, she sends $s_1$, $s_2$, and $m$ to Bob. 
    * If $c$ does in fact equal $g_1^{s_1} * g_2^{(s_1*m)}$, then Bob knows Alice was honest.

Note that one secret key is enough to transation into a discrete logarithm problem; two keys makes finding $m$ even more difficult for Bob.

I couldn't find many direct, formal resources on the Pederson commitment scheme, but there is a [wikipedia page](https://en.wikipedia.org/wiki/Commitment_scheme) on commitment schemes in general.

## Game setup:
* The user has to guess a (random) number from 1-5. If they get it right, they win.

* To prevent any cheating (i.e., changing the random number after the user broadcasts their guess), a Pederson commitment scheme is applied.

* The user can even set the multiplicative group $Z_p^*$. Since this is supposed to be educational, ideally $p\geq5$. Also, this project only uses the rand crate, and by the nature of the Pederson commitment scheme, numbers get big pretty fast; so i128's are somewhat limited. To prevent any multiplication panics, ideally $p\leq19$.

* To showcase how difficult it is to break this commitment scheme (but only for much larger $p$), the user is even given a chance to change their guess after seeing the public information ($g_1, g_2, c$).

* After the user makes a (final) guess, the commitment can be verified.