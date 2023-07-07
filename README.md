# primeish
various prime utilities in a cli

```sh
primeish is [number] # check if a number is prime
primeish factors [number] # list the prime factors of a number
primeish list [number] # list the first [number] primes
primeish nth [number] # list the [number]th prime
primeish next [number] --amount [number] # list the next [number] primes after [number] (amount is optional)
primeish prev [number] --amount [number] # list the previous [number] primes before [number] (amount is optional)
primeish closest [number] # list the closest prime to [number]
primeish help # show this help
```

## next/prev behavior

if the current number is prime, the next/prev primes will be the next/prev primes after the current number
else, it will be the closest prime to the current number