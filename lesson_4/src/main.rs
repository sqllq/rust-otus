use std::collections;

fn main() {
	let socket1 = lib::SmartSocket {};
	let socket2 = lib::SmartSocket {};
	let thermo = lib::SmartThermometer {};

	let house = lib::SmartHouse::new(
		String::from("house"),
		collections::HashMap::from([(String::from("room"), vec![String::from("device")])]),
	);

	let info_provider_1 = lib::OwningDeviceInfoProvider::new(socket1);
	let report1 = house.create_report(&info_provider_1);

	let info_provider_2 = lib::BorrowingDeviceInfoProvider::new(&socket2, &thermo);
	let report2 = house.create_report(&info_provider_2);

	// Выводим отчёты на экран:
	println!("Report #1: {report1}");
	println!("Report #2: {report2}");
}
