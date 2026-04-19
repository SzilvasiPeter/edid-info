import re
import os

def parse_timing(block):
    lines = [line.strip() for line in block.split('\n')]
    
    # Extract Resolution section to check for Reduced Blanking
    res_section = ""
    for line in lines:
        if line.startswith("Resolution:"):
            res_section = line.split("Resolution:")[1].strip()
            break
    reduced_blanking = "true" if "REDUCED BLANKING" in res_section.upper() else "false"

    # Simplified Method matching
    methods = [
        "*** NOT CVT COMPLIANT ***",
        "CVT Compliant",
        "CVT Reduced Blanking",
        "Generated using CVT (Reduced Blanking) Formula",
        "Generated using CVT (Reduced Blanking v2) Formula"
    ]
    method = "n/a"
    for m in methods:
        if m in block:
            method = m
            break

    def get_line_index(parts):
        for i, line in enumerate(lines):
            clean_line = line.replace('//', '').strip('| ').lower()
            if all(p.lower() in clean_line for p in parts):
                return i
        return -1

    def find_val(parts):
        idx = get_line_index(parts)
        if idx == -1: return "n/a"
        
        # Try same line
        match = re.search(r'=\s*([^;|\n]+)', lines[idx])
        if match and match.group(1).strip() and match.group(1).strip() != "|":
            val = match.group(1).strip()
            if val: return val
            
        # Try line above (common for Pixel Clock and Timing Name)
        if idx > 0:
            # Check for value followed by semicolon
            match = re.search(r'([^;|\n]+);', lines[idx-1])
            if match: return match.group(1).strip()
            # Special case for Timing Name without semicolon
            if "timing" in "".join(parts).lower() and "name" in "".join(parts).lower():
                val = lines[idx-1].strip('| ').strip()
                if val and "Resolution:" not in val and "Detailed" not in val and "VESA" not in val:
                    return val
        
        # Try line below (for some messy wraps)
        if idx < len(lines) - 1:
            match = re.search(r'=\s*([^;|\n]+)', lines[idx+1])
            if match and match.group(1).strip() and match.group(1).strip() != "|":
                return match.group(1).strip()

        return "n/a"

    def find_unit_val(parts, unit):
        # Special case for Ver Total Time which is often outside the table
        if parts == ['Ver', 'Total', 'Time']:
            # Look for digits followed by optional pipe and "lines"
            m = re.search(r'Ver Total Time.*?(\d+)\s+(?:\|\s*)?lines', block, re.IGNORECASE | re.DOTALL)
            if m: return m.group(1)

        idx = get_line_index(parts)
        if idx == -1: return "n/a"
        
        # Look in the current line and the next 4 lines
        for i in range(idx, min(idx + 5, len(lines))):
            # Try to find a number followed by the unit, allowing for pipe separators
            # We use a negative lookbehind for a dot to avoid matching the decimal part of a frequency
            match = re.search(fr'(?<![\d.])(\d+)\s+(?:\|\s*)?{unit}', lines[i], re.IGNORECASE)
            if match:
                return match.group(1)
            # Fallback for when there's an equals sign
            match = re.search(fr'=\s*(\d+)\s+(?:\|\s*)?{unit}', lines[i], re.IGNORECASE)
            if match:
                return match.group(1)
        
        return "n/a"

    timing_name = find_val(["Timing", "Name"])
    if timing_name == "n/a" or not timing_name:
        timing_name = res_section if res_section else "Unknown"

    # Extract EDID ID components
    edid_line = ""
    for line in lines:
        if "EDID ID:" in line:
            edid_line = line
            break
    
    dmt_id = "n/a"
    std_code = "n/a"
    cvt_code = "n/a"
    if edid_line:
        dmt_m = re.search(r'DMT ID:\s*([^;]+)', edid_line)
        std_m = re.search(r'Std\. 2 Byte Code:\s*([^;]+)', edid_line)
        cvt_m = re.search(r'CVT 3 Byte Code:\s*([^;]+)', edid_line)
        if dmt_m: dmt_id = dmt_m.group(1).strip()
        if std_m: std_code = std_m.group(1).strip()
        if cvt_m: cvt_code = cvt_m.group(1).strip()

    data = [
        timing_name, dmt_id, std_code, cvt_code, reduced_blanking, method,
        find_val(['Pixel', 'Clock']), find_val(['Hor', 'Frequency']), find_val(['Ver', 'Frequency']), 
        find_val(['Scan', 'Type']), find_val(['Hor', 'Sync', 'Polarity']), find_val(['Ver', 'Sync', 'Polarity']),
        find_unit_val(['Hor', 'Total', 'Time'], 'Pixels'), find_unit_val(['Hor', 'Addr', 'Time'], 'Pixels'),
        find_unit_val(['H', 'Front', 'Porch'], 'Pixels'), find_unit_val(['Hor', 'Sync', 'Time'], 'Pixels'),
        find_unit_val(['H', 'Back', 'Porch'], 'Pixels'),
        find_unit_val(['Ver', 'Total', 'Time'], 'lines'), find_unit_val(['Ver', 'Addr', 'Time'], 'lines'),
        find_unit_val(['V', 'Front', 'Porch'], 'lines'), find_unit_val(['Ver', 'Sync', 'Time'], 'lines'),
        find_unit_val(['V', 'Back', 'Porch'], 'lines')
    ]
    
    headers = [
        "Timing Name", "DMT ID", "Std 2-Byte Code", "CVT 3-Byte Code", "RB", "Method",
        "PCLK (MHz)", "H-Freq (kHz)", "V-Freq (Hz)", "Scan", 
        "H-Pol", "V-Pol", "H-Total (px)", "H-Addr (px)", "H-FP (px)", "H-Sync (px)", "H-BP (px)",
        "V-Total (ln)", "V-Addr (ln)", "V-FP (ln)", "V-Sync (ln)", "V-BP (ln)"
    ]
    
    # Validation: Only Std and CVT codes can be n/a
    for i, val in enumerate(data):
        if val == "n/a" and i not in [2, 3]:
            print(f"CRITICAL VALIDATION ERROR: Column '{headers[i]}' is n/a for timing '{timing_name}'")
            
    return data

