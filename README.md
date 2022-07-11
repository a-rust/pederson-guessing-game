# Pederson commitment scheme over a guessing game
This is an implementation of the Pederson commitment scheme applied to a simple guessing game.

## Formalizing the Pederson commitment scheme
* Alice chooses a (public) multiplicative cyclic group $Z_n^*$ for some integer $n$.
    * Since  $Z_n^* $ needs to be cyclic (with at least 2 generators), $n$ is typically prime (i.e., $Z_p^*$).
* Alice considers 2 generators of $Z_p^*$, namely $g_1$ and $g_2$.
* Alice also considers 2 secret elements of $Z_p^*$, namely $s_1$ and $s_2$.
   * Note that one secret key is enough to transation into a discrete logarithm setting; two keys makes finding $m$ even more difficult.
* Alice computes the commit $c = g_1^{s_1} * g_2^{(s_1*m)}$, where $m$ is Alice's message. 
    * Alice sends $c$, $g_1$, and $g_2$ to Bob, and keeps $s_1$, $s_2$, and $m$ private.
* When Alice is ready to let Bob verify the commitment $c$, she sends $s_1$, $s_2$, and $m$ to Bob. 
    * If $c$ does in fact equal $g_1^{s_1} * g_2^{(s_1*m)}$, then Bob knows Alice was honest.

Check out https://en.wikipedia.org/wiki/Commitment_scheme for more on commitment schemes in general.

## Running
~~~
git clone https://github.com/a-rust/pederson-guessing-game.git
cd pederson-guessing-game
cargo run
~~~

## Game setup
* The user has to guess a (random) number from 1-5. If they get it right, they win.

* To prevent any cheating (i.e., changing the random number after the user broadcasts their guess), a Pederson commitment scheme is applied.

* The user will be asked to set the multiplicative group $Z_p^*$. 
   * This is educational and only uses the primitive type i128, so computations are limited. Ideally, $5\leq p \leq 17$.

* To showcase how difficult it is to break this commitment scheme (but only for much larger $p$), the user is even given a chance to change their guess after seeing the public information ($g_1, g_2, c$).

* After the user makes a (final) guess, the secret keys and message are revealed so that the commitment can be verified.
