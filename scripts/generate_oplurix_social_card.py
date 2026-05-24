from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont, ImageOps


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "docs" / "assets"
LOGO_PATH = ASSETS / "oplurix-logo.png"
OUTPUT_PATH = ASSETS / "oplurix-social-card.png"

WIDTH = 1200
HEIGHT = 630


def load_font(path: str, size: int) -> ImageFont.FreeTypeFont:
    return ImageFont.truetype(path, size=size)


def draw_wrapped_text(draw: ImageDraw.ImageDraw, text: str, font, x: int, y: int, max_width: int, fill, line_gap: int):
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


def draw_pill(draw: ImageDraw.ImageDraw, x: int, y: int, text: str, font, fill, outline, text_fill):
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
    card = Image.new("RGBA", (WIDTH, HEIGHT), "#16331F")

    background = ImageDraw.Draw(card)
    background.rectangle((0, 0, WIDTH, HEIGHT), fill="#16331F")

    glow = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow)
    glow_draw.ellipse((-80, -140, 430, 360), fill=(232, 184, 75, 72))
    glow_draw.ellipse((740, 260, 1250, 820), fill=(127, 168, 130, 92))
    glow_draw.ellipse((850, -90, 1290, 310), fill=(255, 255, 255, 26))
    glow = glow.filter(ImageFilter.GaussianBlur(28))
    card.alpha_composite(glow)

    panel = (72, 74, 422, 556)
    background.rounded_rectangle(panel, radius=36, fill=(251, 248, 242, 245), outline=(255, 255, 255, 42), width=2)

    logo = Image.open(LOGO_PATH).convert("RGBA")
    logo = ImageOps.contain(logo, (270, 270))
    logo_shadow = Image.new("RGBA", logo.size, (0, 0, 0, 0))
    shadow_draw = ImageDraw.Draw(logo_shadow)
    shadow_draw.rounded_rectangle((8, 8, logo.width - 2, logo.height - 2), radius=28, fill=(9, 20, 15, 55))
    logo_shadow = logo_shadow.filter(ImageFilter.GaussianBlur(12))

    logo_x = panel[0] + (panel[2] - panel[0] - logo.width) // 2
    logo_y = panel[1] + 60
    card.alpha_composite(logo_shadow, (logo_x - 10, logo_y + 10))
    card.alpha_composite(logo, (logo_x, logo_y))

    draw = ImageDraw.Draw(card)

    mono_font = load_font("C:/Windows/Fonts/arial.ttf", 24)
    sans_font = load_font("C:/Windows/Fonts/arial.ttf", 24)
    sans_small = load_font("C:/Windows/Fonts/arial.ttf", 20)
    sans_bold = load_font("C:/Windows/Fonts/arialbd.ttf", 26)
    title_font = load_font("C:/Windows/Fonts/georgiab.ttf", 58)
    brand_font = load_font("C:/Windows/Fonts/arialbd.ttf", 54)

    draw.text((panel[0] + 38, panel[3] - 132), "OPLURIX", font=brand_font, fill="#16331F")
    draw.text((panel[0] + 40, panel[3] - 86), "Ecological intelligence", font=sans_font, fill="#536156")

    text_left = 482
    text_right = 1094
    pill_y = 98

    pill_x = draw_pill(draw, text_left, pill_y, "MADAGASCAR-GROUNDED", mono_font, fill=(251, 248, 242, 230), outline=(255, 255, 255, 36), text_fill="#23473D")
    draw_pill(draw, pill_x + 12, pill_y, "RESEARCH • PRODUCTS • SERVICES", mono_font, fill=(232, 184, 75, 34), outline=(232, 184, 75, 68), text_fill="#F8E7B9")

    title_y = draw_wrapped_text(
        draw,
        "Turn real conservation work into public communication people trust.",
        title_font,
        text_left,
        170,
        600,
        "#FBF8F2",
        8,
    )

    body_y = draw_wrapped_text(
        draw,
        "Field-grounded digital tools, research visibility, and ecological intelligence for researchers, NGO teams, and biodiversity projects.",
        sans_font,
        text_left,
        title_y + 18,
        560,
        "#DDE7DE",
        8,
    )

    footer_top = min(body_y + 20, 470)
    footer = (text_left, footer_top, 1120, 566)
    draw.rounded_rectangle(footer, radius=24, fill=(255, 255, 255, 14), outline=(255, 255, 255, 28), width=1)
    draw.text((text_left + 26, footer_top + 18), "Live now", font=sans_small, fill="#E8B84B")
    draw.text((text_left + 126, footer_top + 16), "Expert-to-Influencer Content Engine", font=sans_bold, fill="#16331F")
    draw.text((text_left + 26, footer_top + 46), "Research campaign, field notes, and project proof available on-site.", font=sans_small, fill="#536156")

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    card.convert("RGB").save(OUTPUT_PATH, quality=95)
    print(OUTPUT_PATH)


if __name__ == "__main__":
    main()
