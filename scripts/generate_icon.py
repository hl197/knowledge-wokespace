from pathlib import Path
from PIL import Image, ImageDraw

out = Path(__file__).resolve().parents[1] / 'src-tauri' / 'icons' / 'icon.ico'
sizes = [16, 24, 32, 48, 64, 128, 256]
images = []
for size in sizes:
    image = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    pad = max(1, round(size * 0.03))
    draw.rounded_rectangle((pad, pad, size-pad-1, size-pad-1), radius=max(2, round(size*0.22)), fill='#b69cff')
    cx = cy = size / 2
    r = size * 0.34
    draw.ellipse((cx-r, cy-r, cx+r, cy+r), fill='#211936')
    points = [(cx, cy-size*.24), (cx+size*.055, cy-size*.055), (cx+size*.24, cy), (cx+size*.055, cy+size*.055), (cx, cy+size*.24), (cx-size*.055, cy+size*.055), (cx-size*.24, cy), (cx-size*.055, cy-size*.055)]
    draw.polygon(points, fill='#f5ecff')
    images.append(image)
images[0].save(out, format='ICO', sizes=[(s, s) for s in sizes], append_images=images[1:])
print(f'generated {out}')
