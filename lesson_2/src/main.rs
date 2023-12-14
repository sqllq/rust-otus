trait SmartSocket {
	type Error;
	fn description(&self) -> &str;
	fn power_consumption(&self) -> f64;
	fn turn_on(&self) -> Result<(), Self::Error>;
	fn turn_off(&self) -> Result<(), Self::Error>;
}
trait SmartThermometer {
	fn current_temperature(&self) -> f64;
}

fn main() {}
