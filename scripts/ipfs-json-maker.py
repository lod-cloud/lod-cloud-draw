import json
from urllib.request import urlopen
#from urllib.parse import urlencode
#from bs4 import BeautifulSoup
#import requests
#import sys, traceback
#from itertools import islice
import codecs


reader = codecs.getreader("utf-8")
data = json.load(reader(urlopen("https://lod-cloud.net/extract/datasets")))

print("# IPFS JSON")
print()

#data = list(islice(data.items(),2))
data = data.items()

newDict = dict()


print(len(data))
counter=0
for (identifier, dataset) in data:
    if "other_download" in dataset:
        #print(identifier)
        for other_download in dataset["other_download"]:
             
             
            if len(other_download['mirror'])>0:
         
                newDict[identifier] = dataset
                
                counter=counter+1
     
print(counter)



with open("ipfs-lod.json","w") as out:
        out.write(json.dumps(newDict, indent=2))