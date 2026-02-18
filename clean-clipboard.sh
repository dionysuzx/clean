#!/bin/bash

# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title Clean Clipboard
# @raycast.mode silent

# Optional parameters:
# @raycast.icon 🧹
# @raycast.packageName Clean

pbpaste | clean | pbcopy
echo "Clipboard cleaned"
