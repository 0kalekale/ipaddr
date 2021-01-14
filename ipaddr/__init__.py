from .ipaddr import *
import re
def getip():
    ip = (__getip().split()[2])
    rt = re.sub(r'\"', "",ip)
    return rt