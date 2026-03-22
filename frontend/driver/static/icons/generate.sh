# Generate simple placeholder SVG icons for PWA
# In production, replace with actual designed icons

cat > icon.svg << 'SVG'
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <rect width="512" height="512" rx="96" fill="#2563eb"/>
  <text x="256" y="340" font-family="Arial,sans-serif" font-size="280" font-weight="bold" fill="white" text-anchor="middle">W</text>
</svg>
SVG

echo "Placeholder SVG icon created. Convert to PNG with: rsvg-convert -w 192 icon.svg > icon-192.png"
