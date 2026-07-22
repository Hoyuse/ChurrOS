import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: slideRoot
    anchors.fill: parent
    color: "#111827"

    property int slideIndex: 0

    Timer {
        interval: 5000
        running: true
        repeat: true
        onTriggered: {
            slideIndex = (slideIndex + 1) % slides.count
            slidesView.currentIndex = slideIndex
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 40

        Text {
            Layout.alignment: Qt.AlignHCenter
            text: "ChurrOS"
            color: "#F8FAFC"
            font.pixelSize: 28
            font.bold: true
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: "#374151"
        }

        SwipeView {
            id: slidesView
            Layout.fillWidth: true
            Layout.fillHeight: true

            Repeater {
                id: slides
                model: ListModel {
                    ListElement { title: "Moderno y Rápido"; body: "Construido sobre Arch Linux\ncon rendimiento optimizado." }
                    ListElement { title: "Niri — Tiling Inteligente"; body: "Compositor Wayland con\nscrollable-tiling. Fluido y eficiente." }
                    ListElement { title: "Apps Nativas GTK4"; body: "Centro de control, launcher y más.\nTodo en GTK4 + Libadwaita." }
                    ListElement { title: "Audio Profesional"; body: "PipeWire con soporte para\nbaja latencia y enrutamiento flexible." }
                    ListElement { title: "Listo para Desarrollo"; body: "Python, C, Rust, Go y más.\nEntorno preparado para crear." }
                }

                ColumnLayout {
                    spacing: 16

                    Item { Layout.fillHeight: true }

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        text: title
                        color: "#F97316"
                        font.pixelSize: 22
                        font.bold: true
                    }

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        text: body
                        color: "#9CA3AF"
                        font.pixelSize: 15
                        horizontalAlignment: Text.AlignHCenter
                        lineHeight: 1.4
                    }

                    Item { Layout.fillHeight: true }
                }
            }
        }

        PageIndicator {
            Layout.alignment: Qt.AlignHCenter
            count: slides.count
            currentIndex: slideIndex
            delegate: Rectangle {
                width: 10
                height: 10
                radius: 5
                color: index === slideIndex ? "#F97316" : "#374151"
            }
        }
    }
}