def generate_table(data_rows):
    headers = [
        "Timing Name", "DMT ID", "Std 2-Byte Code", "CVT 3-Byte Code", "RB", "Method",
        "PCLK (MHz)", "H-Freq (kHz)", "V-Freq (Hz)", "Scan", 
        "H-Pol", "V-Pol", "H-Total (px)", "H-Addr (px)", "H-FP (px)", "H-Sync (px)", "H-BP (px)",
        "V-Total (ln)", "V-Addr (ln)", "V-FP (ln)", "V-Sync (ln)", "V-BP (ln)"
    ]
    
    lines = []
    lines.append("| " + " | ".join(headers) + " |")
    lines.append("| " + " | ".join([":---"] * len(headers)) + " |")
    
    for row in data_rows:
        lines.append("| " + " | ".join(row) + " |")
    
    return "\n".join(lines)

def main():
    input_file = 'test.md'
    output_file = 'test_transposed.md'
    
    if not os.path.exists(input_file):
        print(f"Error: {input_file} not found.")
        return

    print(f"Processing {input_file}...")
    with open(input_file, 'r') as f:
        content = f.read()
        
    blocks = re.split(r'VESA MONITOR TIMING STANDARD', content)
    all_data = []
    for block in blocks:
        if 'Resolution:' in block:
            all_data.append(parse_timing(block))
    
    if all_data:
        with open(output_file, 'w') as f:
            f.write(generate_table(all_data))
        print(f"Successfully created {output_file}")

if __name__ == "__main__":
    main()
