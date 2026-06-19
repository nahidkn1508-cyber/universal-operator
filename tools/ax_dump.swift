import Cocoa
import ApplicationServices

func dump(_ element: AXUIElement, level: Int = 0) {
    let indent = String(repeating: "  ", count: level)

    let attrs = [
        kAXRoleAttribute,
        kAXTitleAttribute,
        kAXDescriptionAttribute,
        kAXIdentifierAttribute,
        kAXValueAttribute
    ]

    for attr in attrs {
        var value: CFTypeRef?

        if AXUIElementCopyAttributeValue(
            element,
            attr as CFString,
            &value
        ) == .success {
            print("\(indent)\(attr): \(value!)")
        }
    }

    var children: CFTypeRef?

    if AXUIElementCopyAttributeValue(
        element,
        kAXChildrenAttribute as CFString,
        &children
    ) == .success,
    let list = children as? [AXUIElement] {

        print("\(indent)Children: \(list.count)")

        for child in list {
            dump(child, level: level + 1)
        }
    }
}

let apps = NSWorkspace.shared.runningApplications

guard let chatgpt = apps.first(where: {
    $0.localizedName == "ChatGPT"
}) else {
    fatalError("ChatGPT not running")
}

let app = AXUIElementCreateApplication(chatgpt.processIdentifier)

var value: CFTypeRef?

if AXUIElementCopyAttributeValue(
    app,
    kAXFocusedWindowAttribute as CFString,
    &value
) == .success {

    print("===== FOCUSED WINDOW =====")
    dump(value as! AXUIElement)

} else {
    print("No focused window")
}

print()

if AXUIElementCopyAttributeValue(
    app,
    kAXMainWindowAttribute as CFString,
    &value
) == .success {

    print("===== MAIN WINDOW =====")
    dump(value as! AXUIElement)

} else {
    print("No main window")
}