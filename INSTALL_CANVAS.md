# Installing Canvas for Testing

The comparison script requires the `canvas` package for image processing.

## Installation

### macOS
```bash
# Install dependencies first (if needed)
brew install pkg-config cairo pango libpng jpeg giflib librsvg pixman

# Then install canvas
npm install canvas
# or with bun:
bun install canvas
```

### Linux (Ubuntu/Debian)
```bash
sudo apt-get install build-essential libcairo2-dev libpango1.0-dev libjpeg-dev libgif-dev librsvg2-dev

npm install canvas
```

### Quick Start
```bash
# Just run this in the project root:
npm install
# or
bun install
```

## Troubleshooting

If you get compilation errors, make sure you have:
- Node.js or Bun installed
- Build tools (Xcode Command Line Tools on macOS)
- Cairo and dependencies installed

### Alternative: Use Node instead of Bun

If canvas doesn't work with Bun, you can modify the test script to use Node:
```bash
# Edit test_algorithms.sh and change 'bun run' to 'node'
sed -i '' 's/bun run/node/g' test_algorithms.sh
```

Then run:
```bash
npm install canvas
./test_algorithms.sh
```
