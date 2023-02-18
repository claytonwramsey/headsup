import numpy as np
import matplotlib.pyplot as plt


def helmet_angle_to_math_angle(angle: int) -> int:
  if angle < 90:
    return angle + 90
  return angle - 270


def get_data(angle: int, distance: int) -> np.array:
  data_matrix = np.loadtxt(f"tests_{angle}_{distance}.csv", delimiter=',')
  return data_matrix


def main():
	angles = [i*45 for i in range(8)]
	distances = [5, 10, 20, 40]

	mic_locations = np.array([
	    [-90, -140, -105, -10, 90, 120, 85, 0],
	    [95, 15, -60, -115, -85, 0, 105, 160]
	], dtype=np.float64)
	mic_locations /= 100 # tenths of meter

	plt.scatter(mic_locations[0, :], mic_locations[1, :])
	plt.xlim([-2, 2])
	plt.ylim([-2, 2])
	arrow_len = 1.5

	for angle in angles:
	  total_mean = 0.0
	  adjusted_angle = helmet_angle_to_math_angle(angle)
	  avg_vec = np.array([0.0, 0.0])
	  for radius in distances:
	    ra_data = get_data(angle, radius)[:, -1]
	    for i in range(len(ra_data)):
	      avg_vec += np.array([np.cos(np.pi*ra_data[i]/180), np.sin(np.pi*ra_data[i]/180)])

	  avg_vec /= np.linalg.norm(avg_vec)

	  plt.arrow(0, 0, arrow_len*avg_vec[0], arrow_len*avg_vec[1], width=0.02, color='red', label='Estimate')
	  print(f"Processing math angle: {adjusted_angle}")
	  plt.arrow(0, 0, arrow_len*np.cos(np.pi*adjusted_angle/180), arrow_len*np.sin(np.pi*adjusted_angle/180), width=0.02, color='green', label='Actual')

	plt.legend(['Microphone'])
	plt.show()

if __name__ == '__main__':
	main()
