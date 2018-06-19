# jsg - easiy to use json generator

Writing json in the terminal is such a pain because of the escaping all the charcters. `jsg` mimics how `httpie` generates the json for the request bodies, accepting key values as program arguments. 

## Building

```
$ cargo build
```

## Example Usage


```bash
# simple object
jsg name=jsg num_str=25 num:=25 bool:=true some:=null
'{"name":"jsg","num_str":"25","num":25,"bool":true,"some":null}'

# nested objects
jsg name=test obj:="$(jsg field=value)" arr:="$(jsg --arr 1 string true)"
'{"name":"test","obj":{"field":"value"},"arr":[1,"string",true]}'

# arrays
jsg --arr 1 'string with spaces' string 3
'[1, "string with spaces", "string", 3]'

# arrays - do not evaluate types and accept everything as strings
jsg --arr-str 12 str
'["12", "str"]'

```