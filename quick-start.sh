#!/bin/bash

# Quick Start Script for URL Shortener

echo "🚀 URL Shortener - Quick Start"
echo "================================"
echo ""

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    echo "❌ Wrangler not found. Installing..."
    npm install -g wrangler
fi

# Check if worker-build is installed
if ! command -v worker-build &> /dev/null; then
    echo "📦 Installing worker-build..."
    cargo install worker-build
fi

echo "✅ Dependencies ready!"
echo ""

# Build the project
echo "🔨 Building Rust Worker..."
worker-build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📝 Next steps:"
    echo ""
    echo "1. Login to Cloudflare:"
    echo "   wrangler login"
    echo ""
    echo "2. Create KV namespace:"
    echo "   wrangler kv:namespace create \"URLS\""
    echo ""
    echo "3. Update wrangler.toml with KV ID"
    echo ""
    echo "4. Deploy:"
    echo "   wrangler deploy"
    echo ""
    echo "5. Test locally:"
    echo "   wrangler dev"
    echo ""
    echo "📖 Full guide: DEPLOYMENT.md"
else
    echo "❌ Build failed. Check errors above."
    exit 1
fi
