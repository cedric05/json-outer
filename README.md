# json-outer

launches process from given arguments and prints stdout/stderr distinctively with timestamped

## example

file: sample.py
```python
#!/usr/bin/env python3
import time
import sys
a = 0
while True:
    a += 1
    time.sleep(1)
    if a % 2 == 0:
        print(f'line {a}', flush=True, file=sys.stdout)
    else:
        print(f'line {a}', flush=True, file=sys.stderr)
    if a % 10 == 0:
        inprovided = input("please give me some input")
        print(f"input provided: {inprovided}")
```

`cargo run python3 sample.py`

### Example stdout
```json
{"log":"line 2","stream":"stdout","time":"2023-05-02T18:00:38.526892071+05:30"}
{"log":"line 2","stream":"stdout","time":"2023-05-02T18:00:38.526892071+05:30"}
```