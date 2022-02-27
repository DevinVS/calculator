import QtQuick 2.15
import QtQuick.Layouts 1.1

Rectangle {
    id: window

    width: 400
    height: 500

    color: "#1a1a1a"

    GridLayout {
        anchors.fill: parent
        columns: 5
        columnSpacing: 0
        rowSpacing: 0

        Rectangle {
            Layout.fillHeight: true
            Layout.fillWidth: true

            Layout.columnSpan: 5
            Layout.preferredHeight: 180

            color: "#1a1a1a"

            Rectangle {
                width: parent.width
                height: 2
                anchors.bottom: parent.bottom

                color: "#c782ec"
            }
        }

        Button { text: "7" }
        Button { text: "8" }
        Button { text: "9" }
        Button { text: "("; buttonColor: "#1a1a1a" }
        Button { text: ")"; buttonColor: "#1a1a1a" }
        Button { text: "4" }
        Button { text: "5" }
        Button { text: "6" }
        Button { text: "*"; buttonColor: "#1a1a1a" }
        Button { text: "/"; buttonColor: "#1a1a1a" }
        Button { text: "1" }
        Button { text: "2" }
        Button { text: "3" }
        Button { text: "+"; buttonColor: "#1a1a1a" }
        Button { text: "-"; buttonColor: "#1a1a1a" }
        Button { text: "Del" }
        Button { text: "0" }
        Button { text: "." }

        Button {
            text: "="
            buttonColor: "#c782ec"
            Layout.columnSpan: 2
        }
    }
}
