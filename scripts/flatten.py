#!/usr/bin/env python3
"""Flatten one herdr sub-schema into a standalone JSON Schema typify can consume.

herdr's published schema (`herdr api schema --json`) is a container:

    {"protocol": N, "schema_version": N, "schemas": {"<name>": <sub-schema>, ...}}

Each <sub-schema> is itself a complete JSON Schema, but its internal $refs are
written absolutely as `#/schemas/<name>/$defs/X`. This rewrites those to
root-relative `#/$defs/X` (and the self-ref `#/schemas/<name>` -> `#`) so the
sub-schema stands alone. A cross-sub-schema $ref is a hard error: the container
is asserted to be a set of self-contained sub-schemas, and that invariant is
what makes per-sub-schema generation (rather than one clobbered namespace)
correct.

Usage: flatten.py <schema.json> <sub-schema-name>   # prints the root to stdout
"""
import json
import sys

DRAFT = "https://json-schema.org/draft/2020-12/schema"


def rewrite_refs(node, name):
    self_prefix = f"#/schemas/{name}/$defs/"
    self_root = f"#/schemas/{name}"
    if isinstance(node, dict):
        out = {}
        for k, v in node.items():
            if k == "$ref" and isinstance(v, str):
                if v.startswith(self_prefix):
                    out[k] = "#/$defs/" + v[len(self_prefix):]
                elif v == self_root:
                    out[k] = "#"
                elif v.startswith("#/schemas/"):
                    raise SystemExit(
                        f"cross-sub-schema $ref in '{name}': {v!r} — the flatten "
                        "invariant (self-contained sub-schemas) is broken; the "
                        "per-sub-schema generation strategy no longer holds"
                    )
                else:
                    out[k] = v
            else:
                out[k] = rewrite_refs(v, name)
        return out
    if isinstance(node, list):
        return [rewrite_refs(x, name) for x in node]
    return node


def main():
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    schema_path, name = sys.argv[1], sys.argv[2]
    with open(schema_path) as fh:
        container = json.load(fh)
    subs = container["schemas"]
    if name not in subs:
        raise SystemExit(f"unknown sub-schema {name!r}; have {sorted(subs)}")
    root = rewrite_refs(subs[name], name)
    root["$schema"] = DRAFT
    json.dump(root, sys.stdout, indent=1, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
