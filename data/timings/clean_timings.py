import csv
import os
import re

def parse_hex_id(s):
    if s == 'None' or not s or s == 'n/a':
        return 'None'
    hex_parts = re.findall(r'[0-9A-Fa-f]{2}', s)
    if not hex_parts:
        return s
    return '0x' + ''.join(p.upper() for p in hex_parts)

def clean_csv(input_path, output_path):
    COLUMN_MAPPING = {
        'DMT ID': 'dmt_id',
        'Std 2-Byte Code': 'std_id',
        'CVT 3-Byte Code': 'cvt_id',
        'RB': 'rb',
        'PCLK (MHz)': 'pixclk_khz',
        'H-Freq (kHz)': 'h_freq_hz',
        'V-Freq (Hz)': 'v_freq_mhz',
        'H-Addr (px)': 'h_active',
        'H-FP (px)': 'h_fp',
        'H-Sync (px)': 'h_sync',
        'H-BP (px)': 'h_bp',
        'V-Addr (ln)': 'v_active',
        'V-FP (ln)': 'v_fp',
        'V-Sync (ln)': 'v_sync',
        'V-BP (ln)': 'v_bp',
        'H-Total (px)': 'h_total',
        'V-Total (ln)': 'v_total'
    }

    with open(input_path, 'r', newline='') as f:
        reader = csv.DictReader(f)
        rows = []
        for row in reader:
            # Create new row and apply initial mapping
            new_row = {COLUMN_MAPPING[k]: row[k] for k in COLUMN_MAPPING}
            
            # 1. Standardize IDs
            new_row['dmt_id'] = parse_hex_id(new_row['dmt_id'])
            new_row['std_id'] = parse_hex_id(new_row['std_id'])
            new_row['cvt_id'] = parse_hex_id(new_row['cvt_id'])

            # 2. Booleans
            new_row['interlaced'] = 'true' if row.get('Scan', '').upper() == 'INTERLACED' else 'false'
            new_row['h_pol'] = 'true' if row['H-Pol'].upper() == 'POSITIVE' else 'false'
            new_row['v_pol'] = 'true' if row['V-Pol'].upper() == 'POSITIVE' else 'false'

            # 3. Borders (Use single side as requested)
            new_row['h_border'] = int(row['H-Left-Border (px)'])
            new_row['v_border'] = int(row['V-Top-Border (ln)'])

            # 4. Standard
            method = row.get('Method', '')
            is_rb = row['RB'].lower() == 'true'
            if is_rb:
                new_row['standard'] = 'CVT_V2' if 'v2' in method else 'CVT_V1'
            elif 'CVT Compliant' in method:
                new_row['standard'] = 'CVT'
            else:
                new_row['standard'] = 'NO_CVT'

            # 5. Numeric fields to int
            new_row['pixclk_khz'] = int(float(new_row['pixclk_khz']) * 1000)
            new_row['h_freq_hz'] = int(float(new_row['h_freq_hz']) * 1000)
            new_row['v_freq_mhz'] = int(float(new_row['v_freq_mhz']) * 1000)
            
            # Standard components to int
            for field in ['h_active', 'h_fp', 'h_sync', 'h_bp', 'v_active', 'v_fp', 'v_sync', 'v_bp', 'h_total', 'v_total']:
                new_row[field] = int(new_row[field])
                
            rows.append(new_row)

    fieldnames = [
        'dmt_id', 'std_id', 'cvt_id', 'rb', 'standard', 'pixclk_khz',
        'h_freq_hz', 'v_freq_mhz', 'interlaced', 'h_pol', 'v_pol',
        'h_total', 'h_active', 'h_fp', 'h_sync', 'h_bp',
        'v_total', 'v_active', 'v_fp', 'v_sync', 'v_bp',
        'h_border', 'v_border'
    ]

    with open(output_path, 'w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)

if __name__ == "__main__":
    clean_csv('data/timings/dmt_timings.csv', 'data/timings/dmt_timings_cleaned.csv')
