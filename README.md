## Build tools & versions used
- podman 4
- rust 1.81.0

## Steps to run the app
```bash
# Ensure you have podman on you local machine
podman kube play infra.yaml --replace
# After podman configure and play mongodb, please copy .env.example to .env and save is as .env
# Set all envs that you want
# Ensure that you have cargo on your local machine
cargo run
```
## What was the reason for your focus? What problems were you trying to solve?
Just reply the test task fast as possible my time this week is very short

## How long did you spend on this project?

Start time: 11:04
End Time: 13:56

## Did you make any trade-offs for this project? What would you have done differently with more time?
Any trade-off right now, but if I had time, I will spend more time doing everything in the right way,
handling everything as stream to stream, because right now if the response of the lightning network is a
stream or is so big the program could be crash because the lack of memory.

## What do you think is the weakest part of your project?
It no handle stream, if it handle stream it could work for decades without crashes and it could be more fast than it was.

## Is there any other information you’d like us to know?
I can't make the test pass right now, but I'm really curious about the Sats to BTC scale numbers.