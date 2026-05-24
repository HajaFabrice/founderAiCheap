from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont, ImageOps


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "docs" / "assets"
SOURCE_PATH = ASSETS / "oplurix-website-cover.png"
PREVIEW_PATH = ASSETS / "oplurix-website-preview.png"
LOGO_PATH = ASSETS / "oplurix-logo.png"
OUTPUT_PATH = ASSETS / "oplurix-linkedin-article-field-notebooks.png"

WIDTH = 1200
HEIGHT = 627


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
    return rect


def main():
    source = Image.open(SOURCE_PATH).convert("RGBA")
    preview = Image.open(PREVIEW_PATH).convert("RGBA")
    background = preview.resize((WIDTH, HEIGHT), Image.Resampling.LANCZOS)

    shade = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    shade_draw = ImageDraw.Draw(shade)
    shade_draw.rectangle((0, 0, WIDTH, HEIGHT), fill=(9, 23, 18, 78))
    shade_draw.ellipse((-120, -130, 440, 240), fill=(232, 184, 75, 42))
    shade_draw.ellipse((690, 260, 1370, 860), fill=(11, 42, 32, 156))
    shade = shade.filter(ImageFilter.GaussianBlur(28))
    background.alpha_composite(shade)

    contour = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    contour_draw = ImageDraw.Draw(contour)
    for offset in range(-60, 320, 40):
        contour_draw.arc((560 + offset, -80 + offset, 1330 + offset, 620 + offset), start=205, end=335, fill=(255, 255, 255, 20), width=2)
    contour_draw.line((530, 76, 1088, 76), fill=(255, 255, 255, 18), width=1)
    contour_draw.line((530, 548, 1088, 548), fill=(255, 255, 255, 16), width=1)
    contour = contour.filter(ImageFilter.GaussianBlur(0.5))
    background.alpha_composite(contour)

    notebook = (74, 88, 408, 542)
    paper = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    paper_draw = ImageDraw.Draw(paper)
    paper_draw.rounded_rectangle(notebook, radius=30, fill=(249, 244, 236, 242), outline=(255, 255, 255, 36), width=2)
    paper_draw.rounded_rectangle((notebook[0] + 18, notebook[1] + 18, notebook[2] - 18, notebook[3] - 18), radius=22, outline=(19, 35, 29, 24), width=1)
    for y in range(notebook[1] + 112, notebook[3] - 34, 44):
        paper_draw.line((notebook[0] + 34, y, notebook[2] - 34, y), fill=(57, 90, 78, 44), width=2)
    for y in range(notebook[1] + 42, notebook[3] - 40, 38):
        paper_draw.ellipse((notebook[0] - 8, y, notebook[0] + 12, y + 20), fill=(255, 255, 255, 235), outline=(19, 35, 29, 18), width=1)
    paper = paper.filter(ImageFilter.GaussianBlur(0.2))
    background.alpha_composite(paper)

    draw = ImageDraw.Draw(background)

    mono = load_font("C:/Windows/Fonts/arial.ttf", 20)
    sans = load_font("C:/Windows/Fonts/arial.ttf", 24)
    sans_small = load_font("C:/Windows/Fonts/arial.ttf", 17)
    sans_bold = load_font("C:/Windows/Fonts/arialbd.ttf", 24)
    entry_bold = load_font("C:/Windows/Fonts/arialbd.ttf", 20)
    brand = load_font("C:/Windows/Fonts/arialbd.ttf", 32)
    title = load_font("C:/Windows/Fonts/georgiab.ttf", 52)
    notebook_title = load_font("C:/Windows/Fonts/georgiab.ttf", 22)
    notebook_quote = load_font("C:/Windows/Fonts/georgiab.ttf", 17)

    logo_chip = Image.new("RGBA", (248, 82), (0, 0, 0, 0))
    chip_draw = ImageDraw.Draw(logo_chip)
    chip_draw.rounded_rectangle((0, 0, 248, 82), radius=28, fill=(250, 247, 239, 218), outline=(255, 255, 255, 26), width=2)
    background.alpha_composite(logo_chip, (72, 26))

    logo = Image.open(LOGO_PATH).convert("RGBA")
    logo = ImageOps.contain(logo, (56, 56))
    background.alpha_composite(logo, (92, 39))
    draw.text((160, 42), "OPLURIX", font=brand, fill="#16331F")
    draw.text((160, 74), "Field stories, made public", font=sans_small, fill="#536156")

    right_panel = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    panel_draw = ImageDraw.Draw(right_panel)
    panel_draw.rounded_rectangle((430, 116, 1114, 566), radius=34, fill=(10, 29, 22, 154), outline=(255, 255, 255, 18), width=1)
    right_panel = right_panel.filter(ImageFilter.GaussianBlur(0.6))
    background.alpha_composite(right_panel)
    draw = ImageDraw.Draw(background)

    draw.text((notebook[0] + 34, notebook[1] + 34), "FIELD NOTEBOOK", font=mono, fill="#6F8458")
    draw.text((notebook[0] + 34, notebook[1] + 72), "Work that stayed\nhidden too long", font=notebook_title, fill="#16331F", spacing=4)

    entries = [
        "Beza Mahafaly\nLemur focal follows",
        "Ambatotsirongorongo\nHerpetofauna surveys",
        "Ihofa Forest\nMammal transects",
    ]

    entry_y = notebook[1] + 172
    for entry in entries:
        draw.ellipse((notebook[0] + 36, entry_y + 6, notebook[0] + 52, entry_y + 22), fill="#E8B84B")
        parts = entry.split("\n")
        draw.text((notebook[0] + 66, entry_y), parts[0], font=entry_bold, fill="#16331F")
        draw.text((notebook[0] + 66, entry_y + 24), parts[1], font=sans_small, fill="#536156")
        entry_y += 84

    draw.text((notebook[0] + 34, notebook[3] - 56), "Real observations.\nNo system to share them.", font=notebook_quote, fill="#23473D", spacing=3)

    pill = draw_pill(
        draw,
        454,
        92,
        "LINKEDIN ARTICLE VISUAL",
        mono,
        fill=(249, 244, 236, 216),
        outline=(255, 255, 255, 26),
        text_fill="#23473D",
    )
    draw_pill(
        draw,
        pill[2] + 12,
        92,
        "MADAGASCAR • FIELD COMMUNICATION",
        mono,
        fill=(232, 184, 75, 208),
        outline=(232, 184, 75, 76),
        text_fill="#16331F",
    )

    title_bottom = draw_wrapped_text(
        draw,
        "My most valuable conservation work never left my notebooks.",
        title,
        454,
        162,
        642,
        "#FBF8F2",
        10,
    )

    body_bottom = draw_wrapped_text(
        draw,
        "The Expert-to-Influencer Content Engine helps field biologists and conservation professionals turn raw field experience into clear, credible public content.",
        sans,
        454,
        title_bottom + 18,
        614,
        "#DBE7DD",
        8,
    )

    footer = (454, min(body_bottom + 28, 474), 1094, min(body_bottom + 28, 474) + 86)
    draw.rounded_rectangle(footer, radius=24, fill=(248, 244, 236, 214), outline=(255, 255, 255, 24), width=1)
    draw.text((footer[0] + 22, footer[1] + 16), "Built from real field work in Madagascar", font=sans_bold, fill="#16331F")
    draw.text((footer[0] + 22, footer[1] + 46), "Frameworks • templates • AI prompts • 15-minute workflow", font=sans_small, fill="#536156")

    frame = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    frame_draw = ImageDraw.Draw(frame)
    frame_draw.rounded_rectangle((0, 0, WIDTH, HEIGHT), radius=34, outline=(255, 255, 255, 22), width=2)
    background.alpha_composite(frame)

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    background.convert("RGB").save(OUTPUT_PATH, quality=95)
    print(OUTPUT_PATH)


if __name__ == "__main__":
    main()
