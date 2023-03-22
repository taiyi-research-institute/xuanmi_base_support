#!/usr/bin/env python
from flask import Flask
import json

app = Flask("SampleServer")

@app.route("/test", methods=["POST"])
def test():
    resp = dict(
        a="Data in field `a`.",
        b="Data in field `b`.",
    )
    return json.dumps(resp)

if __name__ == "__main__":
    app.run(port=50000)
