#!/usr/bin/env python3
import os
import json
from sys import argv

CONF_PATH = os.environ['HOME'] + '/.config/to'


def get_settings():
    parsed = json.load(open(CONF_PATH))
    return parsed

settings = get_settings()
directories = settings["directories"]


def save_settings():
    json.dump(settings, open(CONF_PATH, 'w'), indent=4)


def add(dir):
    if dir is None:
        dir = os.getcwd()
    directories.append(dir)
    settings["directories"] = directories
    save_settings()


def remove(index):
    del directories[int(index)]
    settings["directories"] = directories
    save_settings()


def dirs(_):
    for i, d in enumerate(directories):
        print("[{0}] {1}".format(i, d))


def dir_iter(query):
    def iter():
        for base in directories:
            for dir in os.scandir(base):
                if dir.is_dir() and dir.name.startswith(query):
                    yield dir
    return sorted(iter(), key=lambda x: len(x.name))


def list_dirs(query):
    if query is None:
        query = ""
    for dir in dir_iter(query):
        print(dir.name)


def go(query):
    dirs = dir_iter(query)
    if len(dirs) == 0:
        print("No results found!")
    else:
        print(dirs[0].path)


def version(_):
    print("To: Python 3 version")


if __name__ == "__main__":
    q = None
    if len(argv) > 2:
        q = argv[2]
    settings = get_settings()
    {
        "add": add,
        "remove": remove,
        "dirs": dirs,
        "go": go,
        "list": list_dirs,
        "version": version
    }[argv[1]](q)