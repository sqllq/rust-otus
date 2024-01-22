#![allow(dead_code)]

mod devices;
mod providers;

use std::collections;

pub use devices::{SmartSocket, SmartThermometer};
pub use providers::{BorrowingDeviceInfoProvider, DeviceInfoProvider, OwningDeviceInfoProvider};

pub struct SmartHouse {
	name: String,
	rooms: collections::HashMap<String, Vec<String>>,
}

impl SmartHouse {
	#[must_use]
	pub fn new(
		name: String,
		rooms: collections::HashMap<String, Vec<String>>,
	) -> Self {
		Self { name, rooms }
	}

	fn get_rooms(&self) -> Vec<&String> {
		// We really need this sort for tests to work.
		let mut rooms = self.rooms.keys().collect::<Vec<&String>>();
		rooms.sort();
		rooms
	}

	fn devices(
		&self,
		room: &str,
	) -> Option<&Vec<String>> {
		self.rooms.get(room)
	}

	pub fn create_report<T>(
		&self,
		provider: &T,
	) -> String
	where
		T: DeviceInfoProvider,
	{
		let mut states = vec![];
		for (room, devices) in &self.rooms {
			for device in devices {
				states.push(provider.state(room, device));
			}
		}
		states.join("\n")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct MockDeviceInfoProvider;

	fn make_rooms() -> collections::HashMap<String, Vec<String>> {
		let mut rooms = collections::HashMap::new();
		rooms.insert(
			"Living Room".to_string(),
			vec!["TV".to_string(), "Lights".to_string()],
		);
		rooms.insert("Bedroom".to_string(), vec!["Lamp".to_string()]);
		rooms
	}

	impl DeviceInfoProvider for MockDeviceInfoProvider {
		fn state(
			&self,
			_room: &str,
			_device: &str,
		) -> String {
			"Device state".to_string()
		}
	}

	#[test]
	fn test_get_rooms() {
		let rooms = make_rooms();

		let smart_house = SmartHouse::new("My House".to_string(), rooms);
		let bedroom = String::from("Bedroom");
		let living_room = String::from("Living Room");

		let expected_rooms = vec![&bedroom, &living_room];
		let actual_rooms = smart_house.get_rooms();

		assert_eq!(expected_rooms, actual_rooms);
	}

	#[test]
	fn test_devices() {
		let rooms = make_rooms();

		let smart_house = SmartHouse::new("My House".to_string(), rooms);

		let expected_devices = &vec![String::from("TV"), String::from("Lights")];
		let actual_devices = smart_house.devices("Living Room").unwrap();

		assert_eq!(expected_devices, actual_devices);
	}

	#[test]
	fn test_create_report() {
		let rooms = make_rooms();

		let smart_house = SmartHouse::new("My House".to_string(), rooms);
		let provider = MockDeviceInfoProvider;

		let expected_report = "Device state\nDevice state\nDevice state";
		let actual_report = smart_house.create_report(&provider);

		assert_eq!(expected_report, actual_report);
	}
}
