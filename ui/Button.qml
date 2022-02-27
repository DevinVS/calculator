import QtQuick 2.15
import QtQuick.Layouts 1.1

Rectangle {
    id: button
    property alias text: text.text
    property string buttonColor: "#242424"
    signal clicked()

    Layout.fillWidth: true
    Layout.preferredHeight: 80
    color: buttonColor

    MouseArea {
        id: mouseArea
        anchors.fill: parent
        hoverEnabled: true
        onClicked: parent.clicked()
    }

    states: [
        State {
            name: "hover"
            when: mouseArea.containsMouse && !mouseArea.pressed
            PropertyChanges {
                target: button
                color: Qt.lighter(buttonColor, 1.1)
            }
        },
        State {
            name: "pressed"
            when: mouseArea.pressed
            PropertyChanges {
                target: button
                color: Qt.lighter(buttonColor, 1.3)
            }
        }
    ]

    transitions: Transition {
        ColorAnimation { duration: 100 }
    }

    Text {
        id: text
        anchors.centerIn: parent
        color: "white"
        font.pointSize: 18
    }
}
