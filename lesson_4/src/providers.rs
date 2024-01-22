use crate::devices;

pub trait DeviceInfoProvider {
	fn state(
		&self,
		room: &str,
		device: &str,
	) -> String;
}

pub struct OwningDeviceInfoProvider {
	socket: devices::SmartSocket,
}

impl OwningDeviceInfoProvider {
	#[must_use]
	pub fn new(socket: devices::SmartSocket) -> Self {
		Self { socket }
	}
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
	fn state(
		&self,
		room: &str,
		device: &str,
	) -> String {
		format!("owning: room: {room}, device: {device}")
	}
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
	socket: &'a devices::SmartSocket,
	thermo: &'b devices::SmartThermometer,
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
	#[must_use]
	pub fn new(
		socket: &'a devices::SmartSocket,
		thermo: &'b devices::SmartThermometer,
	) -> Self {
		Self { socket, thermo }
	}
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
	fn state(
		&self,
		room: &str,
		device: &str,
	) -> String {
		format!("borrowing: room: {room}, device: {device}")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_owning_state() {
		let owning = OwningDeviceInfoProvider::new(devices::SmartSocket {});
		assert_eq!(
			String::from("owning: room: room, device: device"),
			owning.state("room", "device")
		);
	}

	#[test]
	fn test_borrowing_state() {
		let socket = devices::SmartSocket {};
		let thermo = devices::SmartThermometer {};
		let borrowing = BorrowingDeviceInfoProvider::new(&socket, &thermo);
		assert_eq!(
			String::from("borrowing: room: room, device: device"),
			borrowing.state("room", "device")
		);
	}
}
