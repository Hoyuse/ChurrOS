/* === ChurrOS Calamares sidebar ===
 *
 * The widget sidebar paints the current step with fillRect(), so QSS
 * cannot round it. This QML panel is the supported way to get 12px pills.
 *
 * Branding + ViewManager are both io.calamares.ui (not io.calamares.core).
 * QtQuick only — Controls/Layouts have failed to load in this branding before.
 */

import QtQuick 2.0
import io.calamares.ui 1.0

Rectangle {
    id: root
    color: Branding.styleString(Branding.SidebarBackground)

    Image {
        id: logo
        source: "logo.svg"
        width: 72
        height: 72
        fillMode: Image.PreserveAspectFit
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.topMargin: 16
    }

    ListView {
        id: steps
        model: ViewManager
        clip: true
        spacing: 6
        boundsBehavior: Flickable.StopAtBounds
        anchors.top: logo.bottom
        anchors.topMargin: 16
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: extras.top
        anchors.bottomMargin: 12
        anchors.leftMargin: 10
        anchors.rightMargin: 10

        delegate: Rectangle {
            width: ListView.view.width
            height: 36
            radius: 12
            color: index === ViewManager.currentStepIndex
                   ? Branding.styleString(Branding.SidebarBackgroundCurrent)
                   : (stepHover.containsMouse ? "#24F97316" : "transparent")

            Text {
                anchors.fill: parent
                anchors.leftMargin: 8
                anchors.rightMargin: 8
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                elide: Text.ElideRight
                text: display
                color: index === ViewManager.currentStepIndex
                       ? Branding.styleString(Branding.SidebarTextCurrent)
                       : Branding.styleString(Branding.SidebarText)
                font.family: "Inter"
                font.pixelSize: 13
                font.weight: index === ViewManager.currentStepIndex ? Font.DemiBold : Font.Normal
            }

            MouseArea {
                id: stepHover
                anchors.fill: parent
                hoverEnabled: true
            }
        }
    }

    Column {
        id: extras
        spacing: 8
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: 10

        Rectangle {
            width: parent.width
            height: 34
            radius: 12
            color: aboutHover.containsMouse ? "#24F97316" : "#0DFFFFFF"
            border.width: 1
            border.color: "#47F97316"

            Text {
                anchors.centerIn: parent
                text: qsTr("About")
                color: "#F5F5F5"
                font.family: "Inter"
                font.pixelSize: 12
                font.weight: Font.DemiBold
            }

            MouseArea {
                id: aboutHover
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    if (typeof debug !== "undefined" && debug)
                        debug.about();
                }
            }
        }

        Rectangle {
            width: parent.width
            height: 34
            radius: 12
            visible: ViewManager.isDebugMode
            color: debugHover.containsMouse ? "#24F97316" : "#0DFFFFFF"
            border.width: 1
            border.color: "#47F97316"

            Text {
                anchors.centerIn: parent
                text: qsTr("Debug")
                color: "#F5F5F5"
                font.family: "Inter"
                font.pixelSize: 12
                font.weight: Font.DemiBold
            }

            MouseArea {
                id: debugHover
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    if (typeof debug !== "undefined" && debug)
                        debug.show();
                }
            }
        }
    }
}
