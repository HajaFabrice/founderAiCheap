import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont, ImageOps


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "docs" / "assets"
LOGO_PATH = ASSETS / "oplurix-logo.png"
OUTPUT_DESKTOP = ASSETS / "oplurix-facebook-cover.png"
OUTPUT_MOBILE_SAFE = ASSETS / "oplurix-facebook-cover-mobile-safe.png"

WIDTH = 1640
HEIGHT = 624


def load_font(path: str, size: int):
    return ImageFont.truetype(path, size=size)


def draw_wrapped_text(draw, text, font, x, y, max_width, fill, line_gap):
    words = text.split()
    lines = []
    current = ""

    for word in words:
        trial = word if not current else f"{current} {word}"
        bbox = draw.textbbox((0, 0), trial, font=font)
        if bbox[2] - bbox[0] <= max_width:
            current = trial
        else:
            if current:
                lines.append(current)
            current = word

    if current:
        lines.append(current)

    for line in lines:
        draw.text((x, y), line, font=font, fill=fill)
        bbox = draw.textbbox((x, y), line, font=font)
        y += (bbox[3] - bbox[1]) + line_gap

    return y


def draw_pill(draw, x, y, text, font, fill, outline, text_fill):
    bbox = draw.textbbox((0, 0), text, font=font)
    text_w = bbox[2] - bbox[0]
    text_h = bbox[3] - bbox[1]
    pad_x = 18
    pad_y = 10
    rect = (x, y, x + text_w + pad_x * 2, y + text_h + pad_y * 2)
    draw.rounded_rectangle(rect, radius=24, fill=fill, outline=outline, width=1)
    draw.text((x + pad_x, y + pad_y - 1), text, font=font, fill=text_fill)
    return rect[2]


def build_base():
    cover = Image.new("RGBA", (WIDTH, HEIGHT), "#16331F")
    draw = ImageDraw.Draw(cover)

    draw.rectangle((0, 0, WIDTH, HEIGHT), fill="#16331F")

    glow = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow)
    glow_draw.ellipse((-120, -160, 520, 420), fill=(232, 184, 75, 58))
    glow_draw.ellipse((920, -80, 1700, 580), fill=(127, 168, 130, 62))
    glow_draw.ellipse((1120, 140, 1700, 760), fill=(255, 255, 255, 22))
    glow_draw.ellipse((220, 280, 960, 900), fill=(28, 74, 60, 78))
    glow = glow.filter(ImageFilter.GaussianBlur(38))
    cover.alpha_composite(glow)

    line_layer = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    line_draw = ImageDraw.Draw(line_layer)
    for offset in range(-40, 420, 46):
        line_draw.arc((70 + offset, 30 + offset, 980 + offset, 700 + offset), start=202, end=332, fill=(255, 255, 255, 20), width=2)
    for x in range(90, 500, 64):
        line_draw.line((x, 90, x + 140, 534), fill=(255, 255, 255, 14), width=1)
    line_layer = line_layer.filter(ImageFilter.GaussianBlur(0.5))
    cover.alpha_composite(line_layer)

    logo = Image.open(LOGO_PATH).convert("RGBA")
    return cover, logo


def save_cover(cover, output_path):
    output_path.parent.mkdir(parents=True, exist_ok=True)
    cover.convert("RGB").save(output_path, quality=95)
    return output_path


def render_desktop():
    cover, logo = build_base()
    watermark = ImageOps.contain(logo, (430, 430))
    alpha = watermark.getchannel("A").point(lambda value: int(value * 0.16))
    watermark.putalpha(alpha)
    cover.alpha_composite(watermark, (1160, 110))

    draw = ImageDraw.Draw(cover)
    mono = load_font("C:/Windows/Fonts/arial.ttf", 22)
    sans = load_font("C:/Windows/Fonts/arial.ttf", 28)
    sans_bold = load_font("C:/Windows/Fonts/arialbd.ttf", 84)
    sans_small = load_font("C:/Windows/Fonts/arial.ttf", 24)
    title = load_font("C:/Windows/Fonts/georgiab.ttf", 44)

    logo_chip = Image.new("RGBA", (306, 92), (251, 248, 242, 0))
    chip_draw = ImageDraw.Draw(logo_chip)
    chip_draw.rounded_rectangle((0, 0, 306, 92), radius=28, fill=(251, 248, 242, 210), outline=(255, 255, 255, 28), width=2)
    cover.alpha_composite(logo_chip, (92, 102))

    badge = ImageOps.contain(logo, (64, 64))
    cover.alpha_composite(badge, (116, 116))
    draw.text((198, 124), "OPLURIX", font=load_font("C:/Windows/Fonts/arialbd.ttf", 36), fill="#16331F")
    draw.text((198, 160), "Ecological intelligence", font=load_font("C:/Windows/Fonts/arial.ttf", 18), fill="#536156")

    text_left = 506
    pill_y = 112
    pill_end = draw_pill(draw, text_left, pill_y, "MADAGASCAR-GROUNDED", mono, fill=(251, 248, 242, 224), outline=(255, 255, 255, 32), text_fill="#23473D")
    draw_pill(draw, pill_end + 14, pill_y, "RESEARCH • CONSERVATION • FIELD VISIBILITY", mono, fill=(232, 184, 75, 220), outline=(232, 184, 75, 92), text_fill="#16331F")

    draw.text((text_left, 214), "OPLURIX", font=sans_bold, fill="#FBF8F2")
    draw.text((text_left, 306), "Ecological Intelligence", font=title, fill="#E8B84B")

    subtitle_bottom = draw_wrapped_text(
        draw,
        "Field-grounded tools and research storytelling for conservation teams, researchers, and biodiversity projects.",
        sans,
        text_left,
        378,
        730,
        "#DCE7DC",
        8,
    )

    footer_box = (text_left, min(subtitle_bottom + 24, 478), 1248, min(subtitle_bottom + 24, 478) + 70)
    draw.rounded_rectangle(footer_box, radius=24, fill=(251, 248, 242, 228), outline=(255, 255, 255, 24), width=1)
    draw.text((text_left + 24, footer_box[1] + 22), "Conservation • Research • Field visibility • Mission-first communication", font=sans_small, fill="#23473D")

    return save_cover(cover, OUTPUT_DESKTOP)


