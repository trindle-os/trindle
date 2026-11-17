"""help_names.py: lists help lines that tell people to press or choose a name no on-screen label has."""
import re, sys

MAX_LINES = 50000
VERB = re.compile(r"\b(?:[Pp]ress|[Cc]lick|[Cc]hoose|[Ss]elect|[Oo]pen|[Tt]ap|[Gg]o to)\s+(?:the\s+)?((?:[A-Z][\w'’-]*)(?:\s+(?:[a-z][\w'’-]*|[A-Z][\w'’-]*)){0,4})")
STOP = {"Trindle", "Settings", "Ctrl", "Alt", "Super", "Shift", "Enter", "Esc", "Escape", "Tab", "Space", "Delete", "Backspace", "F1", "F2", "F4", "F11", "Print"}

def norm(s):
    return re.sub(r"[.…:]+$", "", s.strip().replace("’", "'")).lower()

def main(path):
    rows = []
    with open(path, encoding="utf-8", errors="replace") as f:
        for i, line in enumerate(f):
            if i >= MAX_LINES:
                break
            key, _, text = line.rstrip("\n").partition("\t")
            rows.append((key, text))
    # Help often sends people to another app's buttons, so a label anywhere in Trindle counts. A short help line
    # can't vouch for itself: it holds the verb too, so it never equals the name it sends people to.
    labels = {norm(t) for _, t in rows if 0 < len(t.split()) <= 6}
    for key, text in rows:
        if "help" not in key:
            continue
        for m in VERB.finditer(text):
            if m.group(1).split()[0] in STOP:
                continue
            # Read on from the name, past the verb's match, so "press Yes, it matches" finds "Yes, it matches".
            rest = text[m.start(1):].replace("’", "'").lower()
            if any(rest.startswith(l) and not rest[len(l):len(l) + 1].isalnum() for l in labels):
                continue
            print(f"{key}\t{m.group(0)}")

if __name__ == "__main__":
    main(sys.argv[1])
