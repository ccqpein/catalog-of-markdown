# README #

Generate catalog of `README.md` on Github Repo.

## Install ##

`cargo install catalog-of-markdown`

## Usage ##

**Example**

`cargo run -- Example.md`

will generate content below in stdout:

```
- [Example](#example)
  - [Subtitle0](#subtitle0)
    - [sub sub title](#sub-sub-title)
  - [Subtitle 1](#subtitle-1)
  - [Subtitle 2](#subtitle-2)
```

`cargo run -- -d 1 ./Example.md`

```
- [Subtitle0](#subtitle0)
  - [sub sub title](#sub-sub-title)
- [Subtitle 1](#subtitle-1)
- [Subtitle 2](#subtitle-2)
```

`cargo run -- -d 2 ./Example.md`

```
- [sub sub title](#sub-sub-title)
```

`cargo run -- -s "*" ./Example.md`

```
* [Example](#example)
  * [Subtitle0](#subtitle0)
    * [sub sub title](#sub-sub-title)
  * [Subtitle 1](#subtitle-1)
  * [Subtitle 2](#subtitle-2)
```
