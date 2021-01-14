from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="ipaddr",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=["ipaddr"],
    rust_extensions=[RustExtension("ipaddr.ipaddr", "Cargo.toml", debug=False)],
    include_package_data=True,
    zip_safe=False,
)