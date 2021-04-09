#! /usr/bin/env python3
from os import environ, makedirs
import re

target = environ.get('TARGET')
if not target:
  print('::error ::TARGET is required but missing')
  exit(1)

release_tag = environ.get('RELEASE_TAG')
if not release_tag:
  print('::error ::RELEASE_TAG is required but missing')
  exit(1)

checksum = None
word_splitter = re.compile(r'\s+')
for line in open('checksums/sha1sum.txt').readlines():
  line = line.strip()
  if line.endswith(target):
    checksum, _ = word_splitter.split(line)

maintainer = '# Maintainer: Hoàng Văn Khải <hvksmr1996@gmail.com>\n'
readme_url = f'https://raw.githubusercontent.com/KSXGitHub/build-fs-tree/{release_tag}/README.md'
license_url = f'https://raw.githubusercontent.com/KSXGitHub/build-fs-tree/{release_tag}/LICENSE.md'

opening = maintainer + '\n# This file is automatically generated. Do not edit.\n'

print('Generating PKGBUILD for build-fs-tree...')
makedirs('./pkgbuild/build-fs-tree', exist_ok=True)
with open('./pkgbuild/build-fs-tree/PKGBUILD', 'w') as pkgbuild:
  content = opening + '\n'
  content += 'pkgname=build-fs-tree\n'
  content += f'pkgver={release_tag}\n'
  source_url = f'https://github.com/KSXGitHub/build-fs-tree/archive/{release_tag}.tar.gz'
  content += f'source=(build-fs-tree-{release_tag}.tar.gz::{source_url})\n'
  content += 'sha1sums=(SKIP)\n'
  content += open('./template/build-fs-tree/PKGBUILD').read() + '\n'
  pkgbuild.write(content)

print('Generating PKGBUILD for build-fs-tree-bin...')
makedirs('./pkgbuild/build-fs-tree-bin', exist_ok=True)
with open('./pkgbuild/build-fs-tree-bin/PKGBUILD', 'w') as pkgbuild:
  content = opening + '\n'
  content += 'pkgname=build-fs-tree-bin\n'
  content += f'pkgver={release_tag}\n'
  source_url_prefix = f'https://github.com/KSXGitHub/build-fs-tree/releases/download/{release_tag}'
  source_url = f'{source_url_prefix}/build-fs-tree-{target}'
  supported_completions = ['bash', 'fish', 'zsh']
  completion_source = ' '.join(
    f'completion.{release_tag}.{ext}::{source_url_prefix}/completion.{ext}'
    for ext in supported_completions
  )
  content += f'source=(build-fs-tree-{checksum}::{source_url} {completion_source} {readme_url} {license_url})\n'
  content += f'_checksum={checksum}\n'
  completion_checksums = ' '.join('SKIP' for _ in supported_completions)
  content += f'_completion_checksums=({completion_checksums})\n'
  content += open('./template/build-fs-tree-bin/PKGBUILD').read() + '\n'
  pkgbuild.write(content)
