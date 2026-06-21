# Prime generators
def gen_sizes(primes, cap):
    sizes = set()
    for prime in primes:
        s = prime
        while s < cap:
            sizes.add(s);
            s += prime

    return sorted(sizes)

def main():
    primes = [2, 3, 5, 7, 11, 13, 17, 19]
    cap = 100000
    sizes = gen_sizes(primes, cap)
    for s in sizes:
        print(f"{s}")
    l = len(sizes)
    print(f"len: {l}")



def gen_powers(bases, cap):
    sizes = set()
    for base in bases:
        for i in range(3,40):
            p = base**i
            if p > cap:
                break
            sizes.add(p)

    return sorted(sizes)

def main2():
    bases = [2, 3, 5, 7]
    cap = 10000000
    sizes = gen_powers(bases, cap)
    for s in sizes:
        print(f"{s}")
    l = len(sizes)
    print(f"len: {l}")
    print(f"{sizes}")

main2()
