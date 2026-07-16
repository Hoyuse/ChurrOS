import os
import platform
import psutil


# ==========================================
# CPU
# ==========================================

def get_cpu():

    try:

        with open("/proc/cpuinfo") as cpuinfo:

            for line in cpuinfo:

                if line.startswith("model name"):

                    return line.split(":", 1)[1].strip()

    except Exception:

        pass

    return "Desconocido"


# ==========================================
# Kernel
# ==========================================

def get_kernel():

    return platform.release()


# ==========================================
# Hostname
# ==========================================

def get_hostname():

    return platform.node()


# ==========================================
# Memoria RAM
# ==========================================

def get_memory():

    memory = psutil.virtual_memory()

    total = memory.total / (1024 ** 3)

    return f"{total:.1f} GiB"


# ==========================================
# Sistema Operativo
# ==========================================

def get_os():

    try:

        with open("/etc/os-release") as os_release:

            for line in os_release:

                if line.startswith("PRETTY_NAME="):

                    return line.split("=", 1)[1].replace('"', "").strip()

    except Exception:

        pass

    return platform.system()


# ==========================================
# Arquitectura
# ==========================================

def get_architecture():

    return platform.machine()