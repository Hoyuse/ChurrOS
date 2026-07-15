import subprocess


def launch_application(command: str):

    try:
        subprocess.Popen(command.split())

    except Exception as error:

        print(f"No se pudo abrir la aplicación: {error}")