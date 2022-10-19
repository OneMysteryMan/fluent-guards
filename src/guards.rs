use crate::Bound;

/// Provides functions for simple, single use guards
pub struct Guards;

impl Guards {
	/// Ensures that `value` and `test_value` have the same value.
	///
	/// Returns [`Ok`] if the values match, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	///
	/// fn get_5_as_string(value: i32) -> String {
	/// 	let good_value = match Guards::is_equal_to(value, 5, "Value was not 5!") {
	/// 		Ok(val) => val,
	/// 		Err(why) => return why,
	/// 	};
	/// 	good_value.to_string()
	/// }
	///
	/// assert_eq!(get_5_as_string(4), "Value was not 5!");
	/// assert_eq!(get_5_as_string(5), "5");
	/// assert_eq!(get_5_as_string(6), "Value was not 5!");
	/// ```
	pub fn is_equal_to<T: PartialOrd, E: Into<String>>(
		value: T,
		test_value: T,
		error_message: E,
	) -> Result<T, String> {
		if value == test_value {
			Ok(value)
		} else {
			Err(error_message.into())
		}
	}

	/// Ensures that `value` and `test_value` are mot the same value.
	///
	/// Returns [`Ok`] if the values are different, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	///
	/// fn get_not_5_as_string(value: i32) -> String {
	/// 	let good_value = match Guards::is_not_equal_to(value, 5, "Value was 5!") {
	/// 		Ok(val) => val,
	/// 		Err(why) => return why,
	/// 	};
	/// 	good_value.to_string()
	/// }
	///
	/// assert_eq!(get_not_5_as_string(4), "4");
	/// assert_eq!(get_not_5_as_string(5), "Value was 5!");
	/// assert_eq!(get_not_5_as_string(6), "6");
	/// ```
	pub fn is_not_equal_to<T: PartialOrd, E: Into<String>>(
		value: T,
		test_value: T,
		error_message: E,
	) -> Result<T, String> {
		if value != test_value {
			Ok(value)
		} else {
			Err(error_message.into())
		}
	}

	/// Ensures that `value` is less than `test_value`.
	///
	/// Returns [`Ok`] if the value is less, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	///
	/// fn less_than_5_as_string(value: i32) -> String {
	/// 	let good_value = match Guards::is_less_than(value, 5, "Value was 5 or more!") {
	/// 		Ok(val) => val,
	/// 		Err(why) => return why,
	/// 	};
	/// 	good_value.to_string()
	/// }
	///
	/// assert_eq!(less_than_5_as_string(4), "4");
	/// assert_eq!(less_than_5_as_string(5), "Value was 5 or more!");
	/// assert_eq!(less_than_5_as_string(6), "Value was 5 or more!");
	/// ```
	pub fn is_less_than<T: PartialOrd, E: Into<String>>(
		value: T,
		test_value: T,
		error_message: E,
	) -> Result<T, String> {
		if value < test_value {
			Ok(value)
		} else {
			Err(error_message.into())
		}
	}

	/// Ensures that `value` is less than or equal to `test_value`.
	///
	/// Returns [`Ok`] if the value is less or equal, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	///
	/// fn leq_5_as_string(value: i32) -> String {
	/// 	let good_value = match Guards::is_less_or_equal(value, 5, "Value was greater than 5!") {
	/// 		Ok(val) => val,
	/// 		Err(why) => return why,
	/// 	};
	/// 	good_value.to_string()
	/// }
	///
	/// assert_eq!(leq_5_as_string(4), "4");
	/// assert_eq!(leq_5_as_string(5), "5");
	/// assert_eq!(leq_5_as_string(6), "Value was greater than 5!");
	/// ```
	pub fn is_less_or_equal<T: PartialOrd, E: Into<String>>(
		value: T,
		test_value: T,
		error_message: E,
	) -> Result<T, String> {
		if value <= test_value {
			Ok(value)
		} else {
			Err(error_message.into())
		}
	}

	/// Ensures that `value` is greater than `test_value`.
	///
	/// Returns [`Ok`] if the value is more, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	///
	/// fn greater_than_5_as_string(value: i32) -> String {
	/// 	let good_value = match Guards::is_greater_than(value, 5, "Value was 5 or less!") {
	/// 		Ok(val) => val,
	/// 		Err(why) => return why,
	/// 	};
	/// 	good_value.to_string()
	/// }
	///
	/// assert_eq!(greater_than_5_as_string(4), "Value was 5 or less!");
	/// assert_eq!(greater_than_5_as_string(5), "Value was 5 or less!");
	/// assert_eq!(greater_than_5_as_string(6), "6");
	/// ```
	pub fn is_greater_than<T: PartialOrd, E: Into<String>>(
		value: T,
		test_value: T,
		error_message: E,
	) -> Result<T, String> {
		if value > test_value {
			Ok(value)
		} else {
			Err(error_message.into())
		}
	}

