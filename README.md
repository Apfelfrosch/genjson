# genjson
A small command line program to generate json objects

## Usage
``genjson -names Tom Larry -ages.tom 28 -ages.larry 26``

yields the result

``{"names":["Tom","Larry"],"ages":{"tom":28,"larry":26}}``