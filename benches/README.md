# txdo benchmarks

## Sample file generation strategy

```python
#!/usr/bin/env python3

import random
from datetime import datetime

# where words is supplied by https://packages.ubuntu.com/bionic/wamerican

words = [word.strip() for word in open('words', 'r').readlines()]
# len(words) = 102305

def random_date():
    return datetime(random.randint(1920, 2020), random.randint(1, 12), random.randint(1, 28)).isoformat()[:10]
    
PRIORITIES = list("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    
with open('sample.txt', 'w') as sample_file:
    # generate 1,000 items
    for i in range(1000):
        completed = random.choice([True, False])
        use_priority = random.choice([True, False])
        use_completed_date = completed and random.choice([True, False])
        use_created_date = ((not completed) or (completed and use_completed_date)) and random.choice([True, False])
        
        if completed:
            sample_file.write('x ')
        
        if use_priority:
            sample_file.write('({}) '.format(random.choice(PRIORITIES)))
            
        if use_completed_date:
            sample_file.write('{} '.format(random_date()))
            
        if use_created_date:
            sample_file.write('{} '.format(random_date()))
        
        for i in range(random.choice(list(range(1, 25)))):
            prefix = random.choice(['', '', '', '', '', '+', '@', '@', 'foo:', 'sample:', 'bar:'])
            
            sample_file.write('{}{} '.format(prefix, random.choice(words)))

        sample_file.write("\n")
```
