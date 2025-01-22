import pytest
import rstd


def test_sum_as_string():
    assert rstd.sum_as_string(1, 1) == "2"
