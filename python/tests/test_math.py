import math

import pytest
import rstd.math as rust_math


def test_negative_value_raises_value_error():
    with pytest.raises(ValueError) as python_error:
        math.factorial(-9)

    with pytest.raises(ValueError) as rust_error:
        rust_math.factorial(-9)  # type: ignore

    assert type(python_error.value) == type(rust_error.value)


def test_float_value_produces_type_error():
    with pytest.raises(TypeError) as python_error:
        math.factorial(8.9)  # type: ignore

    with pytest.raises(TypeError) as rust_error:
        rust_math.factorial(8.9)

    assert type(python_error.value) == type(rust_error.value)


def test_factorial():
    for n in range(1500):
        expected = math.factorial(n)
        actual = rust_math.factorial(n)
        assert actual == expected
