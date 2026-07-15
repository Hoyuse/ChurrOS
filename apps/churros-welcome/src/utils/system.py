import os
import platform
import getpass


def get_username():

    return getpass.getuser()


def get_hostname():

    return platform.node()


def get_kernel():

    return platform.release()


def get_architecture():

    return platform.machine()


def get_python_version():

    return platform.python_version()


def get_home():

    return os.path.expanduser("~")