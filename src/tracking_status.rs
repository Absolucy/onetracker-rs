use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TrackingStatus {
	/// Parcel is just created
	PreTransit,
	/// Parcel is in transit
	InTransit,
	/// Parcel is out for delivery
	OutForDelivery,
	/// Parcel is returning to sender
	ReturnToSender,
	/// Parcel is available for pickup
	AvailableForPickup,
	/// Parcel is delivered
	Delivered,
	/// Shipment cancelled
	Cancelled,
	/// An exception has occured
	Exception,
	/// A failure has occured
	Failure,
	/// Tracking error
	Error,
	/// Parcel has not updated for a long time
	Expired,
	/// Parcel will not be tracked
	NotTracked,
	/// Shipment status is unknown
	Unknown,
}
