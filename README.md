# CTF Challenge: Magic Dragon (rev/crypto)

This repository contains all files for the CTF task _Magic Dragon_ of the HackIM CTF '22 for the **nullcon Berlin conference**.
The challenge was solved 0 times during the [competition](https://ctftime.org/event/1594/).

## About this Repository
If you want to play the challenge yourself, head to the `release` directory.
The challenge binary can be found in `target/`, pre-compiled for x64. 
Reverse it and understand what is happening!
When you feel ready to see it in action, start the docker container:
```
docker-compose up
```
and connect to it locally with `nc localhost 10110` or fire up the exploit script at the repo's root to watch the magic happen.
While the readily provided challenge binary is not stripped, it contains only minimal, ambiguous symbols.
If you want to understand more and have a look at the full source code, you can find it in `source`, ready to be built with `cargo`.

## Challenge Writeup
This part, obviously, contains heavy spoilers.
If you want to try it yourself first, stop right here.

When communicating with the program, it becomes apparent very quickly that we are presented with some kind of challenge-response system:
Every message contains an array of 64 entries of 1s and -1s.
On sending a new line, we see a new challenge. 
When trying to answer with 1.0 or -1.0, we will notice soon, that either the system seems pleased and asks us to continue, or tells us that we are wrong and the answer it expected (how very nice).
If we have a look at the main function, we will find a counter, which will trigger the `win()` function after reaching 64. 
The counter resets if the wrong response is transmitted, so we have to provide the correct answer 64 times in a row in order to get the flag.
Furthermore, every 64 iterations of the loop, two methods are called on an object, which is instantiated at the top of the main function.
One output is printed to the user, the other one is used for comparison, so we can deduce that the object is the core of our challenge-response system. 
Having a look at its implementations, i.e., associated functions, we can see lots of maths and arrays and randomness going on.
I won't go too much into the details but there is one primary aspect that should become apparent at some point:
No matter how much noise we add and how much we shift and permute the arrays, we do nothing which would end up losing us linearity. 
All operations, while in sum complex, are invertible.
In conjunction with the fact that we have a system that provides us with endless challenges AND corresponding responses, this screams: _LINEAR REGRESSION_.
Although this exact part might be hard to find out, I can tell you that, in fact, you have a simulated PUF in front of you: a system which emulates a [Physically Unclonable Function](https://en.wikipedia.org/wiki/Physical_unclonable_function). 
To be precise, the instance used here is a 4-XOR Arbiter PUF.
The implementation is a partial Rust-port port of [pypuf](https://github.com/nils-wisiol/pypuf/), a very nice python library that not only implements a multitude of PUFs but also various attacks against such.
We use the lib to perform the attack against this challenge (cf. `exploit.py`).
Playing around with different regressions and training corpus sizes should allow you to model the 'PUF' running at the core of _Magic Dragon_.
Please note, that the random noise is indeed random, so you have to observe, train and trick the thing in one go, resetting the connection will restart the binary at the other end and you will end up with a slightly differently behaving PUF.
In the best case, we understand that we observe a PUF and find all its hyperparameters:
* 64 bits
* 4 lanes
* threshold transformation
* xor combination

Using pypuf's logistical regression attack with these parameters, we can achieve roughly 100% accuracy after observing and training on 21000 challenges (~40 sec).  
But it should also be possible with less knowledge and more data.
NO, bruteforce is not an option to guess 64 correct values, you will need 9,223,372,036,854,775,808 tries in the average case.