def render_mobile_safe():
    cover, logo = build_base()

    watermark = ImageOps.contain(logo, (350, 350))
    alpha = watermark.getchannel("A").point(lambda value: int(value * 0.12))
    watermark.putalpha(alpha)
    cover.alpha_composite(watermark, (1106, 138))

    draw = ImageDraw.Draw(cover)
    mono = load_font("C:/Windows/Fonts/arial.ttf", 21)
    sans = load_font("C:/Windows/Fonts/arial.ttf", 26)
    sans_small = load_font("C:/Windows/Fonts/arial.ttf", 22)
    brand = load_font("C:/Windows/Fonts/arialbd.ttf", 74)
    title = load_font("C:/Windows/Fonts/georgiab.ttf", 42)

    safe_left = 360
    safe_right = 1280
    center_x = (safe_left + safe_right) // 2

    logo_chip = Image.new("RGBA", (286, 86), (251, 248, 242, 0))
    chip_draw = ImageDraw.Draw(logo_chip)
    chip_draw.rounded_rectangle((0, 0, 286, 86), radius=28, fill=(251, 248, 242, 214), outline=(255, 255, 255, 28), width=2)
    chip_x = center_x - 143
    chip_y = 90
    cover.alpha_composite(logo_chip, (chip_x, chip_y))

    badge = ImageOps.contain(logo, (58, 58))
    cover.alpha_composite(badge, (chip_x + 18, chip_y + 14))
    draw.text((chip_x + 92, chip_y + 18), "OPLURIX", font=load_font("C:/Windows/Fonts/arialbd.ttf", 34), fill="#16331F")
    draw.text((chip_x + 92, chip_y + 50), "Ecological intelligence", font=load_font("C:/Windows/Fonts/arial.ttf", 17), fill="#536156")

    pill_y = 200
    left_pill = "MADAGASCAR-GROUNDED"
    right_pill = "RESEARCH • FIELD VISIBILITY"
    left_bbox = draw.textbbox((0, 0), left_pill, font=mono)
    right_bbox = draw.textbbox((0, 0), right_pill, font=mono)
    left_width = (left_bbox[2] - left_bbox[0]) + 36
    right_width = (right_bbox[2] - right_bbox[0]) + 36
    gap = 16
    total_width = left_width + right_width + gap
    left_x = center_x - total_width // 2
    left_end = draw_pill(draw, left_x, pill_y, left_pill, mono, fill=(251, 248, 242, 224), outline=(255, 255, 255, 32), text_fill="#23473D")
    draw_pill(draw, left_end + gap, pill_y, right_pill, mono, fill=(232, 184, 75, 220), outline=(232, 184, 75, 92), text_fill="#16331F")

    brand_text = "OPLURIX"
    brand_bbox = draw.textbbox((0, 0), brand_text, font=brand)
    draw.text((center_x - (brand_bbox[2] - brand_bbox[0]) // 2, 278), brand_text, font=brand, fill="#FBF8F2")

    title_text = "Ecological Intelligence"
    title_bbox = draw.textbbox((0, 0), title_text, font=title)
    draw.text((center_x - (title_bbox[2] - title_bbox[0]) // 2, 366), title_text, font=title, fill="#E8B84B")

    subtitle = "Field-grounded tools and research storytelling for conservation teams, researchers, and biodiversity projects."
    subtitle_width = 700
    subtitle_left = center_x - subtitle_width // 2
    subtitle_bottom = draw_wrapped_text(
        draw,
        subtitle,
        sans,
        subtitle_left,
        438,
        subtitle_width,
        "#DCE7DC",
        8,
    )

    footer_box = (center_x - 360, min(subtitle_bottom + 20, 504), center_x + 360, min(subtitle_bottom + 20, 504) + 64)
    draw.rounded_rectangle(footer_box, radius=24, fill=(251, 248, 242, 228), outline=(255, 255, 255, 22), width=1)
    footer_text = "Conservation • Research • Field visibility"
    footer_bbox = draw.textbbox((0, 0), footer_text, font=sans_small)
    draw.text((center_x - (footer_bbox[2] - footer_bbox[0]) // 2, footer_box[1] + 18), footer_text, font=sans_small, fill="#23473D")

    return save_cover(cover, OUTPUT_MOBILE_SAFE)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--variant", choices=["desktop", "mobile-safe"], default="desktop")
    args = parser.parse_args()

    if args.variant == "mobile-safe":
        output = render_mobile_safe()
    else:
        output = render_desktop()

    print(output)


if __name__ == "__main__":
    main()
