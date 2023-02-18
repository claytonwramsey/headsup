import numpy as np
import matplotlib.pyplot as plt


def main():
	mic_locations = np.array([
	    [-90, -140, -105, -10, 90, 120, 85, 0],
	    [95, 15, -60, -115, -85, 0, 105, 160]
	], dtype=np.float64)
	mic_locations /= 100 # tenths of meter


if __name__ == '__main__':
	main()