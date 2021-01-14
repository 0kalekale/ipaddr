## build 

```
python3 setup.py install --user
```

## build requirements

- rustc
- cargo
- python3-dev or python3-devel

## install from per built binary
```
wget https://github.com/0kalekale/ipaddr/releases/download/0.1.0/ipaddr-0.1.0-py3.8-linux-x86_64.egg
pip3 install ipaddr-0.1.0-py3.8-linux-x86_64.egg
```

## using
```py
import ipaddr
ip = ipaddr.getip()
```