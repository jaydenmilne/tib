#!/usr/bin/env python3

"""
Quick and dirty test runner for tib
"""

import os
import subprocess
import time
import glob
from collections import namedtuple

TIB_EXEC = '..' + os.path.sep + 'tib'
TIB_FLAGS = []

TAG_DELIM = '#!'
ARGS = 'args='
RETV = 'retval='
DESC = 'desc='
NAME = 'name='

TestInfo = namedtuple('test_info', ['retval', 'args', 'desc', 'name'])
TestFailureInfo = namedtuple('test_failure_info', ['info', 'filename', 'reason'])

tests = [y for x in os.walk('.') for y in glob.glob(os.path.join(x[0], '*.tib'))]
tests_output = [y for x in os.walk('.') for y in glob.glob(os.path.join(x[0], '*.tib.out'))]

start = time.time()

def get_test_info(filename):
    file = open(filename)

    retval = 0
    args = ""
    desc = ""
    name = ""

    line = file.readline()

    while line.startswith(TAG_DELIM):
        line = line[len(TAG_DELIM):]
        if line.startswith(RETV):
            retval = int(line[len(RETV):])
        elif line.startswith(ARGS):
            args = line[len(ARGS):].rstrip()
        elif line.startswith(DESC):
            desc = line[len(DESC):]
        elif line.startswith(NAME):
            name = line[len(NAME):]
    
    return TestInfo(retval, args, desc, name)

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
        print(results.stdout)
        failures.append(TestFailureInfo(info, test, "Expected return code {}, got {}".format(info.retval, results.returncode)))
        continue

    if test + '.out' in tests_output:
        output_file = open(test + '.out')
        output = output_file.read().replace('\r','')
        output_file.close()
        std_out = results.stdout.decode('unicode_escape').replace('\r','')

        if std_out != output:
            print("{}\nGot:".format(test))
            print(std_out)
            failures.append(TestFailureInfo(info, test, "Output did not match!"))
            print("\nExpected:")
            print(output)
            continue
    else:
        print("Warning! {} is missing a correct output file!".format(test))

print("{}/{} tests passed in {}s".format(count-len(failures), count, time.time() - start))

for failure in failures:
    print("\t{filename} \n\t\t{reason}".format(filename=failure.filename[2:], reason=failure.reason))

print("\nDone.")    
