# fluent-guards

A function library that provides both single use guard functions and chainable
functions to guard against invalid values for your code.

## Example

_See function documentation for more examples._

### Single use

```rust
use fluent_guards::Guards;

fn get_5_as_string(value: i32) -> String {
	let good_value = match Guards::is_equal_to(value, 5, "Value was not 5!") {
		Ok(val) => val,
		Err(why) => return why,
	};
	good_value.to_string()
}

assert_eq!(get_5_as_string(4), "Value was not 5!");
assert_eq!(get_5_as_string(5), "5");
assert_eq!(get_5_as_string(6), "Value was not 5!");
```

### Chainable

```rust
use fluent_guards::Guard;
use fluent_guards::Bound;

fn set_tv(channel: u32, volume: f32) -> Result<(), String> {
	let channel = Guard::new(channel)
		.is_between(1, 15, Bound::Inclusive, "Invalid channel!")
		.is_not_equal_to(13, "Channel 13 is blocked!") // Block channel 13
		.result()?;

	let volume = Guard::new(volume)
		.is_not_equal_to(0.0, "TV does not support mute!")
		.is_greater_or_equal(0.1, "Volume must be more than 10%!")
		.is_less_or_equal(1.0, "Volume cannot be more than 100%!")
		.result()?;

	println!("Setting TV to channel {} and volume to {:.0}%", channel, volume * 10.0);
	Ok(())
}

assert_eq!(set_tv(5, 0.5), Ok(()));
assert_eq!(set_tv(2, 0.7), Ok(()));
assert_eq!(set_tv(0, 0.5), Err(String::from("Invalid channel!")));
assert_eq!(set_tv(13, 0.5), Err(String::from("Channel 13 is blocked!")));
assert_eq!(set_tv(7, 0.0), Err(String::from("TV does not support mute!")));
assert_eq!(set_tv(7, 0.05), Err(String::from("Volume must be more than 10%!")));
assert_eq!(set_tv(7, 1.1), Err(String::from("Volume cannot be more than 100%!")));
```
