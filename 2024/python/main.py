import argparse
from solutions.day01 import bonus as day01solve

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', '--day', type=int, required=True)

    args = parser.parse_args()

    match args.day:
        case 1:
            day01solve()




if __name__ == "__main__":
    main()
