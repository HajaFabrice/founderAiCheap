from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageOps


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "docs" / "assets"
SOURCE_PATH = ASSETS / "oplurix-website-cover.png"
LOGO_PATH = ASSETS / "oplurix-logo.png"
OUTPUT_PATH = ASSETS / "oplurix-website-preview.png"

WIDTH = 1200
HEIGHT = 630


def main():
    source = Image.open(SOURCE_PATH).convert("RGBA")

    # Keep the preview image forest-led instead of text-led.
    crop = source.crop((0, 88, 840, 529)).resize((WIDTH, HEIGHT), Image.Resampling.LANCZOS)

    tone = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    tone_draw = ImageDraw.Draw(tone)
    tone_draw.rectangle((0, 0, WIDTH, HEIGHT), fill=(7, 22, 17, 24))
    tone_draw.ellipse((-180, -180, 380, 260), fill=(232, 184, 75, 52))
    tone_draw.ellipse((760, 320, 1320, 860), fill=(17, 52, 40, 148))
    tone = tone.filter(ImageFilter.GaussianBlur(26))
    crop.alpha_composite(tone)

    border = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    border_draw = ImageDraw.Draw(border)
    border_draw.rounded_rectangle((0, 0, WIDTH, HEIGHT), radius=34, outline=(255, 255, 255, 26), width=2)
    crop.alpha_composite(border)

    logo_plate = Image.new("RGBA", (156, 156), (0, 0, 0, 0))
    plate_draw = ImageDraw.Draw(logo_plate)
    plate_draw.rounded_rectangle((10, 10, 146, 146), radius=34, fill=(12, 33, 25, 176), outline=(255, 255, 255, 34), width=2)
    logo_plate = logo_plate.filter(ImageFilter.GaussianBlur(0.4))

    logo = Image.open(LOGO_PATH).convert("RGBA")
    logo = ImageOps.contain(logo, (84, 84))
    plate_x = WIDTH - 180
    plate_y = HEIGHT - 182
    crop.alpha_composite(logo_plate, (plate_x, plate_y))
    crop.alpha_composite(logo, (plate_x + 36, plate_y + 36))

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    crop.convert("RGB").save(OUTPUT_PATH, quality=95)
    print(OUTPUT_PATH)


if __name__ == "__main__":
    main()
