import math as pymath

import rstd.math as rsmath


class Factorial:
    def time_python(self):
        for _ in range(100):
            for n in range(1500):
                _ = pymath.factorial(n)

    def time_rust(self):
        for _ in range(100):
            for n in range(1500):
                _ = rsmath.factorial(n)


class Comb:
    def time_python(self):
        for n in range(0, 1500, 2):
            for k in range(0, n, 2):
                _ = pymath.comb(n, k)

    def time_rust(self):
        for n in range(0, 1500, 2):
            for k in range(0, n, 2):
                _ = rsmath.comb(n, k)
