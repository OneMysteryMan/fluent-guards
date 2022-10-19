use crate::{guards::Guards, Bound};

/// Provides chainable functions for multiple guards
///
/// ## Example
/// ```
/// use fluent_guards::Guard;
/// use fluent_guards::Bound;
///
/// fn set_tv(channel: u32, volume: f32) -> Result<(), String> {
/// 	let channel = Guard::new(channel)
/// 		.is_between(1, 15, Bound::Inclusive, "Invalid channel!")
/// 		.is_not_equal_to(13, "Channel 13 is blocked!") // Block channel 13
/// 		.result()?;
///
/// 	let volume = Guard::new(volume)
/// 		.is_not_equal_to(0.0, "TV does not support mute!")
/// 		.is_greater_or_equal(0.1, "Volume must be more than 10%!")
/// 		.is_less_or_equal(1.0, "Volume cannot be more than 100%!")
/// 		.result()?;
///
/// 	println!("Setting TV to channel {} and volume to {:.0}%", channel, volume * 10.0);
/// 	Ok(())
/// }
///
/// assert_eq!(set_tv(5, 0.5), Ok(()));
/// assert_eq!(set_tv(2, 0.7), Ok(()));
/// assert_eq!(set_tv(0, 0.5), Err(String::from("Invalid channel!")));
/// assert_eq!(set_tv(13, 0.5), Err(String::from("Channel 13 is blocked!")));
/// assert_eq!(set_tv(7, 0.0), Err(String::from("TV does not support mute!")));
/// assert_eq!(set_tv(7, 0.05), Err(String::from("Volume must be more than 10%!")));
/// assert_eq!(set_tv(7, 1.1), Err(String::from("Volume cannot be more than 100%!")));
/// ```
pub struct Guard<T: PartialOrd> {
	value: T,
	error: Option<String>,
}

impl<T: PartialOrd> Guard<T> {
	fn error(
		self,
		error: String,
	) -> Guard<T> {
		Guard {
			value: self.value,
			error: Some(error),
		}
	}

	/// Create a new guard on the provided `value`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let pass = Guard::new(5).is_equal_to(5, "?!").result();
	/// assert_eq!(pass, Ok(5));
	///
	/// let fail = Guard::new(4).is_equal_to(5, "4 != 5").result();
	/// assert_eq!(fail, Err(String::from("4 != 5")));
	/// ```
	pub fn new(value: T) -> Guard<T> {
		Guard {
			value,
			error: Option::None,
		}
	}

	/// Returns the result of the guard chain.
	///
	/// If the guard passed an [`Ok`] with the value is returned,
	/// otherwise an [`Err`] with the error message.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let pass = Guard::new(5).is_equal_to(5, "?!").result();
	/// assert_eq!(pass, Ok(5));
	///
	/// let fail = Guard::new(4).is_equal_to(5, "4 != 5").result();
	/// assert_eq!(fail, Err(String::from("4 != 5")));
	/// ```
	pub fn result(self) -> Result<T, String> {
		match self.error {
			None => Ok(self.value),
			Some(message) => Err(message),
		}
	}