	/// Ensures that `value` is greater than or equal to `test_value`.
	///
	/// Returns [`Ok`] if the value is greater or equal, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	///
	/// fn geq_5_as_string(value: i32) -> String {
	/// 	let good_value = match Guards::is_greater_or_equal(value, 5, "Value was less than 5!") {
	/// 		Ok(val) => val,
	/// 		Err(why) => return why,
	/// 	};
	/// 	good_value.to_string()
	/// }
	///
	/// assert_eq!(geq_5_as_string(4), "Value was less than 5!");
	/// assert_eq!(geq_5_as_string(5), "5");
	/// assert_eq!(geq_5_as_string(6), "6");
	/// ```
	pub fn is_greater_or_equal<T: PartialOrd, E: Into<String>>(
		value: T,
		test_value: T,
		error_message: E,
	) -> Result<T, String> {
		if value >= test_value {
			Ok(value)
		} else {
			Err(error_message.into())
		}
	}

	/// Ensures that `value` is between `lower_bound` and `upper_bound` (exclusive).
	///
	/// Returns [`Ok`] if the value is exclusively between, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	/// use fluent_guards::Bound;
	///
	/// fn is_between_4_and_6(value: i32, bound_mode: Bound) -> bool {
	/// 	match Guards::is_between(value, 4, 6, bound_mode, "Value was not between 4 and 6!") {
	/// 		Ok(val) => true,
	/// 		Err(why) => false,
	/// 	}
	/// }
	///
	/// assert_eq!(is_between_4_and_6(3, Bound::Exclusive), false);
	/// assert_eq!(is_between_4_and_6(4, Bound::Exclusive), false);
	/// assert_eq!(is_between_4_and_6(5, Bound::Exclusive), true);
	/// assert_eq!(is_between_4_and_6(6, Bound::Exclusive), false);
	///
	/// assert_eq!(is_between_4_and_6(6, Bound::Inclusive), true);
	/// assert_eq!(is_between_4_and_6(7, Bound::Inclusive), false);
	/// ```
	pub fn is_between<T: PartialOrd, E: Into<String>>(
		value: T,
		lower_bound: T,
		upper_bound: T,
		bound_mode: Bound,
		error_message: E,
	) -> Result<T, String> {
		match bound_mode {
			Bound::Exclusive => {
				if value > lower_bound && value < upper_bound {
					Ok(value)
				} else {
					Err(error_message.into())
				}
			},
			Bound::Inclusive => {
				if value >= lower_bound && value <= upper_bound {
					Ok(value)
				} else {
					Err(error_message.into())
				}
			},
		}
	}

	/// Ensures that `value` is outside `lower_bound` and `upper_bound` (exclusive).
	///
	/// Returns [`Ok`] if the value is exclusively outside, otherwise returns [`Err`] with the given `error_message`.
	///
	/// ## Example
	/// ```
	/// use fluent_guards::Guards;
	/// use fluent_guards::Bound;
	///
	/// fn is_not_between_4_and_6(value: i32, bound_mode: Bound) -> bool {
	/// 	match Guards::is_outside(value, 4, 6, bound_mode, "Value was between 4 and 6!") {
	/// 		Ok(val) => true,
	/// 		Err(why) => false,
	/// 	}
	/// }
	///
	/// assert_eq!(is_not_between_4_and_6(3, Bound::Exclusive), true);
	/// assert_eq!(is_not_between_4_and_6(4, Bound::Exclusive), true);
	/// assert_eq!(is_not_between_4_and_6(5, Bound::Exclusive), false);
	/// assert_eq!(is_not_between_4_and_6(6, Bound::Exclusive), true);
	///
	/// assert_eq!(is_not_between_4_and_6(6, Bound::Inclusive), false);
	/// assert_eq!(is_not_between_4_and_6(7, Bound::Inclusive), true);
	/// ```
	pub fn is_outside<T: PartialOrd, E: Into<String>>(
		value: T,
		lower_bound: T,
		upper_bound: T,
		bound_mode: Bound,
		error_message: E,
	) -> Result<T, String> {
		match bound_mode {
			Bound::Exclusive => {
				if value <= lower_bound || value >= upper_bound {
					Ok(value)
				} else {
					Err(error_message.into())
				}
			},
			Bound::Inclusive => {
				if value < lower_bound || value > upper_bound {
					Ok(value)
				} else {
					Err(error_message.into())
				}
			},
		}
	}
}
