import webbrowser


def open_url(url: str):

    try:
        webbrowser.open(url)

    except Exception as error:

        print(f"No se pudo abrir el navegador: {error}")