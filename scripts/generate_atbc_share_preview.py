from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont, ImageOps


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "docs" / "assets"
BACKGROUND_PATH = ASSETS / "oplurix-website-preview.png"
LOGO_PATH = ASSETS / "oplurix-logo.png"
OUTPUT_PATH = ASSETS / "atbc-2026-share-preview.png"

WIDTH = 1200
HEIGHT = 630


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
    pad_x = 16
    pad_y = 9
    rect = (x, y, x + text_w + pad_x * 2, y + text_h + pad_y * 2)
    draw.rounded_rectangle(rect, radius=22, fill=fill, outline=outline, width=1)
    draw.text((x + pad_x, y + pad_y - 1), text, font=font, fill=text_fill)
    return rect[2]


def main():
    background = Image.open(BACKGROUND_PATH).convert("RGBA").resize((WIDTH, HEIGHT), Image.Resampling.LANCZOS)

    overlay = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    overlay_draw = ImageDraw.Draw(overlay)
    overlay_draw.rectangle((0, 0, WIDTH, HEIGHT), fill=(7, 20, 16, 34))
    overlay_draw.rounded_rectangle((420, 70, 1110, 558), radius=36, fill=(8, 28, 22, 162), outline=(255, 255, 255, 18), width=1)
    overlay = overlay.filter(ImageFilter.GaussianBlur(0.4))
    background.alpha_composite(overlay)

    draw = ImageDraw.Draw(background)
    mono = load_font("C:/Windows/Fonts/arial.ttf", 22)
    sans = load_font("C:/Windows/Fonts/arial.ttf", 24)
    sans_small = load_font("C:/Windows/Fonts/arial.ttf", 19)
    sans_bold = load_font("C:/Windows/Fonts/arialbd.ttf", 24)
    title = load_font("C:/Windows/Fonts/georgiab.ttf", 50)

    logo = Image.open(LOGO_PATH).convert("RGBA")
    logo = ImageOps.contain(logo, (76, 76))
    logo_plate = Image.new("RGBA", (112, 112), (0, 0, 0, 0))
    plate_draw = ImageDraw.Draw(logo_plate)
    plate_draw.rounded_rectangle((10, 10, 102, 102), radius=28, fill=(248, 244, 236, 224), outline=(255, 255, 255, 26), width=2)
    background.alpha_composite(logo_plate, (72, 74))
    background.alpha_composite(logo, (90, 92))

    draw.text((208, 90), "ATBC 2026 RESEARCH CAMPAIGN", font=mono, fill="#F7F2E8")
    draw.text((208, 122), "Thermal drone surveys in Madagascar", font=sans, fill="#DDE7DE")

    left_pill_end = draw_pill(
        draw,
        454,
        110,
        "ACCEPTED FOR PRESENTATION",
        mono,
        fill=(248, 244, 236, 214),
        outline=(255, 255, 255, 26),
        text_fill="#23473D",
    )
    draw_pill(
        draw,
        left_pill_end + 12,
        110,
        "ATBC 2026",
        mono,
        fill=(232, 184, 75, 214),
        outline=(232, 184, 75, 78),
        text_fill="#16331F",
    )

    title_bottom = draw_wrapped_text(
        draw,
        "Thermal drone surveys in under-monitored forests in Madagascar",
        title,
        454,
        184,
        602,
        "#FBF8F2",
        8,
    )

    body_bottom = draw_wrapped_text(
        draw,
        "Research from Manakoakora and Ihofa brings overlooked forests into international conservation visibility without leaving field reality behind.",
        sans,
        454,
        title_bottom + 18,
        560,
        "#DCE7DC",
        8,
    )

    stats = (454, min(body_bottom + 28, 460), 1092, min(body_bottom + 28, 460) + 96)
    draw.rounded_rectangle(stats, radius=24, fill=(248, 244, 236, 214), outline=(255, 255, 255, 22), width=1)
    draw.text((stats[0] + 24, stats[1] + 16), "Manakoakora + Ihofa", font=sans_bold, fill="#16331F")
    draw.text((stats[0] + 24, stats[1] + 46), "Mavic 3T thermal surveys • lemur detections across body sizes", font=sans_small, fill="#536156")
    draw.text((stats[0] + 24, stats[1] + 70), "Registration paid • travel visibility support next", font=sans_small, fill="#536156")

    frame = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    frame_draw = ImageDraw.Draw(frame)
    frame_draw.rounded_rectangle((0, 0, WIDTH, HEIGHT), radius=34, outline=(255, 255, 255, 22), width=2)
    background.alpha_composite(frame)

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    background.convert("RGB").save(OUTPUT_PATH, quality=95)
    print(OUTPUT_PATH)


if __name__ == "__main__":
    main()
