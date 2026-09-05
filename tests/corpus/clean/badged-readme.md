# Requests-style README

<!-- Would-be PI026: the badge row every popular README opens with. A badge
     image is a markdown image whose URL carries a query string -- exactly the
     shape PI026 describes -- but the query VALUE is a short word
     (`version=latest`, `branch=main`, `style=flat-square`), not data. A beacon
     that exfiltrates context has to carry the context, so the value is long
     or is a placeholder the model is meant to fill. This file is modelled on
     `psf/requests`'s README, which produced two HIGH findings. -->

[![Documentation](https://readthedocs.org/projects/requests/badge/?version=latest)](https://requests.readthedocs.io)
[![Build](https://github.com/psf/requests/actions/workflows/run-tests.yml/badge.svg?branch=main)](https://github.com/psf/requests/actions)
[![Coverage](https://codecov.io/gh/psf/requests/branch/main/graph/badge.svg?token=abc123)](https://codecov.io/gh/psf/requests)
[![PyPI](https://img.shields.io/pypi/v/requests.svg?style=flat-square&logo=pypi)](https://pypi.org/project/requests/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg?label=Build%20Status)](LICENSE)

**Requests** is a simple, yet elegant, HTTP library.

```python
>>> import requests
>>> r = requests.get('https://api.github.com/user', auth=('user', 'pass'))
>>> r.status_code
200
```

Requests is one of the most downloaded Python packages today, pulling in around
`300M downloads / week` — see the project on the
[Python Package Index](https://pypi.org/project/requests/?tab=files) and the
[JCache reference](https://jcp.org/en/jsr/detail?id=107) for the caching layer.

## Installing

```console
$ python -m pip install requests
```

Requests officially supports Python 3.9+, and runs great on PyPy.

## Supported Features & Best–Practices

- Keep-Alive & Connection Pooling
- International Domains and URLs
- Sessions with Cookie Persistence
- Browser-style TLS/SSL Verification

## API Reference and User Guide available on [Read the Docs](https://requests.readthedocs.io/?page=quickstart)
