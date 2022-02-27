import QtQuick 2.15
import QtQuick.Layouts 1.1
import QtQuick.Window 2.0
import Calculator 1.0

Window {
    id: window

    visible: true
    width: 400
    height: 500
    title: "Calculator"

    color: "#1a1a1a"

    Calculator {
        id: calculator

        onResultChanged: {
            resultText.text = result
            input.text = ""
        }
    }

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

            TextInput {
                id: input
                width: parent.width
                color: "#ffffff"
                font.pointSize: 24
                anchors.left: parent.left
                anchors.bottom: parent.bottom
                anchors.leftMargin: 32
                anchors.bottomMargin: 16
                selectByMouse: true
                selectionColor: "#c782ec"

                validator: RegularExpressionValidator {
                    regularExpression: /[0-9\-+*/.\(\)]*/
                }

                onAccepted: {
                    calculator.compute(text)
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.IBeamCursor
                    acceptedButtons: Qt.NoButton
                }
            }

            TextInput {
                id: resultText


                text: calculator.result
                color: "#A7A7A7"
                font.pointSize: 24
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                anchors.rightMargin: 32
                anchors.bottomMargin: 16
                readOnly: true
                selectByMouse: true
                selectionColor: "#c782ec"

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.IBeamCursor
                    acceptedButtons: Qt.NoButton
                }
            }

            Rectangle {
                width: parent.width
                height: 2
                anchors.bottom: parent.bottom

                color: "#c782ec"
            }
        }

        Button { text: "7"; onClicked: {input.text += "7"} }
        Button { text: "8"; onClicked: {input.text += "8"} }
        Button { text: "9"; onClicked: {input.text += "9"} }
        Button {
            text: "("
            onClicked: {input.text += "("}
            buttonColor: "#1a1a1a"
        }
        Button {
            text: ")"
            onClicked: {input.text += ")"}
            buttonColor: "#1a1a1a"
        }
        Button { text: "4"; onClicked: {input.text += "4"} }
        Button { text: "5"; onClicked: {input.text += "5"} }
        Button { text: "6"; onClicked: {input.text += "6"} }
        Button {
            text: "*"
            onClicked: {input.text += "*"}
            buttonColor: "#1a1a1a"
        }
        Button {
            text: "/"
            onClicked: {input.text += "/"}
            buttonColor: "#1a1a1a"
        }
        Button { text: "1"; onClicked: {input.text += "1"} }
        Button { text: "2"; onClicked: {input.text += "2"} }
        Button { text: "3"; onClicked: {input.text += "3"} }
        Button { text: "+"; buttonColor: "#1a1a1a"; onClicked: {input.text += "+"}}
        Button { text: "-"; buttonColor: "#1a1a1a"; onClicked: {input.text += "-"}}
        Button {
            text: "Del"
            onClicked: {
                input.text = input.text.slice(0, input.text.length-1)
            }
        }
        Button { text: "0"; onClicked: {input.text += "0"} }
        Button { text: "."; onClicked: {input.text += "."} }

        Button {
            text: "="
            buttonColor: "#c782ec"
            Layout.columnSpan: 2
            onClicked: {
                calculator.compute(input.text)
            }
        }
    }
}
