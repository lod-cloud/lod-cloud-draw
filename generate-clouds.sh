#!/bin/bash

cargo run --release -- lod-data.json clouds/cross-domain-lod.svg -n 10 -i 5000 -c 350 --ident=neighbour --settings=clouds/cross-domain-lod.json

cargo run --release -- lod-data.json clouds/geography-lod.svg -n 10 -i 5000 -c 300 --ident=neighbour --settings=clouds/geography-lod.json

cargo run --release -- lod-data.json clouds/government-lod.svg -n 10 -i 5000 -c 400 --ident=neighbour --settings=clouds/government-lod.json

cargo run --release -- lod-data.json clouds/life-sciences-lod.svg -n 10 -i 5000 -c 600 --ident=neighbour --settings=clouds/life-sciences-lod.json

cargo run --release -- lod-data.json clouds/linguistic-lod.svg -n 10 -i 5000 -c 500 --ident=neighbour --settings=clouds/linguistic-lod.json

cargo run --release -- lod-data.json clouds/media-lod.svg -n 10 -i 5000 -c 300 --ident=neighbour --settings=clouds/media-lod.json

cargo run --release -- lod-data.json clouds/publications-lod.svg -n 10 -i 5000 -c 400 --ident=neighbour --settings=clouds/publications-lod.json

cargo run --release -- lod-data.json clouds/social-networking-lod.svg -n 10 -i 5000 -c 400 --ident=neighbour --settings=clouds/social-networking-lod.json

cargo run --release -- lod-data.json clouds/user-generated-lod.svg -n 10 -i 5000 -c 300 --ident=neighbour --settings=clouds/user-generated-lod.json

cargo run --release -- lod-data.json clouds/lod-cloud.svg -n 10 -i 5000 --ident=neighbour --settings=clouds/lod-cloud-settings.json

cd clouds
for f in *.svg ; do convert -density 300 -alpha off $f ${f%.svg}.png ; done
convert -scale 40% lod-cloud.png lod-cloud-sm.jpg
cd -
