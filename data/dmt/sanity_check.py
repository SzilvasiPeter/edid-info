import csv
import os


def check_std_code(row):
    std_code_raw = row["std_code"]
    if std_code_raw == "None":
        return True

    code = int(std_code_raw, 16)
    byte1 = (code >> 8) & 0xFF
    byte2 = code & 0xFF

    h_active = int(row["h_active"])
    v_active = int(row["v_active"])
    v_freq = int(row["v_freq_mhz"]) / 1000.0

    # Byte 1: (Horizontal addressable pixels / 8) - 31
    expected_byte1 = (h_active // 8) - 31
    if byte1 != expected_byte1:
        print(
            f"DMT {row['dmt_id']}: std_code byte1 mismatch. Expected {expected_byte1:02X}, got {byte1:02X}"
        )
        return False

    # Byte 2 bits 7-6: Aspect Ratio
    # 00: 16:10, 01: 4:3, 10: 5:4, 11: 16:9
    ar_bits = (byte2 >> 6) & 0x03

    # Simple aspect ratio check (might need more robust logic for edge cases)
    ratio = h_active / v_active
    if ar_bits == 0b00:  # 16:10
        expected_ratio = 16 / 10
    elif ar_bits == 0b01:  # 4:3
        expected_ratio = 4 / 3
    elif ar_bits == 0b10:  # 5:4
        expected_ratio = 5 / 4
    elif ar_bits == 0b11:  # 16:9
        expected_ratio = 16 / 9
    else:
        expected_ratio = 0

    if abs(ratio - expected_ratio) > 0.01:
        print(
            f"DMT {row['dmt_id']}: std_code aspect ratio mismatch. Bits {ar_bits:02b} for {h_active}x{v_active}"
        )
        return False

    # Byte 2 bits 5-0: Refresh Rate - 60
    expected_refresh = int(round(v_freq)) - 60
    actual_refresh = byte2 & 0x3F
    if actual_refresh != expected_refresh:
        # Some DMT entries might have slightly off frequencies in the CSV
        if abs(actual_refresh - expected_refresh) > 1:
            print(
                f"DMT {row['dmt_id']}: std_code refresh mismatch. Expected {expected_refresh}, got {actual_refresh}"
            )
            return False

    return True


def check_cvt_code(row):
    cvt_code_raw = row["cvt_code"]
    if cvt_code_raw == "None":
        return True

    code = int(cvt_code_raw, 16)
    byte1 = (code >> 16) & 0xFF
    byte2 = (code >> 8) & 0xFF
    byte3 = code & 0xFF

    v_active = int(row["v_active"])
    h_active = int(row["h_active"])
    v_freq = int(row["v_freq_mhz"]) / 1000.0
    rb = row["rb"].lower() == "true"

    # 12 Bit Value Stored = [(Addressable Lines per Field / 2) - 1]
    expected_lines_val = (v_active // 2) - 1
    actual_lines_val = (byte1) | ((byte2 & 0xF0) << 4)
    if actual_lines_val != expected_lines_val:
        print(
            f"DMT {row['dmt_id']}: cvt_code lines mismatch. Expected {expected_lines_val:03X}, got {actual_lines_val:03X}"
        )
        return False

    # Byte 2 bits 3-2: Aspect Ratio
    # 00: 4:3, 01: 16:9, 10: 16:10, 11: 15:9
    ar_bits = (byte2 >> 2) & 0x03
    ratio = h_active / v_active
    if ar_bits == 0b00:
        expected_ratio = 4 / 3
    elif ar_bits == 0b01:
        expected_ratio = 16 / 9
    elif ar_bits == 0b10:
        expected_ratio = 16 / 10
    elif ar_bits == 0b11:
        expected_ratio = 15 / 9
    else:
        expected_ratio = 0

    if abs(ratio - expected_ratio) > 0.01:
        print(
            f"DMT {row['dmt_id']}: cvt_code aspect ratio mismatch. Bits {ar_bits:02b} for {h_active}x{v_active}"
        )
        return False

    # Byte 3 bits 6-5: Preferred Vertical Rate
    # 00: 50Hz, 01: 60Hz, 10: 75Hz, 11: 85Hz
    pref_rate_bits = (byte3 >> 5) & 0x03
    pref_rates = {0: 50, 1: 60, 2: 75, 3: 85}
    expected_pref_rate = pref_rates[pref_rate_bits]

    if abs(v_freq - expected_pref_rate) > 1.0:
        print(f"DMT {row['dmt_id']}: cvt_code preferred rate mismatch. Expected ~{expected_pref_rate}Hz, got {v_freq:.2f}Hz")

    # Byte 3 bits 4-0: Supported Vertical Rate and Blanking Style
    # bit 4: 50Hz Standard, bit 3: 60Hz Standard, bit 2: 75Hz Standard, 
    # bit 1: 85Hz Standard, bit 0: 60Hz Reduced Blanking
    supported_bits = byte3 & 0x1F
    v_freq_rounded = int(round(v_freq))

    if rb:
        if v_freq_rounded == 60 and not (supported_bits & 0x01):
            print(f"DMT {row['dmt_id']}: cvt_code signals RB but 60Hz RB bit is not set")
    else:
        bit_map = {50: 0x10, 60: 0x08, 75: 0x04, 85: 0x02}
        expected_bit = bit_map.get(v_freq_rounded)
        if expected_bit and not (supported_bits & expected_bit):
            print(f"DMT {row['dmt_id']}: cvt_code bit for {v_freq_rounded}Hz Standard not set")

    return True

def check_timing_rules(row):
    dmt_id = row["dmt_id"]
    h_active = int(row["h_active"])
    h_total = int(row["h_total"])
    h_fp = int(row["h_fp"])
    h_sync = int(row["h_sync"])
    h_border = int(row["h_border"])
    v_active = int(row["v_active"])
    v_total = int(row["v_total"])
    v_fp = int(row["v_fp"])
    v_sync = int(row["v_sync"])
    v_border = int(row["v_border"])
    interlaced = row["interlaced"].lower() == "true"
    rb = row["rb"].lower() == "true"
    formula = row["formula"]
    h_pol = row["h_pol"].lower() == "true"
    v_pol = row["v_pol"].lower() == "true"
    pixclk_khz = int(row["pixclk_khz"])
    v_freq_mhz = int(row["v_freq_mhz"])

    h_blank = h_total - h_active
    v_blank = v_total - v_active
    target_v_freq = v_freq_mhz / 1000.0

    # 1. Refresh rate inconsistency (> 0.5%)
    calc_v_freq = (pixclk_khz * 1000.0) / (h_total * v_total)
    diff_percent = abs(calc_v_freq - target_v_freq) / target_v_freq
    if diff_percent > 0.005 and not interlaced:
        print(
            f"DMT {dmt_id}: Refresh rate mismatch. Calc: {calc_v_freq:.3f} Hz, Target: {target_v_freq:.3f} Hz ({diff_percent:.2%} diff)"
        )

    # 2. h_total % 8 != 0
    if h_total % 8 != 0:
        print(f"DMT {dmt_id}: h_total {h_total} is not divisible by 8")

    # 3. h_fp + h_sync >= h_blank -> impossible back porch
    if h_fp + h_sync >= h_blank:
        print(
            f"DMT {dmt_id}: h_fp ({h_fp}) + h_sync ({h_sync}) >= h_blank ({h_blank}) -> Non-positive back porch"
        )

    # 4. Horizontal Frequency > 150 kHz
    h_freq_khz = pixclk_khz / h_total
    if h_freq_khz > 150:
        print(
            f"DMT {dmt_id}: High Bandwidth (> 150 kHz horizontal frequency): {h_freq_khz:.2f} kHz"
        )

    # 5. h_blank < 80 pixels
    if h_blank < 80:
        print(
            f"DMT {dmt_id}: h_blank ({h_blank}) < 80 pixels (Non-Standard/Proprietary)"
        )

    # 6. v_sync > v_blank
    if v_sync > v_blank:
        print(f"DMT {dmt_id}: v_sync ({v_sync}) > v_blank ({v_blank}) (Invalid)")

    # 7. Porch or sync is zero
    if any(val == 0 for val in [h_fp, h_sync, v_fp, v_sync]):
        print(f"DMT {dmt_id}: Non-standard timing with zero porch or sync value")

    # 8. rb is true and v_blank significantly higher than 62 for 60Hz signal
    if rb and 59 < target_v_freq < 61:
        if v_blank > 62:
            print(
                f"DMT {dmt_id}: rb=true for 60Hz but v_blank ({v_blank}) > 62 (Not following CVT-RBv2 strictly)"
            )

    # 9. h_active % 8 != 0 -> Non-CVT Compliant (except known exceptions)
    if h_active % 8 != 0 and dmt_id not in ["0x51", "0x56"]:
        if formula != "NO_CVT":
            print(
                f"DMT {dmt_id}: h_active {h_active} not divisible by 8 but formula is {formula}"
            )

    # 10. interlaced is true -> rb must be false
    if interlaced and rb:
        print(f"DMT {dmt_id}: Interlaced cannot have reduced blanking")

    # 11 & 12. RB and h_blanking determining CVT version
    if rb:
        if h_blank == 160 and formula != "CVT_V1":
            print(
                f"DMT {dmt_id}: rb is true and h_blank is 160, expected CVT_V1, got {formula}"
            )
        if h_blank == 80 and formula != "CVT_V2":
            print(
                f"DMT {dmt_id}: rb is true and h_blank is 80, expected CVT_V2, got {formula}"
            )

    # 13. h_pol true, v_pol true -> Legacy DMT; formula NO_CVT, rb false
    if h_pol and v_pol:
        if formula != "NO_CVT" or rb:
            print(
                f"DMT {dmt_id}: h_pol and v_pol both true implies Legacy DMT (NO_CVT, no rb). Got {formula}, rb={rb}"
            )

    # 14. rb true and h_sync 32 -> strong indicator of CVT RB
    if rb and h_sync != 32 and formula in ["CVT_V1", "CVT_V2"]:
        print(
            f"DMT {dmt_id}: {formula} with rb=true usually has h_sync=32, got {h_sync}"
        )

    # 15 & 16. Vertical front porch constants for CVT RB
    if rb:
        if formula == "CVT_V1" and v_fp != 3:
            print(f"DMT {dmt_id}: CVT_V1 RB expected v_fp=3, got {v_fp}")
        if formula == "CVT_V2" and v_fp != 48:
            print(f"DMT {dmt_id}: CVT_V2 RB expected v_fp=48, got {v_fp}")

    # 17. Borders -> Legacy DMT, cannot be CVT
    if (h_border > 0 or v_border > 0) and formula != "NO_CVT":
        print(f"DMT {dmt_id}: Borders present but formula is {formula}")


def main():
    csv_path = os.path.join(os.path.dirname(__file__), "timings_cleaned.csv")
    with open(csv_path, "r") as f:
        reader = csv.DictReader(f)
        for row in reader:
            check_std_code(row)
            check_cvt_code(row)
            check_timing_rules(row)


if __name__ == "__main__":
    main()
