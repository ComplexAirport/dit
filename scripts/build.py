from argparse import ArgumentParser
from pathlib import Path
import subprocess as sp
import shutil
import os


def main():
    parser = ArgumentParser()
    parser.add_argument('--cwd', default='.')
    parser.add_argument('--release', action='store_true')
    parser.add_argument('--output-dir', default='bin/')
    parser.add_argument('-o', '--output-exe', default='dit.exe' if os.name == 'nt' else 'dit')
    parser.add_argument('-b', '--source-exe', default='dit_cli.exe' if os.name == 'nt' else 'dit_cli')
    args = parser.parse_args()

    command = ['cargo', 'build']
    if args.release:
        command.append('--release')

    os.chdir(args.cwd)
    sp.run(command, shell=True)

    out_dir = Path(args.output_dir)
    if not out_dir.is_dir():
        out_dir.mkdir()
    out = out_dir.joinpath(args.output_exe)
    source_dir = 'target/release/' if args.release else 'target/debug/'
    source = Path(source_dir).joinpath(args.source_exe)

    if source.is_file():
        shutil.copy(source, out)
        print(f'Built to {out}')
    else:
        print('Something went wrong.')


if __name__ == '__main__':
    main()
