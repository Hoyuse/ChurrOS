import platform
import psutil


def get_cpu():

    return platform.processor()


def get_kernel():

    return platform.release()


def get_hostname():

    return platform.node()


def get_memory():

    memory = psutil.virtual_memory()

    return f"{round(memory.total / (1024**3), 1)} GiB"