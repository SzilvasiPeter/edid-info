import csv
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = ROOT / "data/dmt/timings_cleaned.csv"
DMT_RS_PATH = ROOT / "src/base/dmt.rs"


def parse_num(value: str) -> int:
    if value.startswith("0x"):
        return int(value, 16)
    return int(value)


def fmt_dec(value: int) -> str:
    text = str(value)
    parts = []
    while text:
        parts.append(text[-3:])
        text = text[:-3]
    return "_".join(reversed(parts))


def fmt_hex(value: int, width: int) -> str:
    text = f"{value:0{width}X}"
    if len(text) <= 4:
        return f"0x{text}"
    first = len(text) % 4
    groups = []
    if first:
        groups.append(text[:first])
    i = first
    while i < len(text):
        groups.append(text[i : i + 4])
        i += 4
    return "0x" + "_".join(groups)


def pol(val: str) -> str:
    return "Positive" if val == "true" else "Negative"


def make_entry(row: dict[str, str]) -> str:
    dmt_id = parse_num(row["dmt_id"])
    std_code = row["std_code"]
    cvt_code = row["cvt_code"]
    pixclk = parse_num(row["pixclk_khz"])
    interlaced = row["interlaced"]

    h_total = parse_num(row["h_total"])
    h_active = parse_num(row["h_active"])
    h_blank = h_total - h_active
    h_fp = parse_num(row["h_fp"])
    h_sync = parse_num(row["h_sync"])
    h_border = parse_num(row["h_border"])

    v_total = parse_num(row["v_total"])
    v_active = parse_num(row["v_active"])
    v_blank = v_total - v_active
    v_fp = parse_num(row["v_fp"])
    v_sync = parse_num(row["v_sync"])
    v_border = parse_num(row["v_border"])

    std_val = "None" if std_code == "None" else f"Some({fmt_hex(parse_num(std_code), 4)})"
    cvt_val = "None" if cvt_code == "None" else f"Some({fmt_hex(parse_num(cvt_code), 6)})"

    return (
        "    Dmt {\n"
        f"        id: {fmt_hex(dmt_id, 2)},\n"
        f"        std_code: {std_val},\n"
        f"        cvt_code: {cvt_val},\n"
        f"        pixel_clock_khz: {fmt_dec(pixclk)},\n"
        f"        interlaced: {interlaced},\n"
        f"        horizontal: Timing::new({h_active}, {h_blank}, {h_fp}, {h_sync}, {h_border}),\n"
        f"        vertical: Timing::new({v_active}, {v_blank}, {v_fp}, {v_sync}, {v_border}),\n"
        "        sync: SyncPolarity {\n"
        f"            horizontal: Polarity::{pol(row['h_pol'])},\n"
        f"            vertical: Polarity::{pol(row['v_pol'])},\n"
        "        },\n"
        "    },\n"
    )


def build_array(rows: list[dict[str, str]]) -> str:
    out = [f"pub const DMT_ARRAY: [Dmt; {len(rows)}] = [\n"]
    for row in rows:
        out.append(make_entry(row))
    out.append("];\n")
    return "".join(out)


def replace_array(src: str, new_array: str) -> str:
    pattern = re.compile(r"pub const DMT_ARRAY: \[Dmt; \d+\] = \[(?s:.*?)\];\\n")
    match = pattern.search(src)
    if match is None:
        raise RuntimeError("DMT_ARRAY block not found")
    return src[: match.start()] + new_array + src[match.end() :]


def main() -> None:
    with CSV_PATH.open("r", newline="") as f:
        rows = list(csv.DictReader(f))

    src = DMT_RS_PATH.read_text()
    out = replace_array(src, build_array(rows))
    DMT_RS_PATH.write_text(out)


if __name__ == "__main__":
    main()
