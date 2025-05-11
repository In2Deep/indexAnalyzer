
#     • Re-scan your .windsurf/rules/ and .windsurf/workflows/ directories
#     • Re-generate exactly one JSON file in each (all_rules.json and all_workflows.json)
#     • Strip out every backtick (`) so you never get bit by those again
#     • Remove the old rules.json (you’ll have only all_rules.json in .windsurf/rules)

#         python3 scripts/update_windsurf_configs.py



#!/usr/bin/env python3
"""
Regenerate .windsurf/rules/all_rules.json and
            .windsurf/workflows/all_workflows.json
from the current .md files, stripping out all backticks,
and delete the old rules.json so you end up with exactly
one JSON file per directory.
"""

import os
import json
import shutil

ROOT = os.path.abspath(os.path.dirname(__file__) + "/..")
WINDSURF = os.path.join(ROOT, ".windsurf")

def regenerate_rules():
    rules_dir = os.path.join(WINDSURF, "rules")
    out_path = os.path.join(rules_dir, "all_rules.json")
    # get every .md file
    rules = []
    for fn in sorted(os.listdir(rules_dir)):
        if not fn.endswith(".md"):
            continue
        path = os.path.join(rules_dir, fn)
        lines = open(path, "r").read().splitlines()
        # find front–matter
        try:
            i1 = next(i for i,l in enumerate(lines) if l.strip()=="---")
            i2 = next(i for i,l in enumerate(lines[i1+1:], i1+1) if l.strip()=="---")
        except StopIteration:
            continue
        fm = {}
        for l in lines[i1+1:i2]:
            if ":" in l:
                k,v = l.split(":",1)
                fm[k.strip()] = v.strip()
            else:
                parts = l.strip().split(None,1)
                if len(parts)==2:
                    fm[parts[0]] = parts[1]
        body_lines = [l.strip() for l in lines[i2+1:] if l.strip() and l.strip()!="---"]
        # collapse into a single paragraph, strip out any backticks
        paragraph = " ".join(body_lines).replace(chr(96),"")
        rules.append({
            "id": os.path.splitext(fn)[0],
            "trigger": fm.get("trigger",""),
            "body": [paragraph]
        })
    # write JSON
    with open(out_path, "w") as f:
        json.dump({"rules": rules}, f, indent=2)
    # remove old rules.json if present
    old = os.path.join(rules_dir, "rules.json")
    if os.path.exists(old):
        os.remove(old)

def regenerate_workflows():
    wf_dir = os.path.join(WINDSURF, "workflows")
    out_path = os.path.join(wf_dir, "all_workflows.json")
    wfs = []
    for fn in sorted(os.listdir(wf_dir)):
        if not fn.endswith(".md"):
            continue
        path = os.path.join(wf_dir, fn)
        lines = open(path, "r").read().splitlines()
        # find front–matter block
        try:
            i1 = next(i for i,l in enumerate(lines) if l.strip()=="---")
            i2 = next(i for i,l in enumerate(lines[i1+1:], i1+1) if l.strip()=="---")
        except StopIteration:
            continue
        fm = {}
        for l in lines[i1+1:i2]:
            if ":" in l:
                k,v = l.split(":",1)
                fm[k.strip()] = v.strip()
            else:
                parts = l.strip().split(None,1)
                if len(parts)==2:
                    fm[parts[0]] = parts[1]
        # collect the rest as body lines, strip out backticks
        body_raw = lines[i2+1:]
        body = [l.rstrip().replace(chr(96),"") for l in body_raw if l.strip() and l.strip()!="---"]
        # drop any leading/trailing blank rows
        while body and not body[0].strip():
            body.pop(0)
        while body and not body[-1].strip():
            body.pop()
        wfs.append({
            "id": os.path.splitext(fn)[0],
            "description": fm.get("description","").replace(chr(96),""),
            "body": body
        })
    with open(out_path, "w") as f:
        json.dump({"workflows": wfs}, f, indent=2)

if __name__ == "__main__":
    regenerate_rules()
    regenerate_workflows()
    print("✅  Updated .windsurf/rules/all_rules.json and")
    print("   .windsurf/workflows/all_workflows.json")
    print("   (old rules.json removed, backticks stripped)")
