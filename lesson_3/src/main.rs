#![allow(dead_code)]

use std::collections;

struct SmartHouse {
	name: String,
	rooms: collections::HashMap<String, Vec<String>>,
}

impl SmartHouse {
	fn new(
		name: String,
		rooms: collections::HashMap<String, Vec<String>>,
	) -> Self {
		Self { name, rooms }
	}

	fn get_rooms(&self) -> Vec<&String> {
		self.rooms.keys().collect::<Vec<&String>>()
	}

	fn devices(
		&self,
		room: &str,
	) -> Option<&Vec<String>> {
		self.rooms.get(room)
	}

	fn create_report<T>(
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

trait DeviceInfoProvider {
	fn state(
		&self,
		room: &str,
		device: &str,
	) -> String;
}

struct SmartSocket {}
struct SmartThermometer {}

struct OwningDeviceInfoProvider {
	socket: SmartSocket,
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
struct BorrowingDeviceInfoProvider<'a, 'b> {
	socket: &'a SmartSocket,
	thermo: &'b SmartThermometer,
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

fn main() {
	let socket1 = SmartSocket {};
	let socket2 = SmartSocket {};
	let thermo = SmartThermometer {};

	let house = SmartHouse::new(
		String::from("house"),
		collections::HashMap::from([(String::from("room"), vec![String::from("device")])]),
	);

	let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
	let report1 = house.create_report(&info_provider_1);

	let info_provider_2 = BorrowingDeviceInfoProvider {
		socket: &socket2,
		thermo: &thermo,
	};
	let report2 = house.create_report(&info_provider_2);

	// Выводим отчёты на экран:
	println!("Report #1: {report1}");
	println!("Report #2: {report2}");
}
