#!/usr/bin/env python3

import os
import re
import glob

# Sample
"""
const HEXAGRAM_GLYPHS_01: [(&str, &[u8]); 3] = [
    ("甲骨文元", include_bytes!("../assets/images/gd01_甲骨文元.jpg")),
    ("甲骨文利", include_bytes!("../assets/images/gd01_甲骨文利.jpg")),
    ("", &[]),
];
"""

def print_one_hexagram(order: str, tps: list[tuple[str, str]]):
    if not tps: return

    print(f"const HEXAGRAM_GLYPHS_{order}: [(&str, &[u8]); 3] = [")
    for tp in tps:
        print(f'    ("{tp[0]}", include_bytes!("../{tp[1]}")),')
    if len(tps) < 3:
        for i in range(3 - len(tps)):
            print('    ("", &[]),')
    print("];")

fn_pat = re.compile(r".*/gd(\d+)_(.+)\.jpg")

current_order = '00'
tps = []
orders = []
for file in sorted(glob.glob("assets/images/gd*.jpg")):
    r = fn_pat.match(file)
    order = r.groups()[0]
    name = r.groups()[1]

    if current_order != order:
        # found new one, print out last one
        print_one_hexagram(current_order, tps)

        tps = []
        current_order = order

    tps.append((name, file))
    orders.append(order)

print_one_hexagram(current_order, tps)

print("const ALL_HEXAGRAM_GLYPHS: [[(&str, &[u8]); 3]; 64] = [")
for iorder in range(1, 65):
    s_order = f"{iorder:02}"
    if s_order in orders:
        print(f"    HEXAGRAM_GLYPHS_{s_order},")
    else:
        print(f"    HEXAGRAM_GLYPHS_NULL,")
print("];")
