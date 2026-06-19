import Cocoa
import ApplicationServices

func findTextArea(_ element: AXUIElement) -> AXUIElement? {
    var value: CFTypeRef?

    if AXUIElementCopyAttributeValue(
        element,
        kAXRoleAttribute as CFString,
        &value
    ) == .success {

        if let role = value as? String,
           role == kAXTextAreaRole as String {
            return element
        }
    }

    if AXUIElementCopyAttributeValue(
        element,
        kAXChildrenAttribute as CFString,
        &value
    ) == .success {

        if let children = value as? [AXUIElement] {
            for child in children {
                if let result = findTextArea(child) {
                    return result
                }
            }
        }
    }

    return nil
}

func findSendButton(_ element: AXUIElement) -> AXUIElement? {
    var value: CFTypeRef?

    if AXUIElementCopyAttributeValue(
        element,
        kAXDescriptionAttribute as CFString,
        &value
    ) == .success {

        if let description = value as? String,
           description == "Send" {
            return element
        }
    }

    if AXUIElementCopyAttributeValue(
        element,
        kAXChildrenAttribute as CFString,
        &value
    ) == .success {

        if let children = value as? [AXUIElement] {
            for child in children {
                if let button = findSendButton(child) {
                    return button
                }
            }
        }
    }

    return nil
}

guard let runningApp = NSWorkspace.shared.runningApplications.first(where: {
    $0.localizedName == "ChatGPT"
}) else {
    print("ChatGPT is not running.")
    exit(1)
}

let axApp = AXUIElementCreateApplication(runningApp.processIdentifier)

var focused: CFTypeRef?

guard AXUIElementCopyAttributeValue(
    axApp,
    kAXFocusedWindowAttribute as CFString,
    &focused
) == .success else {
    print("No focused window.")
    exit(1)
}

let focusedWindow = focused as! AXUIElement

guard let textArea = findTextArea(focusedWindow) else {
    print("AXTextArea not found.")
    exit(1)
}

let message = "Hello from Universal Operator"

let setResult = AXUIElementSetAttributeValue(
    textArea,
    kAXValueAttribute as CFString,
    message as CFTypeRef
)

if setResult != .success {
    print("Failed to set text: \(setResult.rawValue)")
    exit(1)
}

print("✓ Text inserted")

guard let sendButton = findSendButton(focusedWindow) else {
    print("Send button not found.")
    exit(1)
}

let pressResult = AXUIElementPerformAction(
    sendButton,
    kAXPressAction as CFString
)

if pressResult == .success {
    print("✓ Message sent")
} else {
    print("Failed to press Send: \(pressResult.rawValue)")
}