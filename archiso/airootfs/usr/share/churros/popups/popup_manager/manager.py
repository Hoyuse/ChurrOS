from popup import Popup
from process import PopupProcess


class PopupManager:

    @staticmethod
    def show(name):

        if not PopupProcess.running():

            Popup.open(name)

            return

        if PopupProcess.name() == name:

            PopupProcess.kill()

            return

        PopupProcess.kill()

        Popup.open(name)