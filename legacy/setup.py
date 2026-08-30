import re
from setuptools import setup

version = ''
with open('jigoku/__init__.py') as f:
    version = re.search(
        r'^__version__\s*=\s*[\'"]([^\'"]*)[\'"]', f.read(), re.MULTILINE).group(1)


requirements = []
with open('requirements.txt') as f:
    requirements = f.read().splitlines()


if not version:
    raise RuntimeError('version is not set')

readme = ''
with open('README.md', encoding="utf8") as f:
    readme = f.read()

setup(
    name='jigoku',
    author='sinkaroid',
    author_email='anakmancasan@gmail.com',
    version=version,
    long_description=readme,
    long_description_content_type='text/markdown',
    url='https://github.com/sinkaroid/jigoku',
    project_urls={
        "Discord": "https://discord.gg/8wj4vM5hHM",
        "Funding": "https://github.com/sponsors/sinkaroid",
        "Issue tracker": "https://github.com/sinkaroid/jigoku/issues/new/choose",
        "Documentation": "https://github.com/sinkaroid/jigoku/wiki",
    },
    packages=['jigoku', 'jigoku.constant', 'jigoku.utils', 'jigoku.client'],
    license='MIT',
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Customer Service',
        'License :: OSI Approved :: MIT License',
        "Natural Language :: English",
        "Operating System :: OS Independent",
        "Environment :: Console",
        'Programming Language :: Python :: 3.7',
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Topic :: Artistic Software",
        "Topic :: Games/Entertainment",
        "Topic :: Internet :: WWW/HTTP :: Dynamic Content",
        "Topic :: Scientific/Engineering :: Image Processing",
    ],
    description='Bulk downloader for booru imageboards with evil intentions',
    include_package_data=True,
    entry_points={
        'console_scripts': [
            'jigoku = jigoku:main',
        ]
    },
       keywords=[
        "booru",
        "downloader"
        "bulk downloader",
        "mass downloader",
        "gelbooru",
        "rule34",
        "safebooru",
        "xbooru",
        "tbib",
        "realbooru",
        "hypnohub",
        "danbooru",
        "atfbooru",
        "yandere",
        "konachan",
        "konachan.net",
        "lolibooru",
        "e621",
        "e926",
        "paheal",
    ],
    install_requires=requirements,
)
