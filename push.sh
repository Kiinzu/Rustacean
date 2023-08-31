#!/bin/bash

# Set GitHub credentials
username="Kiinzu"
password="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"

# Add changes, commit, and push
git add .
git commit -m "krebi krebi"
git pull origin main  # Pull remote changes to prevent conflicts

# Push using embedded credentials in the URL
url="https://${username}:${password}@github.com/Kiinzu/Rustacean.git"
git push "$url" main

echo "Changes have been added, committed, and pushed."
