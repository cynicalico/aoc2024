import argparse
import datetime
import os

import browser_cookie3
import requests


def dl(args):
    os.makedirs(args.dir, exist_ok=True)
    r = requests.get(f"https://adventofcode.com/2024/day/{args.day}/input",
                     cookies=browser_cookie3.load('adventofcode.com'))

    filename = os.path.join(args.dir, f'day_{args.day}.txt')
    with open(filename, 'w') as f:
        f.write(r.text)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()

    subparsers = parser.add_subparsers(required=True)

    parser_dl = subparsers.add_parser('dl', help='download input')
    parser_dl.set_defaults(func=dl)

    parser_dl.add_argument('-day', default=datetime.datetime.today().day, type=int, choices=range(1, 26))
    parser_dl.add_argument('-dir', default='input', type=str)

    args = parser.parse_args()
    args.func(args)
