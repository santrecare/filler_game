from setuptools import setup, find_packages

setup(
    name='filler_arena',
    version='1.0',
    description='Filler arena',
    author='Vivien Klaousen',
    author_email='vivien.klaousen@gmail.com',
    packages=find_packages(include=['filler_arena']),
    install_requires=[
        'websockets',
    ],
)
