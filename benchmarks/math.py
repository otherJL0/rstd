import math as pymath

import rstd.math as rsmath


class Factorial:
    """
    An example benchmark that times the performance of various kinds
    of iterating over dictionaries in Python.
    """

    def time_python(self):
        for _ in range(100):
            for i in range(1500):
                _ = pymath.factorial(i)

    def time_rust(self):
        for _ in range(100):
            for i in range(1500):
                _ = rsmath.factorial(i)
