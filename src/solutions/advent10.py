import sys

import numpy as np
import matplotlib.pyplot as plt


def align_stars(positions, velocities):
    seconds = 0
    last_std = positions.std()

    while True:
        positions += velocities
        seconds += 1

        if positions.std() < last_std:
            last_std = positions.std()

        print(last_std)

        # empirically determined by watching the std scroll by
        # (it's pretty quick) until it started increasing again
        if positions.std() < 24: 
            print("Waited {}s".format(seconds))
            pmin = positions.min()
            pmax = positions.max()
            minmax = pmax-pmin
            positions -= pmin

            # plot the image, simply adjusted to the maximum pixel distance
            image = np.zeros((minmax+1, minmax+1), dtype=bool)
            for pos in positions:
                image[pos[0], pos[1]] = True

            plt.imshow(image)
            plt.show()



def main(args):
    input_file = args[1]
    star_lines = [line.strip() for line in open(input_file)]

    # just parse this with hardcoded ranges
    positions = np.array([[int(line[10:16]), int(line[18:24])] for line in star_lines])
    velocities = np.array([[int(line[36:38]), int(line[40:42])] for line in star_lines])

    align_stars(positions, velocities)

if __name__ == '__main__':
    main(sys.argv)
