#!/usr/bin/env python
import argparse
import numpy as np


# compute log2(25*pow(35,n-1))
def complexity(n: int):
    lb5, lb7 = np.log2(5.0), np.log2(7.0)
    return (n+1)*lb5 + (n-1)*lb7


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("n", type=int)
    args = parser.parse_args()
    n = args.n
    y = complexity(n)
    print(f"ID09akmz of length {n} provides complexity of 2^{y:.2f} (approx)")
