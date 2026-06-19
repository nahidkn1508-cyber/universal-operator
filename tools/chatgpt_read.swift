import Cocoa
import ApplicationServices

var collected: [String] = []

func walk(_ element: AXUIElement) {

    var roleValue: CFTypeRef?
    var valueValue: CFTypeRef?
    var descValue: CFTypeRef?
    var childrenValue: CFTypeRef?

    var role = ""

    if AXUIElementCopyAttributeValue(
        element,
        kAXRoleAttribute as CFString,
        &roleValue
    ) == .success {
        role = roleValue as? String ?? ""
    }

    let value: String = {
        if AXUIElementCopyAttributeValue(
            element,
            kAXValueAttribute as CFString,
            &valueValue
        ) == .success {
            return valueValue as? String ?? ""
        }
        return ""
    }()

    let desc: String = {
        if AXUIElementCopyAttributeValue(
            element,
            kAXDescriptionAttribute as CFString,
            &descValue
        ) == .success {
            return descValue as? String ?? ""
        }
        return ""
    }()

    if !value.isEmpty {
        collected.append("[\(role)] VALUE: \(value)")
    }

    if !desc.isEmpty {
        collected.append("[\(role)] DESC: \(desc)")
    }

    if AXUIElementCopyAttributeValue(
        element,
        kAXChildrenAttribute as CFString,
        &childrenValue
    ) == .success,
       let children = childrenValue as? [AXUIElement] {

        for child in children {
            walk(child)
        }
    }
}

guard let app = NSWorkspace.shared.runningApplications.first(where: {
    $0.localizedName == "ChatGPT"
}) else {
    print("❌ ChatGPT is not running.")
    exit(1)
}

print("✅ ChatGPT PID:", app.processIdentifier)

let axApp = AXUIElementCreateApplication(app.processIdentifier)

var focused: CFTypeRef?

let result = AXUIElementCopyAttributeValue(
    axApp,
    kAXFocusedWindowAttribute as CFString,
    &focused
)

print("Focused window result:", result.rawValue)

guard result == .success else {
    print("❌ Unable to access focused window.")
    exit(1)
}

print("✅ Got focused window")

walk(focused as! AXUIElement)

print("Collected:", collected.count)

for (i, item) in collected.enumerated() {
    print("========== \(i) ==========")
    print(item)
}