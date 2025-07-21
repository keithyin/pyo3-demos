import pyo3_demos
import numpy as np


def main():
    print(pyo3_demos.sum_as_string(1, 2))
    value = np.array([[2.0, 3.0]], dtype=np.float64)
    pyo3_demos.square_in_place(value)
    print(value)
    pass


if __name__ == "__main__":
    main()
    pass
