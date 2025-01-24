import pytest
import itertools as it
import math

import rstd.math as rust_math


def test_factorial():
    for n in range(1500):
        expected = math.factorial(n)
        actual = rust_math.factorial(n)
        assert actual == expected
