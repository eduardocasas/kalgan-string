# kalgan-string

A collection of functions for string manipulation used by Kalgan Framework.

## Examples
```
use kalgan_string;

assert_eq!(kalgan_string::strip("-Hello World-", '-'), "Hello World");
```
```
use kalgan_string;

assert_eq!(kalgan_string::strip_right("Hello World", 'd'), "Hello Worl");
```
```
use kalgan_string;

assert_eq!(kalgan_string::strip_left("Hello World", 'H'), "ello World");
```
```
use kalgan_string;

assert_eq!(kalgan_string::strip_both("Hello World", 'H', 'd'), "ello Worl");
```
```
use kalgan_string;

assert_eq!(kalgan_string::is_numeric("1.000"), true);
```
## Documentation

For further information please visit:

* [Official Kalgan Site](https://kalgan.eduardocasas.com)
* [API Documentation on docs.rs](https://docs.rs/crate/kalgan-string/latest)


## License

This crate is licensed under either of the following licenses:

* [MIT License](https://choosealicense.com/licenses/mit/)
* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
