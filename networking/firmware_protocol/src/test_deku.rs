use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use deku::DekuContainerWrite;
use nalgebra031::{Quaternion, UnitQuaternion};

use crate::{CBPacket, Packet, SBPacket};

#[test]
/// Since we have the use of hrtb in the generics of the `Packet`, this ensures that
/// the behaviour of the lifetimes matches what we would expect.
///
/// This code should fail to compile if we did the lifetimes wrong.
fn test_lifetimes() {
	let a = vec![0, 1];
	let a_slice: &[u8] = &a;
	let static_slice: &'static [u8] = &[2, 3, 4];

	let a_result = Packet::<SBPacket>::deserialize_from(a_slice);
	let static_result = Packet::<SBPacket>::deserialize_from(static_slice);

	drop(a);
	drop(static_slice);

	drop(a_result);
	drop(static_result);
}

#[test]
fn handshake() {
	let mac: [u8; 6] = [121, 34, 164, 250, 231, 204]; // test mac
	let handshake = Packet::new(
		1,
		SBPacket::Handshake {
			board: 2,
			imu: 3,
			mcu_type: 4,
			imu_info: (5, 6, 7),
			build: 8,
			firmware: "test".to_string().into(),
			mac_address: mac,
		},
	);

	let data: Vec<u8> = vec![
		0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0,
		0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 4, 116, 101, 115, 116, 121, 34, 164,
		250, 231, 204,
	];

	assert_eq!(handshake.to_bytes().unwrap(), data);
}
#[test]
fn quat() {
	let packet = Packet::new(1, CBPacket::Heartbeat);
	let data: Vec<u8> = vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1];

	assert_eq!(packet.to_bytes().unwrap(), data);
}
#[test]
fn sensor_info() {
	let sensor_info = Packet::new(
		1,
		SBPacket::SensorInfo {
			sensor_id: 64,
			sensor_status: 3,
			sensor_type: 5,
		},
	);
	let data: Vec<u8> = vec![0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 1, 64, 3, 5];

	assert_eq!(sensor_info.to_bytes().unwrap(), data);
}
#[test]
fn quat_fancy() {
	let quat = UnitQuaternion::new_unchecked(Quaternion::new(1.0f32, 0.0, 0.0, 0.0));
	let rotation = Packet::new(
		1,
		SBPacket::RotationData {
			sensor_id: 64,
			data_type: 1,
			quat: (*quat.quaternion()).into(),
			calibration_info: 0,
		},
	);

	let data: Vec<u8> = vec![
		0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 1, 64, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		63, 128, 0, 0, 0,
	];

	assert_eq!(rotation.to_bytes().unwrap(), data);
}
#[test]
fn test_ping() {
	let data = [0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 3, 4];
	let (seq, data): (_, SBPacket) = Packet::deserialize_from(&data).unwrap().split();
	assert_eq!(data, SBPacket::Ping { id: 16909060 });
	assert_eq!(seq, 1);
}
#[test]
fn test_acceleration() {
	let acc = Packet::new(
		16,
		SBPacket::Acceleration {
			vector: (0.1, 0.5, 0.9),
			sensor_id: Some(32),
		},
	);

	let data: Vec<u8> = vec![
		0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 16, 61, 204, 204, 205, 63, 0, 0, 0, 63, 102,
		102, 102, 32,
	];

	assert_eq!(acc.to_bytes().unwrap(), data);
}
