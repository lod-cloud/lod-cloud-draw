import re
import json

data = json.load(open("lod-data.json"))


def toint(y):
    if isinstance(y, int):
        return y
    else:
        x = y.replace(",","").replace(".","")
        if re.match("^[0-9]+$", x):
            return int(x)
        else:
            return 0

print("Triples")
print(sum(toint(x["triples"]) for x in data.values()))
print("")
print("Links")
print(sum(toint(x["value"])  for y in data.values() for x in y["links"]))
