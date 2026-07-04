tell application "iTerm"
    set dirlist to {"/Users/dezloper/Desktop/CEX/cex-be/api", "/Users/dezloper/Desktop/CEX/cex-be/ws", "/Users/dezloper/Desktop/CEX/cex-be/engine", "/Users/dezloper/Desktop/CEX/cex-be/docker"}
    repeat with d in dirlist
        set newWindow to (create window with default profile)
        tell current session of newWindow
            write text "cd " & d
        end tell
    end repeat
end tell