	/// Ensures that `value` and `test_value` have the same value.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let pass = Guard::new(5).is_equal_to(5, "?!").result();
	/// assert_eq!(pass, Ok(5));
	///
	/// let fail = Guard::new(4).is_equal_to(5, "4 != 5").result();
	/// assert_eq!(fail, Err(String::from("4 != 5")));
	/// ```
	pub fn is_equal_to<E: Into<String>>(
		self,
		test_value: T,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_equal_to(&self.value, &test_value, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}

	/// Ensures that `value` and `test_value` are not the same value.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let pass = Guard::new(4).is_not_equal_to(5, "1984").result();
	/// assert_eq!(pass, Ok(4));
	///
	/// let fail = Guard::new(4).is_not_equal_to(4, "4 == 4").result();
	/// assert_eq!(fail, Err(String::from("4 == 4")));
	/// ```
	pub fn is_not_equal_to<E: Into<String>>(
		self,
		test_value: T,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_not_equal_to(&self.value, &test_value, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}

	/// Ensures that `value` is less than `test_value`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let pass = Guard::new(4).is_less_than(5, "?!").result();
	/// assert_eq!(pass, Ok(4));
	///
	/// let fail = Guard::new(5).is_less_than(5, "5 >= 5").result();
	/// assert_eq!(fail, Err(String::from("5 >= 5")));
	/// ```
	pub fn is_less_than<E: Into<String>>(
		self,
		test_value: T,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_less_than(&self.value, &test_value, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}

	/// Ensures that `value` is less than or equal to `test_value`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let pass = Guard::new(4).is_less_or_equal(5, "?!").result();
	/// assert_eq!(pass, Ok(4));
	///
	/// let pass = Guard::new(5).is_less_or_equal(5, "?!").result();
	/// assert_eq!(pass, Ok(5));
	///
	/// let fail = Guard::new(6).is_less_or_equal(5, "6 > 5").result();
	/// assert_eq!(fail, Err(String::from("6 > 5")));
	/// ```
	pub fn is_less_or_equal<E: Into<String>>(
		self,
		test_value: T,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_less_or_equal(&self.value, &test_value, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}

	/// Ensures that `value` is greater than `test_value`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let fail = Guard::new(4).is_greater_than(5, "4 <= 5").result();
	/// assert_eq!(fail, Err(String::from("4 <= 5")));
	///
	/// let fail = Guard::new(5).is_greater_than(5, "5 <= 5").result();
	/// assert_eq!(fail, Err(String::from("5 <= 5")));
	///
	/// let pass = Guard::new(6).is_greater_than(5, "?!").result();
	/// assert_eq!(pass, Ok(6));
	/// ```
	pub fn is_greater_than<E: Into<String>>(
		self,
		test_value: T,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_greater_than(&self.value, &test_value, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}

	/// Ensures that `value` is greater than or equal to `test_value`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	///
	/// let fail = Guard::new(4).is_greater_or_equal(5, "4 < 5").result();
	/// assert_eq!(fail, Err(String::from("4 < 5")));
	///
	/// let pass = Guard::new(5).is_greater_or_equal(5, "?!").result();
	/// assert_eq!(pass, Ok(5));
	///
	/// let pass = Guard::new(6).is_greater_or_equal(5, "?!").result();
	/// assert_eq!(pass, Ok(6));
	/// ```
	pub fn is_greater_or_equal<E: Into<String>>(
		self,
		test_value: T,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_greater_or_equal(&self.value, &test_value, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}

	/// Ensures that `value` is between `lower_bound` and `upper_bound`.
	///
	/// See [Guards] for more examples.
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	/// use fluent_guards::Bound;
	///
	/// let fail = Guard::new(4).is_between(4, 6, Bound::Exclusive, "4 not between 4 and 6").result();
	/// assert_eq!(fail, Err(String::from("4 not between 4 and 6")));
	///
	/// let pass = Guard::new(4).is_between(4, 6, Bound::Inclusive, "?!").result();
	/// assert_eq!(pass, Ok(4));
	/// ```
	pub fn is_between<E: Into<String>>(
		self,
		lower_bound: T,
		upper_bound: T,
		bound_mode: Bound,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_between(&self.value, &lower_bound, &upper_bound, bound_mode, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}

	/// Ensures that `value` is outside `lower_bound` and `upper_bound`.
	///
	/// See [Guards] for more examples.
	/// ## Example
	/// ```
	/// use fluent_guards::Guard;
	/// use fluent_guards::Bound;
	///
	/// let fail = Guard::new(4).is_outside(4, 6, Bound::Exclusive, "?!").result();
	/// assert_eq!(fail, Ok(4));
	///
	/// let pass = Guard::new(4).is_outside(4, 6, Bound::Inclusive, "4 is not outside 4 and 6").result();
	/// assert_eq!(pass, Err(String::from("4 is not outside 4 and 6")));
	/// ```
	pub fn is_outside<E: Into<String>>(
		self,
		lower_bound: T,
		upper_bound: T,
		bound_mode: Bound,
		error_message: E,
	) -> Self {
		if self.error.is_some() {
			return self;
		}

		match Guards::is_outside(&self.value, &lower_bound, &upper_bound, bound_mode, error_message) {
			Ok(_) => self,
			Err(error) => self.error(error),
		}
	}
}
