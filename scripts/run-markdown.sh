#!/bin/bash
# Run markdown code blocks script
# Mirrors the run-markdown job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

echo "🔧 Running code blocks in Markdown files..."

# Find markdown files in project folders, excluding dependencies and reference folder
# Include: root level .md files, docs/, examples/, and project-specific folders
# Exclude: target/, reference/, and dependency folders like ~/.cargo/
find . \
    -type d \( -name target -o -name reference -o -name .git -o -name node_modules \) -prune \
    -o -name "*.md" -print | \
    grep -E "^\./(README\.md|docs/|examples/|scripts/)" | \
    while read -r md_file; do
        echo "🔧 Running code blocks in $md_file"
        set -euo pipefail
        md_dir=$(dirname "$md_file")

        # Extract and run bash code blocks (only ```bash, not ```sh, ```shell, etc.)
        bash_blocks=$(awk '
            BEGIN { inblock=0 }
            /^```bash[ \t]*$/ { inblock=1; next }
            /^```/ { if (inblock) inblock=0; next }
            inblock { print }
        ' "$md_file")

        # Only run if there are bash blocks (specifically ```bash, not ```sh or ```shell)
        if [[ -n "$bash_blocks" ]]; then
            echo "   Found bash code blocks, executing..."
            echo "$bash_blocks" | (cd "$md_dir" && bash) || {
                echo "❌ Bash code blocks in $md_file failed"
                exit 1
            }
            echo "✅ Bash code blocks in $md_file succeeded"
        else
            echo "ℹ️  No \`\`\`bash code blocks found in $md_file (skipping \`\`\`sh, \`\`\`shell, etc.)"
        fi
    done

echo "✅ All Markdown code blocks executed successfully!"
