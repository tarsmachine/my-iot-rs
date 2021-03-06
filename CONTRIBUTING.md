## How to help?

For now the architecture is highly unstable. I change the core quite often. So the main help would be code review and advising on better and idiomatic implementations.

Performance and technical debt items are also actual options.

You may implement services but then try to not depend on the current service interface as much as possible.

## Workflow

- Latest code is always in `master`
- I commit directly to `master` because I don't care – and because I run `make check` locally
- Feature branches get deleted after merging
- No release branches, flat commit history
- Releases get tagged with [annotated tags](https://git-scm.com/book/en/v2/Git-Basics-Tagging)
- `CHANGELOG.md` is really maintained
- [Semantic Versioning](https://semver.org/) is used, well… almost
- Refer to issues in commit messages. Sometimes I personally forget to do that though

## Principles

- Clean code, because it's a learning project
- Stability, it should «just work»
- Easy installation on Raspberry Pi
- Good performance on [Raspberry Pi Zero W](https://www.raspberrypi.org/products/raspberry-pi-zero-w/)
- No plugins, [«batteries included»](https://en.wikipedia.org/wiki/Batteries_Included)
- Little configuration, good defaults, automatically discover peripherals and network devices
- Unit tests where applicable, don't be obsessed with them though
- [Don't repeat yourself](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself)
