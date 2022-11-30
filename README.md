# UPE-lang

![](https://upe.berkeley.edu/images/WhaleBlueHeader.png)

This turing-complete, brainfuck-based esolang was created as a challenge for [Upsilon Pi Epsilon](https://upe.berkeley.edu/)'s Nu chapter.

#### Specification

| Brainfuck | UPE-lang
|-----------|----------
| `-` | `u`
| `+` | `U`
| `<` | `p`
| `>` | `P`
| `[` | `e`
| `]` | `E`

Of course, rather than cells having values 0-255, cells have values `uUpPeE`, in that order. Incrementing `E` wraps back around to `u`.

#### Sample Code

Hello world:

```shell
$ upe 'UPUUUPu'
UPE
```

Move the value of the first cell to the second cell:

```shell
$ upe 'euPUpE' 'pu'
up
```

Sum from one to n, modulo 6:

```shell
$ upe 'ePUpuEPeePUPUppuEPPeuppUPPEpuEpeePepUPuEEppEPepUPuE' 'E'
Puuuuuuu
```
