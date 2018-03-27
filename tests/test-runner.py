"""
Quick and dirty test runner for tib
"""

import os
import subprocess
from collections import namedtuple

TIB_EXEC = '..' + os.path.sep + 'tib'
TIB_FLAGS = []

TAG_DELIM = '#!'
ARGS = 'args='
RETV = 'retval='
DESC = 'desc='
NAME = 'name='

class TestInfo:
    __init__


TestInfo = namedtuple('test_info', ['retval', 'args', 'desc', 'name'])
TestFailureInfo = namedtuple('test_failure_info', ['info', 'filename', 'reason'])

tests = [x for x in os.listdir() if x.endswith('.tib')]
tests_output = [x for x in os.listdir() if x.endswith('.tib.out')]



def get_test_info(filename):
    file = open(filename)

    info = TestInfo(0,'','',filename)
    line = file.readline()

    while line.startswith(TAG_DELIM):
        line = line[len(TAG_DELIM):]
        print(line)
        if line.startswith(RETV):
            info.retval = int(line[len(RETV):])
            print (info.retval)
        elif line.startswith(ARGS):
            info.args = line[len(ARGS):]
        elif line.startswith(DESC):
            info.desc = line[len(DESC):]
        elif line.startswith(NAME):
            info.name = line[len(NAME):]
    
    return info

failures = []
count = 0

for test in tests:
    count = count + 1
    info = get_test_info(test)

    name = test
    if info.name:
        name = info.name

    args = '-d'
    if len(info.args) != 0:
        args = info.args
    
    results = subprocess.run([TIB_EXEC, test, args], stdout=subprocess.PIPE)

    if results.returncode != info.retval:
        failures.append(TestFailureInfo(info, test, "Expected return code {}, got {}".format(info.retval, results.returncode)))
        continue

    if test + '.out' in tests_output:
        output_file = open(test + '.out')
        output = output_file.read()
        output_file.close()

        if results.stdout != output:
            failures.append(TestFailureInfo(info, test, "Output did not match!"))
            continue

print("{}/{} tests passed.".format(count-len(failures), count))

for failure in failures:
    print("{filename} - {reason}".format(testname=failure.info.name, filename=failure.filename, reason=failure.reason))

print("\nDone.")    