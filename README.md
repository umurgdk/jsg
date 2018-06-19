# jsg - easiy to use json generator

Writing json in the terminal is such a pain because of the escaping all the charcters. `jsg` mimics how `httpie` generates the json for the request bodies, accepting key values as program arguments. 

## Building

```
$ cargo build
```

## Example Usage

```
$ jsg name=jsg num=25 num_as_str="25" arr="$(jsg --arr 1 'string with spaces' string 3)" obj="$(jsg field=value)" bool=false another=null
```